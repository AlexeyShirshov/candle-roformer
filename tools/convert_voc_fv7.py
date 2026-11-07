#!/usr/bin/env python3
"""Offline developer converter/verifier for the voc_fv7 checkpoint.

Only the standard library is required for hashing, size checks, key/shape/buffer
validation and manifest emission. ``torch`` is imported lazily and only inside
the restricted load path.
"""

from __future__ import annotations

import argparse
import hashlib
import math
import json
import struct
import sys
from pathlib import Path
from typing import Any, Dict, Iterable, List, Mapping, Optional, Sequence

EXPECTED_SHA256 = "77b9e4b54802d670c02daceb30c61ec825dc54ca6c29c34b03cdd5e9f78382b6"
EXPECTED_SIZE = 489571079

PROFILE: Dict[str, Any] = {
    "dim": 256,
    "depth": 12,
    "stereo": True,
    "num_stems": 1,
    "num_bands": 60,
    "dim_head": 64,
    "heads": 8,
    "flash_attn": True,
    "dim_freqs_in": 1025,
    "sample_rate": 44100,
    "stft_n_fft": 2048,
    "stft_hop_length": 441,
    "stft_win_length": 2048,
    "stft_normalized": False,
    "mask_estimator_depth": 2,
    "use_torch_checkpoint": True,
    "time_transformer_depth": 1,
    "freq_transformer_depth": 1,
    "mlp_expansion_factor": 4,
    "linear_transformer_depth": 0,
    "norm_output": True,
    "skip_connection": False,
    "zero_dc": True,
}


# Explicit, versioned v7 schema. This is the single source of truth for what a
# voc_fv7 checkpoint's state_dict must contain; the normal CLI path validates
# against it mandatorily. Exact parameter names/shapes are populated as the model
# is ported; until then the schema is intentionally strict (empty expected set
# means any key is unexpected) so that validation can never be silently disabled.
V7_SCHEMA_VERSION = "v7-2026-11-07"


def _mel_bin_counts(profile: Mapping[str, Any]) -> List[int]:
    """Compute per-band frequency-bin counts using the upstream slaney rule (f64)."""
    n_fft = int(profile["stft_n_fft"])
    sr = int(profile["sample_rate"])
    n_mels = int(profile["num_bands"])
    fmax = sr / 2.0

    def hz_to_mel(hz: float) -> float:
        f_min = 0.0
        f_sp = 200.0 / 3.0
        min_log_hz = 1000.0
        min_log_mel = (min_log_hz - f_min) / f_sp
        log_step = math.log(6.4) / 27.0
        if hz < min_log_hz:
            return (hz - f_min) / f_sp
        return min_log_mel + math.log(hz / min_log_hz) / log_step

    def mel_to_hz(mel: float) -> float:
        f_min = 0.0
        f_sp = 200.0 / 3.0
        min_log_hz = 1000.0
        min_log_mel = (min_log_hz - f_min) / f_sp
        log_step = math.log(6.4) / 27.0
        if mel < min_log_mel:
            return f_min + f_sp * mel
        return min_log_hz * math.exp(log_step * (mel - min_log_mel))

    mel_min = hz_to_mel(0.0)
    mel_max = hz_to_mel(fmax)
    hz = [mel_to_hz(mel_min + (mel_max - mel_min) * i / (n_mels + 1)) for i in range(n_mels + 2)]

    n_bins = n_fft // 2 + 1
    fft_freqs = [i * sr / n_fft for i in range(n_bins)]

    weights = [[0.0] * n_mels for _ in range(n_bins)]
    for i in range(n_mels):
        f_left = hz[i]
        f_center = hz[i + 1]
        f_right = hz[i + 2]
        denom_left = f_center - f_left
        denom_right = f_right - f_center
        if denom_left == 0:
            denom_left = 1e-12
        if denom_right == 0:
            denom_right = 1e-12
        for b in range(n_bins):
            freq = fft_freqs[b]
            lower = (freq - f_left) / denom_left
            upper = (f_right - freq) / denom_right
            w = lower if lower < upper else upper
            if w < 0:
                w = 0.0
            weights[b][i] = w

    # slaney normalization
    for i in range(n_mels):
        enorm = 2.0 / (hz[i + 2] - hz[i])
        for b in range(n_bins):
            weights[b][i] *= enorm

    # force endpoints (upstream)
    weights[0][0] = 1.0
    weights[n_bins - 1][n_mels - 1] = 1.0

    # membership: > 0
    membership = [[] for _ in range(n_mels)]
    for b in range(n_bins):
        assigned = False
        for i in range(n_mels):
            if weights[b][i] > 0:
                membership[i].append(b)
                assigned = True
        if not assigned:
            raise ConverterError(f"mel filterbank left an orphan frequency bin {b} (upstream assert)")
    # require every band non-empty
    for i in range(n_mels):
        if not membership[i]:
            raise ConverterError(f"mel band {i} is empty")
    # coverage assert
    covered = [False] * n_bins
    for i in range(n_mels):
        for b in membership[i]:
            covered[b] = True
    if not all(covered):
        raise ConverterError("not all frequency bins are covered by mel bands")
    return [len(membership[i]) for i in range(n_mels)]


def _build_v7_parameters(profile: Mapping[str, Any]) -> Dict[str, Any]:
    dim = int(profile["dim"])
    depth = int(profile["depth"])
    num_stems = int(profile["num_stems"])
    num_bands = int(profile["num_bands"])
    dim_head = int(profile["dim_head"])
    heads = int(profile["heads"])
    stereo = bool(profile["stereo"])
    audio_channels = 2 if stereo else 1
    time_depth = int(profile["time_transformer_depth"])
    freq_depth = int(profile["freq_transformer_depth"])
    linear_depth = int(profile["linear_transformer_depth"])
    norm_output = bool(profile["norm_output"])
    mlp_expansion = int(profile["mlp_expansion_factor"])
    mask_depth = int(profile["mask_estimator_depth"])

    counts = _mel_bin_counts(profile)
    if any(c < 1 for c in counts):
        raise ConverterError("mel band count must be >= 1")
    in_dims = [2 * audio_channels * c for c in counts]

    params: Dict[str, Any] = {}

    def add(name: str, shape: Sequence[int]) -> None:
        params[name] = {"shape": [int(x) for x in shape], "dtype": "float32"}

    # band split
    for b in range(num_bands):
        di = in_dims[b]
        add(f"band_split.to_features.{b}.0.gamma", (di,))
        add(f"band_split.to_features.{b}.1.weight", (dim, di))
        add(f"band_split.to_features.{b}.1.bias", (dim,))

    # mask estimators
    hidden = dim * mlp_expansion
    for s in range(num_stems):
        for b in range(num_bands):
            di = in_dims[b]
            out = di * 2
            add(f"mask_estimators.{s}.to_freqs.{b}.0.0.weight", (hidden, dim))
            add(f"mask_estimators.{s}.to_freqs.{b}.0.0.bias", (hidden,))
            add(f"mask_estimators.{s}.to_freqs.{b}.0.2.weight", (hidden, hidden))
            add(f"mask_estimators.{s}.to_freqs.{b}.0.2.bias", (hidden,))
            add(f"mask_estimators.{s}.to_freqs.{b}.0.4.weight", (out, hidden))
            add(f"mask_estimators.{s}.to_freqs.{b}.0.4.bias", (out,))

    dim_inner = heads * dim_head
    rotary_shape = (dim_head // 2,)

    # layer tran_modules layout
    if linear_depth > 0:
        tran_specs = [
            (0, linear_depth, False),   # linear attention (no rotary)
            (1, time_depth, True),      # time transformer
            (2, freq_depth, True),      # freq transformer
        ]
    else:
        tran_specs = [
            (0, time_depth, True),      # time transformer
            (1, freq_depth, True),      # freq transformer
        ]

    for i in range(depth):
        for idx, tdepth, has_rotary in tran_specs:
            for d in range(tdepth):
                attn = f"layers.{i}.{idx}.layers.{d}.0"
                add(f"{attn}.norm.gamma", (dim,))
                add(f"{attn}.to_qkv.weight", (dim_inner * 3, dim))
                add(f"{attn}.to_gates.weight", (heads, dim))
                add(f"{attn}.to_gates.bias", (heads,))
                add(f"{attn}.to_out.0.weight", (dim, dim_inner))
                if has_rotary:
                    add(f"{attn}.rotary_embed.freqs", rotary_shape)
                ff = f"layers.{i}.{idx}.layers.{d}.1"
                add(f"{ff}.net.0.gamma", (dim,))
                add(f"{ff}.net.1.weight", (dim * 4, dim))
                add(f"{ff}.net.1.bias", (dim * 4,))
                add(f"{ff}.net.4.weight", (dim, dim * 4))
                add(f"{ff}.net.4.bias", (dim,))
            if tdepth > 0 and norm_output:
                add(f"layers.{i}.{idx}.norm.gamma", (dim,))

    return params


def build_v7_schema(profile: Mapping[str, Any] = PROFILE) -> Dict[str, Any]:
    params = _build_v7_parameters(profile)
    return {
        "version": V7_SCHEMA_VERSION,
        "profile": dict(profile),
        "parameters": params,
        "buffers": [],
    }


V7_SCHEMA: Dict[str, Any] = build_v7_schema(PROFILE)


class ConverterError(Exception):
    """Raised for every refused or failed conversion step."""


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with Path(path).open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def file_size(path: Path) -> int:
    return Path(path).stat().st_size


def validate_checkpoint_file(path: Path) -> Dict[str, Any]:
    path = Path(path)
    if not path.is_file():
        raise ConverterError(f"checkpoint not found: {path}")
    size = file_size(path)
    if size != EXPECTED_SIZE:
        raise ConverterError(
            f"checkpoint size mismatch: got {size}, expected {EXPECTED_SIZE}"
        )
    digest = sha256_file(path)
    if digest.lower() != EXPECTED_SHA256.lower():
        raise ConverterError(
            f"checkpoint SHA256 mismatch: got {digest}, expected {EXPECTED_SHA256}"
        )
    return {"path": str(path), "size": size, "sha256": digest}


# Backwards/alternate names that tests or callers may use.
check_checkpoint = validate_checkpoint_file
check_sha256 = validate_checkpoint_file
verify_sha256 = validate_checkpoint_file
validate_size = validate_checkpoint_file


def torch_load_restricted(path: Path) -> Any:
    """Load with restricted unpickling only. Never retries with unsafe loading."""

    try:
        import torch
    except Exception as exc:  # pragma: no cover - environment dependent
        raise ConverterError(f"torch is not available: {exc}") from exc

    try:
        return torch.load(str(path), map_location="cpu", weights_only=True)
    except Exception as exc:
        raise ConverterError(
            f"restricted torch.load(weights_only=True, map_location='cpu') failed: {exc}"
        ) from exc


load_checkpoint = torch_load_restricted


def _tensor_shape(value: Any) -> tuple:
    if isinstance(value, dict):
        if "shape" in value:
            return tuple(int(x) for x in value["shape"])
        if "sizes" in value:
            return tuple(int(x) for x in value["sizes"])
        raise ConverterError("tensor-like dict is missing 'shape'")
    shape = getattr(value, "shape", None)
    if shape is not None:
        try:
            return tuple(int(x) for x in shape)
        except TypeError:
            pass
    size_fn = getattr(value, "size", None)
    if callable(size_fn):
        try:
            result = size_fn()
            if isinstance(result, int):
                return (int(result),)
            return tuple(int(x) for x in result)
        except Exception:
            pass
    if isinstance(value, (tuple, list)):
        # Allow a bare shape tuple/list to be validated directly.
        try:
            return tuple(int(x) for x in value)
        except (TypeError, ValueError):
            pass
    raise ConverterError(f"cannot determine tensor shape for value: {value!r}")


def _tensor_dtype(value: Any) -> Optional[str]:
    if isinstance(value, dict):
        dtype = value.get("dtype", None)
    else:
        dtype = getattr(value, "dtype", None)
    if dtype is None:
        return None
    text = str(dtype)
    # Normalize torch dtype reprs such as torch.float32.
    if "." in text:
        text = text.rsplit(".", 1)[-1]
    return text


def _parse_expected_entry(value: Any) -> tuple:
    """Return (shape_tuple_or_None, dtype_or_None) for an expected entry."""
    if isinstance(value, (list, tuple)):
        try:
            return (tuple(int(x) for x in value), None)
        except (TypeError, ValueError):
            return (None, None)
    if isinstance(value, Mapping):
        shape = value.get("shape", value.get("sizes", None))
        dtype = value.get("dtype", None)
        if shape is not None:
            try:
                shape = tuple(int(x) for x in shape)
            except (TypeError, ValueError):
                shape = None
        return (shape, dtype)
    if isinstance(value, str):
        # name-only entry: value may be a dtype hint
        return (None, value)
    return (None, None)


def _resolve_expected(
    expected_keys: Any = None,
    schema: Any = None,
    expected_shapes: Any = None,
    expected_dtypes: Any = None,
) -> tuple:
    """Resolve into (expected_map: dict name->(shape_or_None, dtype_or_None), buffer_set: set)."""
    expected_map: Dict[str, tuple] = {}
    buffer_set: set = set()

    if schema is None:
        schema = None
    if schema is not None and isinstance(schema, Mapping):
        params = schema.get("parameters", schema.get("params", {}))
        if isinstance(params, Mapping):
            for name, info in params.items():
                expected_map[str(name)] = _parse_expected_entry(info)
        bufs = schema.get("buffers", schema.get("expected_buffers", []))
        if isinstance(bufs, (list, tuple, set)):
            for b in bufs:
                if isinstance(b, Mapping):
                    name = b.get("name", b.get("buffer"))
                    if name is not None:
                        buffer_set.add(str(name))
                        expected_map[str(name)] = _parse_expected_entry(b)
                else:
                    buffer_set.add(str(b))
                    expected_map.setdefault(str(b), (None, None))

    if expected_keys is not None:
        if isinstance(expected_keys, Mapping):
            for name, val in expected_keys.items():
                sh, dt = _parse_expected_entry(val)
                # If val is None/empty mapping, treat as name-only
                if isinstance(val, Mapping) and not val:
                    sh, dt = None, None
                expected_map[str(name)] = (sh, dt)
        else:
            for name in expected_keys:
                expected_map.setdefault(str(name), (None, None))

    if isinstance(expected_shapes, Mapping):
        for name, sh in expected_shapes.items():
            prev = expected_map.get(str(name), (None, None))
            try:
                sh_t = tuple(int(x) for x in sh)
            except (TypeError, ValueError):
                sh_t = None
            expected_map[str(name)] = (sh_t, prev[1])

    if isinstance(expected_dtypes, Mapping):
        for name, dt in expected_dtypes.items():
            prev = expected_map.get(str(name), (None, None))
            expected_map[str(name)] = (prev[0], dt)

    return expected_map, buffer_set



def _float32_bits(value: Any) -> bytes:
    """Exact 32-bit pattern of a scalar as little-endian bytes (NaN/-0.0 aware)."""
    try:
        return struct.pack("<f", float(value))
    except (TypeError, ValueError, OverflowError):
        return repr(value).encode("utf-8", "surrogateescape")


def _values_bitwise_equal(a: Any, b: Any) -> bool:
    """Compare two tensor value collections by exact bit patterns, not Python ==."""
    if isinstance(a, (list, tuple)) and isinstance(b, (list, tuple)):
        if len(a) != len(b):
            return False
        return all(_values_bitwise_equal(x, y) for x, y in zip(a, b))
    return _float32_bits(a) == _float32_bits(b)


def validate_state_dict(
    state_dict: Any,
    expected_keys: Any = None,
    schema: Any = None,
    expected_shapes: Any = None,
    expected_dtypes: Any = None,
    buffers: Any = None,
) -> Dict[str, Any]:
    if not isinstance(state_dict, Mapping):
        raise ConverterError(f"checkpoint did not contain a state_dict mapping: {type(state_dict)!r}")

    # Mandatory schema: default to V7_SCHEMA when nothing explicit is supplied.
    if schema is None and expected_keys is None and expected_shapes is None and expected_dtypes is None and buffers is None:
        schema = V7_SCHEMA

    expected_map, buffer_set = _resolve_expected(
        expected_keys=expected_keys,
        schema=schema,
        expected_shapes=expected_shapes,
        expected_dtypes=expected_dtypes,
    )
    if isinstance(buffers, (list, tuple, set)):
        for b in buffers:
            buffer_set.add(str(b))
            expected_map.setdefault(str(b), (None, None))

    if not state_dict:
        raise ConverterError("state_dict is empty")

    # Key-level validation is mandatory: an empty expected set never disables
    # validation silently. If no expected keys are defined, every actual key is
    # treated as unexpected (refuse rather than self-confirm).
    expected_names = set(expected_map.keys())
    actual = set(state_dict.keys())
    missing = sorted(expected_names - actual)
    unexpected = sorted(actual - expected_names)
    if not expected_map and actual:
        unexpected = sorted(actual)
    if missing or unexpected:
        raise ConverterError(
            f"state_dict key mismatch: missing={missing}, unexpected={unexpected}"
        )

    validated_keys: List[str] = []
    for key, value in state_dict.items():
        if not isinstance(key, str) or not key:
            raise ConverterError(f"invalid state_dict key: {key!r}")

        # Route through the single shape resolver / outcome record.
        out = _decode_outcome(key, value)
        code = out["code"]
        shape = out["shape"]
        dtype = out["dtype"]

        if code in ("invalid_shape", "count_mismatch", "shape_data_mismatch", "data_missing"):
            if code == "invalid_shape":
                raise ConverterError(f"invalid shape for key {key!r}: {out['declared_shape'] or out['message']}")
            if code == "count_mismatch":
                raise ConverterError(out["message"])
            if code == "shape_data_mismatch":
                raise ConverterError(f"shape mismatch for key {key!r}: {out['message']}")
            if code == "data_missing":
                raise ConverterError(f"data missing for key {key!r}")

        # dtype handling: schema-level mismatch takes precedence over writer dtype
        dtype_code = code if code in ("missing_dtype", "unsupported_dtype") else None

        # Per-key shape/dtype comparison against expected schema when available.
        if key in expected_map:
            exp_shape, exp_dtype = expected_map[key]
            if exp_shape is not None and shape is not None and tuple(int(x) for x in shape) != tuple(int(x) for x in exp_shape):
                raise ConverterError(
                    f"shape mismatch for key {key!r}: expected {list(exp_shape)}, got {list(shape)}"
                )
            if exp_dtype is not None:
                def _norm_d(x):
                    x = str(x)
                    if "." in x:
                        x = x.rsplit(".", 1)[-1]
                    return x
                if dtype is None or dtype_code == "missing_dtype":
                    raise ConverterError(
                        f"dtype missing for key {key!r}: expected {exp_dtype!r}, got None"
                    )
                if _norm_d(dtype) != _norm_d(exp_dtype):
                    raise ConverterError(
                        f"dtype mismatch for key {key!r}: expected {exp_dtype!r}, got {dtype!r}"
                    )
        else:
            # No expected dtype: enforce writer dtype via outcome.
            if dtype_code == "missing_dtype":
                raise ConverterError(f"missing dtype for key {key!r}: expected float32")
            if dtype_code == "unsupported_dtype":
                raise ConverterError(f"unsupported dtype for key {key!r}: {dtype!r}")

        # If outcome reported a dtype code and no schema comparison caught it, raise.
        if dtype_code == "missing_dtype" and key not in expected_map:
            raise ConverterError(f"missing dtype for key {key!r}: expected float32")
        if dtype_code == "unsupported_dtype" and key not in expected_map:
            raise ConverterError(f"unsupported dtype for key {key!r}: {dtype!r}")

        # Per-key shape/dtype comparison against expected schema when available.
        if key in expected_map:
            exp_shape, exp_dtype = expected_map[key]
            if exp_shape is not None and tuple(int(x) for x in shape) != tuple(int(x) for x in exp_shape):
                raise ConverterError(
                    f"shape mismatch for key {key!r}: expected {list(exp_shape)}, got {list(shape)}"
                )
            if exp_dtype is not None:
                def _norm_d(x):
                    x = str(x)
                    if "." in x:
                        x = x.rsplit(".", 1)[-1]
                    return x
                if dtype is None:
                    raise ConverterError(
                        f"dtype missing for key {key!r}: expected {exp_dtype!r}, got None"
                    )
                if _norm_d(dtype) != _norm_d(exp_dtype):
                    raise ConverterError(
                        f"dtype mismatch for key {key!r}: expected {exp_dtype!r}, got {dtype!r}"
                    )

        validated_keys.append(key)

    # Rotary check: every rotary_embed.freqs must share one shape, and (when data
    # is available) be bitwise identical across attentions.
    rotary_keys = [k for k in state_dict if str(k).endswith("rotary_embed.freqs")]
    if rotary_keys:
        rotary_shapes_set = set()
        for k in rotary_keys:
            o = _decode_outcome(k, state_dict[k])
            if o["code"] is not None or o["shape"] is None:
                raise ConverterError(f"invalid shape for key {k!r}: {o['message']}")
            rotary_shapes_set.add(tuple(int(x) for x in o["shape"]))
        if len(rotary_shapes_set) != 1:
            raise ConverterError(
                "rotary freqs shape mismatch: "
                + ", ".join(f"{k}={list(_decode_outcome(k, state_dict[k])['shape'])}" for k in rotary_keys)
            )
        rotary_data = []
        ok = True
        for k in rotary_keys:
            try:
                o = _decode_outcome(k, state_dict[k])
                if o["code"] is not None or o["values"] is None:
                    raise ConverterError(o["message"] or "data missing")
                rotary_data.append(o["values"])
            except ConverterError:
                ok = False
                break
        if not ok:
            raise ConverterError("rotary data missing for one or more rotary_embed.freqs")
        if rotary_data:
            first = rotary_data[0]
            for vals in rotary_data[1:]:
                if not _values_bitwise_equal(vals, first):
                    raise ConverterError("rotary freqs are not bitwise identical across attentions")

    return {"num_keys": len(validated_keys), "keys": sorted(validated_keys)}


validate_state_dict_keys = validate_state_dict
validate_keys = validate_state_dict


def validate_profile(profile: Mapping[str, Any] = PROFILE) -> bool:
    if not isinstance(profile, Mapping):
        raise ConverterError("profile must be a mapping")
    for key in PROFILE:
        if key not in profile:
            raise ConverterError(f"profile is missing key: {key}")
    if int(profile["stft_n_fft"]) % 2 != 0:
        raise ConverterError("stft_n_fft must be even")
    if int(profile["stft_hop_length"]) <= 0:
        raise ConverterError("stft_hop_length must be > 0")
    if int(profile["stft_win_length"]) <= 0 or int(profile["stft_win_length"]) > int(profile["stft_n_fft"]):
        raise ConverterError("stft_win_length must be in 1..=stft_n_fft")
    if int(profile["dim_freqs_in"]) != int(profile["stft_n_fft"]) // 2 + 1:
        raise ConverterError("dim_freqs_in must equal stft_n_fft/2 + 1")
    return True


def _flatten_data(data: Any) -> list:
    """Canonically flatten a (possibly nested) data container into a flat list.

    This is the single, shared flattening rule used by the decoder; no other
    code path may re-implement its own flattening.
    """
    if isinstance(data, (list, tuple)):
        out: list = []
        for item in data:
            out.extend(_flatten_data(item))
        return out
    return [data]


def _infer_extents(data: Any) -> Optional[tuple]:
    """Infer the regular nested extent tuple of ``data``.

    Returns ``None`` when the structure is ragged or inconsistent. A flat
    sequence of scalars yields a 1-tuple ``(len(data),)``.
    """
    if isinstance(data, (list, tuple)):
        if len(data) == 0:
            return (0,)
        if all(not isinstance(x, (list, tuple)) for x in data):
            return (len(data),)
        child_extents = [_infer_extents(x) for x in data]
        if any(e is None for e in child_extents):
            return None
        if len(set(child_extents)) != 1:
            return None
        return (len(data),) + child_extents[0]
    return ()


def _is_flat_sequence(data: Any) -> bool:
    if isinstance(data, (list, tuple)):
        return all(not isinstance(x, (list, tuple)) for x in data)
    return True


def _normalize_dtype(src_dtype: Any) -> Optional[str]:
    if src_dtype is None:
        return None
    t = str(src_dtype)
    if "." in t:
        t = t.rsplit(".", 1)[-1]
    return t


def _declared_shape_raw(value: Any) -> tuple:
    """Return (raw_container, conflict_bool, has_declared) for a value.

    Covers ``shape``, ``sizes``, or neither. If both are present and disagree,
    ``conflict`` is True.
    """
    if isinstance(value, dict):
        has_shape = "shape" in value and value.get("shape") is not None
        has_sizes = "sizes" in value and value.get("sizes") is not None
        if has_shape and has_sizes:
            try:
                if list(value["shape"]) != list(value["sizes"]):
                    return (None, True, True)
            except Exception:
                return (None, True, True)
            return (value["shape"], False, True)
        if has_shape:
            return (value["shape"], False, True)
        if has_sizes:
            return (value["sizes"], False, True)
        return (None, False, False)
    # torch-like
    shape = getattr(value, "shape", None)
    if shape is not None:
        try:
            return (list(shape), False, True)
        except TypeError:
            return (shape, False, True)
    size_fn = getattr(value, "size", None)
    if callable(size_fn):
        try:
            result = size_fn()
            if isinstance(result, int):
                return ([result], False, True)
            return (list(result), False, True)
        except Exception:
            pass
    return (None, False, False)


def _raw_shape(value: Any) -> Optional[Any]:
    """Backwards-compatible raw shape accessor (declared container)."""
    raw, _conflict, has = _declared_shape_raw(value)
    if not has:
        if isinstance(value, (list, tuple)):
            return value
        return None
    return raw


def _shape_entries_valid(shape_container: Any) -> bool:
    if shape_container is None:
        return False
    try:
        for dim in shape_container:
            if isinstance(dim, bool) or (not isinstance(dim, int)):
                return False
            if int(dim) < 0:
                return False
    except TypeError:
        return False
    return True


def _extract_data_and_shape(value: Any) -> tuple:
    """Backwards-compatible (shape, flat) extractor."""
    out = _decode_outcome("__extract__", value)
    if out["code"] is not None or out["shape"] is None or out["values"] is None:
        # Fall back to old behaviour for non-decodable values.
        if isinstance(value, dict) and "data" in value:
            data = value["data"]
            flat = _flatten_data(data)
            if "shape" in value and value.get("shape") is not None:
                shape = tuple(int(x) for x in value["shape"])
            elif "sizes" in value and value.get("sizes") is not None:
                shape = tuple(int(x) for x in value["sizes"])
            else:
                shape = (len(flat),)
            return shape, flat
        if hasattr(value, "tolist") and not isinstance(value, (list, tuple, dict)):
            obj = value
            if hasattr(obj, "detach"):
                obj = obj.detach()
            if hasattr(obj, "cpu"):
                obj = obj.cpu()
            if hasattr(obj, "flatten"):
                obj = obj.flatten()
            data = obj.tolist() if hasattr(obj, "tolist") else None
            shape = _tensor_shape(value)
            flat = data if not isinstance(data, list) else _flatten_data(data)
            return shape, (list(flat) if not isinstance(flat, list) else flat)
        if isinstance(value, (list, tuple)):
            if value and all(isinstance(x, (int, float, bool)) and not isinstance(x, (list, tuple)) for x in value):
                return (len(value),), list(value)
            flat = _flatten_data(value)
            return (len(flat),), flat
        raise ConverterError(f"cannot extract tensor data for value: {value!r}")
    return out["shape"], out["values"]


def _decode_outcome(key: Any, value: Any) -> Dict[str, Any]:
    """Non-raising canonical decode producing an outcome record.

    Fields: shape, values, dtype, code, message, declared_shape,
    expected_numel, actual_count, inferred_extents.
    ``code`` is None on success.
    """
    rec: Dict[str, Any] = {
        "shape": None,
        "values": None,
        "dtype": None,
        "code": None,
        "message": None,
        "declared_shape": None,
        "expected_numel": None,
        "actual_count": None,
        "inferred_extents": None,
    }

    src_dtype = _tensor_dtype(value)
    rec["dtype"] = src_dtype
    norm_dtype = _normalize_dtype(src_dtype)

    # Declared shape info.
    declared_raw, conflict, has_declared = _declared_shape_raw(value)
    if conflict:
        rec["code"] = "invalid_shape"
        rec["message"] = f"invalid shape for key {key!r}: shape and sizes disagree"
        return rec

    # Data extraction.
    is_torch = hasattr(value, "tolist") and not isinstance(value, (list, tuple, dict))
    if isinstance(value, dict):
        if "data" in value and value.get("data") is not None:
            data_obj = value["data"]
            inferred = _infer_extents(data_obj)
            flat = _flatten_data(data_obj)
            is_flat = _is_flat_sequence(data_obj)
        else:
            data_obj = None
            inferred = None
            flat = None
            is_flat = True
    elif is_torch:
        obj = value
        if hasattr(obj, "detach"):
            obj = obj.detach()
        if hasattr(obj, "cpu"):
            obj = obj.cpu()
        if hasattr(obj, "flatten"):
            obj = obj.flatten()
        data_list = obj.tolist() if hasattr(obj, "tolist") else None
        data_obj = data_list
        inferred = None
        flat = data_list if not isinstance(data_list, list) else _flatten_data(data_list)
        if not isinstance(flat, list):
            flat = [flat]
        is_flat = True
    elif isinstance(value, (list, tuple)):
        data_obj = value
        inferred = _infer_extents(value)
        flat = _flatten_data(value)
        is_flat = _is_flat_sequence(value)
    else:
        data_obj = None
        inferred = None
        flat = None
        is_flat = True

    rec["inferred_extents"] = inferred

    # Validate declared shape entries.
    if has_declared:
        if not _shape_entries_valid(declared_raw):
            rec["code"] = "invalid_shape"
            rec["message"] = f"invalid shape for key {key!r}: {declared_raw!r}"
            return rec
        try:
            declared_shape = tuple(int(x) for x in declared_raw)
        except Exception:
            rec["code"] = "invalid_shape"
            rec["message"] = f"invalid shape for key {key!r}: {declared_raw!r}"
            return rec
        rec["declared_shape"] = declared_shape
        rec["shape"] = declared_shape

        if data_obj is None and not is_torch:
            # no data key present in dict
            if isinstance(value, dict) and "data" not in value:
                rec["code"] = "data_missing"
                rec["message"] = f"data missing for key {key!r}"
                rec["shape"] = tuple(int(x) for x in declared_shape)
                return rec
        if flat is None:
            rec["code"] = "data_missing"
            rec["message"] = f"data missing for key {key!r}"
            rec["shape"] = tuple(int(x) for x in declared_shape)
            return rec

        if is_flat:
            expected_numel = 1
            for dim in declared_shape:
                expected_numel *= int(dim) if int(dim) >= 0 else 1
            if len(declared_shape) == 0:
                expected_numel = 1
            rec["expected_numel"] = expected_numel
            rec["actual_count"] = len(flat)
            rec["values"] = list(flat)
            if expected_numel != len(flat):
                rec["code"] = "count_mismatch"
                rec["message"] = (
                    f"numel mismatch for key {key!r}: shape {list(declared_shape)} "
                    f"implies {expected_numel}, got {len(flat)}"
                )
                return rec
        else:
            # nested regular data must have inferred extents == declared shape
            if inferred is None:
                rec["code"] = "shape_data_mismatch"
                rec["message"] = (
                    f"shape/data mismatch for key {key!r}: ragged or irregular data"
                )
                rec["values"] = list(flat) if flat is not None else None
                return rec
            if tuple(int(x) for x in inferred) != declared_shape:
                rec["code"] = "shape_data_mismatch"
                rec["message"] = (
                    f"shape/data mismatch for key {key!r}: data extents {list(inferred)} "
                    f"!= declared shape {list(declared_shape)}"
                )
                rec["values"] = list(flat) if flat is not None else None
                return rec
            expected_numel = 1
            for dim in declared_shape:
                expected_numel *= int(dim) if int(dim) >= 0 else 1
            if len(declared_shape) == 0:
                expected_numel = 1
            rec["expected_numel"] = expected_numel
            rec["actual_count"] = len(flat)

        shape = declared_shape
        rec['shape'] = tuple(int(x) for x in shape) if shape is not None else None
    else:
        # No declared shape: len(flat) is the shape.
        if flat is None:
            rec["code"] = "data_missing"
            rec["message"] = f"data missing for key {key!r}"
            rec["shape"] = tuple(int(x) for x in declared_shape)
            return rec
        shape = (len(flat),)
        rec['shape'] = tuple(int(x) for x in shape)
        rec["expected_numel"] = len(flat)
        rec["actual_count"] = len(flat)

    # Record decoded shape/values even when dtype fails, so consumers can
    # derive fields from the outcome.
    rec["shape"] = tuple(int(x) for x in shape) if shape is not None else None
    rec["values"] = list(flat) if flat is not None else []

    # Dtype validation.
    if norm_dtype is None:
        rec["code"] = "missing_dtype"
        rec["message"] = f"missing dtype for key {key!r}: expected float32"
        rec["values"] = None
        return rec
    if norm_dtype not in {"float32", "f32", "float"}:
        rec["code"] = "unsupported_dtype"
        rec["message"] = (
            f"unsupported dtype for writing key {key!r}: {src_dtype!r} (writer emits F32 only)"
        )
        rec["values"] = None
        return rec

    rec["values"] = list(flat) if flat is not None else []
    return rec


def decode_and_validate_tensor(key: Any, value: Any) -> tuple:
    """Canonical decode + validate for one tensor.

    Single source of truth used by both ``write_safetensors`` and
    ``schema_report`` (via ``_decode_outcome``).
    """
    out = _decode_outcome(key, value)
    if out["code"] is not None:
        raise ConverterError(out["message"])
    return out["shape"], out["values"]




def validate_tensor(key: Any, value: Any) -> None:
    """Single shared shape/dtype/count validation for one tensor.

    Delegates to ``decode_and_validate_tensor`` so there is exactly one
    decode-and-validate implementation.
    """
    decode_and_validate_tensor(key, value)
    return None


# Aliases for callers/tests that may use a different name.
validate_tensor_value = validate_tensor
validate_tensor_shape_dtype_count = validate_tensor
check_tensor = validate_tensor
decode_and_validate = decode_and_validate_tensor
canonical_decode_and_validate = decode_and_validate_tensor


def write_safetensors(path: Path, state_dict: Mapping[str, Any]) -> None:
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    header: Dict[str, Any] = {}
    data = bytearray()

    for key in sorted(state_dict):
        # Single canonical decode + validate (same as schema_report).
        shape, values = decode_and_validate_tensor(key, state_dict[key])

        # Pack data buffer contiguously: no per-tensor padding.
        start = len(data)
        try:
            packed = struct.pack(f"<{len(values)}f", *[float(x) for x in values])
        except struct.error as exc:
            raise ConverterError(f"failed to pack tensor {key!r}: {exc}") from exc
        data.extend(packed)
        end = len(data)
        header[key] = {
            "dtype": "F32",
            "shape": [int(x) for x in shape],
            "data_offsets": [start, end],
        }

    header_json = json.dumps(header, separators=(",", ":")).encode("utf-8")
    while len(header_json) % 8 != 0:
        header_json += b" "

    with path.open("wb") as f:
        f.write(struct.pack("<Q", len(header_json)))
        f.write(header_json)
        f.write(data)


def schema_report(state_dict: Any, schema: Any = V7_SCHEMA) -> Dict[str, Any]:
    """Read-only diff of a state_dict against the derived v7 schema (no writes).

    Consumes the canonical decoder (``_decode_outcome``) for every key: each
    outcome record is computed once per key and cached; every report field is
    derived from it. No independent product/shape comparison is performed here.
    """
    if not isinstance(state_dict, Mapping):
        raise ConverterError(f"state_dict must be a mapping, got {type(state_dict)!r}")
    expected_map, buffer_set = _resolve_expected(schema=schema)
    expected_names = set(expected_map.keys())
    actual_names = set(state_dict.keys()) if isinstance(state_dict, Mapping) else set()
    missing = sorted(expected_names - actual_names)
    unexpected = sorted(actual_names - expected_names)

    # Compute one outcome per key, cached.
    outcomes: Dict[str, Any] = {}
    for name in sorted(actual_names):
        outcomes[name] = _decode_outcome(name, state_dict[name])

    shape_mismatches = []
    dtype_mismatches = []
    for name in sorted(expected_names & actual_names):
        exp_shape, exp_dtype = expected_map[name]
        out = outcomes[name]
        act_shape = out["shape"]
        act_dtype = out["dtype"]
        if exp_shape is not None:
            if act_shape is None or tuple(int(x) for x in act_shape) != tuple(int(x) for x in exp_shape):
                shape_mismatches.append({
                    "key": name,
                    "expected": list(exp_shape),
                    "actual": (list(act_shape) if act_shape is not None else None),
                })
        if exp_dtype is not None and act_dtype is not None:
            def _norm(x):
                t = str(x)
                if "." in t:
                    t = t.rsplit(".", 1)[-1]
                return t
            if _norm(act_dtype) != _norm(exp_dtype):
                dtype_mismatches.append({"key": name, "expected": exp_dtype, "actual": act_dtype})

    # Per-tensor dtype presence.
    dtype_missing = []
    for name in sorted(actual_names):
        out = outcomes[name]
        if out["code"] == "missing_dtype" or out["dtype"] is None:
            dtype_missing.append(name)

    # Tensor data presence.
    data_missing = []
    for name in sorted(actual_names):
        out = outcomes[name]
        if out["code"] == "data_missing" or out["values"] is None:
            data_missing.append(name)

    # Shared per-tensor validation derived from outcomes.
    invalid_shape_errors = []
    count_mismatches = []
    shape_data_mismatches = []
    writer_dtype_invalid = []
    for name in sorted(actual_names):
        out = outcomes[name]
        code = out["code"]
        if code == "invalid_shape":
            invalid_shape_errors.append(name)
        if code == "count_mismatch":
            count_mismatches.append({
                "key": name,
                "expected": out["expected_numel"],
                "actual": out["actual_count"],
            })
        if code == "shape_data_mismatch":
            shape_data_mismatches.append(name)
        if code in ("missing_dtype", "unsupported_dtype"):
            writer_dtype_invalid.append(name)

    rotary_keys = [k for k in state_dict if str(k).endswith("rotary_embed.freqs")]
    rotary_shapes = sorted({tuple(int(x) for x in outcomes[k]["shape"]) for k in rotary_keys if outcomes[k]["shape"] is not None})
    expected_rotary_count = sum(1 for k in expected_map if str(k).endswith("rotary_embed.freqs"))
    rotary_data_present = True
    rotary_vals = []
    for k in rotary_keys:
        out = outcomes[k]
        if out["code"] is not None or out["values"] is None:
            rotary_vals = []
            rotary_data_present = False
            break
        rotary_vals.append(out["values"])
    if rotary_data_present and rotary_vals and len(rotary_vals) == len(rotary_keys):
        rotary_data_identical = all(_values_bitwise_equal(v, rotary_vals[0]) for v in rotary_vals[1:])
    else:
        rotary_data_identical = (len(rotary_keys) == 0)
    if len(rotary_keys) == 0:
        rotary_data_identical = None

    # bitwise pack check using outcome values.
    bitwise_pack_ok = True
    for name in sorted(actual_names):
        out = outcomes[name]
        if out["values"] is None:
            bitwise_pack_ok = False
            break
        try:
            for v in out["values"]:
                struct.pack("<f", float(v))
        except Exception:
            bitwise_pack_ok = False
            break

    exact_key_set_ok = (len(missing) == 0 and len(unexpected) == 0 and len(expected_names) == len(actual_names))
    invalid_shape_ok = (len(invalid_shape_errors) == 0)
    shape_equal_ok = (len(shape_mismatches) == 0 and len(shape_data_mismatches) == 0)
    count_mismatch_ok = (len(count_mismatches) == 0)
    dtype_present_ok = (len(dtype_missing) == 0)
    dtype_equal_ok = (len(dtype_mismatches) == 0)
    writer_dtype_ok = (len(writer_dtype_invalid) == 0)
    tensor_data_present_ok = (len(data_missing) == 0)
    shape_data_mismatch_ok = (len(shape_data_mismatches) == 0)
    rotary_count_ok = (len(rotary_keys) == expected_rotary_count)
    rotary_shape_ok = (len(rotary_shapes) <= 1) if rotary_keys else True
    rotary_data_present_ok = rotary_data_present if rotary_keys else True
    rotary_bitwise_ok = (rotary_data_identical is True) if (rotary_keys and rotary_data_present) else (len(rotary_keys) == 0)

    required_checks = [
        ("exact_key_set", exact_key_set_ok),
        ("invalid_shape", invalid_shape_ok),
        ("shape_mismatch", shape_equal_ok),
        ("shape_data_mismatch", shape_data_mismatch_ok),
        ("count_mismatch", count_mismatch_ok),
        ("missing_dtype", dtype_present_ok),
        ("dtype_mismatch", dtype_equal_ok),
        ("writer_dtype", writer_dtype_ok),
        ("data_missing", tensor_data_present_ok),
        ("bitwise_mismatch", bitwise_pack_ok),
        ("rotary_absent", rotary_count_ok),
        ("rotary_shape_mismatch", rotary_shape_ok),
        ("rotary_data_missing", rotary_data_present_ok),
        ("rotary_bitwise_mismatch", rotary_bitwise_ok),
    ]

    reason_code = None
    for name, ok in required_checks:
        if ok is not True:
            reason_code = name
            break
    reason = reason_code
    if reason_code == "count_mismatch" and count_mismatches:
        cm = count_mismatches[0]
        reason = f"count_mismatch: {cm['key']} (expected {cm['expected']}, got {cm['actual']})"
    if reason_code == "invalid_shape" and invalid_shape_errors:
        reason = f"invalid_shape: {invalid_shape_errors[0]}"
    if reason_code == "shape_data_mismatch" and shape_data_mismatches:
        reason = f"shape_data_mismatch: {shape_data_mismatches[0]}"

    diff_count = len(missing) + len(unexpected) + len(shape_mismatches) + len(dtype_mismatches) + len(shape_data_mismatches)

    match = all(ok is True for (_n, ok) in required_checks)

    return {
        "schema_version": schema.get("version") if isinstance(schema, Mapping) else None,
        "expected_count": len(expected_names),
        "actual_count": len(actual_names),
        "missing": missing,
        "unexpected": unexpected,
        "shape_mismatches": shape_mismatches,
        "dtype_mismatches": dtype_mismatches,
        "dtype_missing": dtype_missing,
        "data_missing": data_missing,
        "invalid_shape": invalid_shape_errors,
        "count_mismatches": count_mismatches,
        "shape_data_mismatches": shape_data_mismatches,
        "writer_dtype_invalid": writer_dtype_invalid,
        "rotary": {
            "count": len(rotary_keys),
            "expected_count": expected_rotary_count,
            "shapes": [list(s) for s in rotary_shapes],
            "all_identical_shape": len(rotary_shapes) <= 1,
            "bitwise_identical": rotary_data_identical,
            "data_present": rotary_data_present,
        },
        "reason": reason,
        "reason_code": reason_code,
        "code": reason_code,
        "match": match,
        "diff_count": diff_count,
    }


def build_manifest(
    checkpoint: Path,
    output: Path,
    profile: Mapping[str, Any],
    state_dict: Mapping[str, Any],
    source_sha: str,
    source_size: int,
    expected_keys: Any = None,
    schema: Any = None,
) -> Dict[str, Any]:
    output_sha = sha256_file(output) if Path(output).is_file() else None
    if schema is None and expected_keys is None:
        schema = V7_SCHEMA
    expected_map, _ = _resolve_expected(expected_keys=expected_keys, schema=schema)
    if expected_map:
        expected = set(expected_map.keys())
    elif schema is not None and isinstance(schema, Mapping):
        expected = set()
    else:
        expected = set()
    actual = set(state_dict.keys())
    matched = sorted(expected & actual)
    missing = sorted(expected - actual)
    unexpected = sorted(actual - expected)
    schema_version = None
    if isinstance(schema, Mapping):
        schema_version = schema.get("version")
    parameters = []
    for key in sorted(state_dict):
        parameters.append(
            {
                "name": key,
                "dtype": _tensor_dtype(state_dict[key]),
                "written_dtype": "F32",
                "shape": [int(x) for x in _tensor_shape(state_dict[key])],
            }
        )
    return {
        "source_checkpoint": str(Path(checkpoint)),
        "source_sha256": source_sha,
        "source_size": source_size,
        "expected_sha256": EXPECTED_SHA256,
        "expected_size": EXPECTED_SIZE,
        "profile": dict(profile),
        "output": str(Path(output)),
        "output_sha256": output_sha,
        "validated_schema": schema_version,
        "num_keys": len(state_dict),
        "keys": sorted(state_dict.keys()),
        "parameters": parameters,
        "matched_keys": matched,
        "missing_keys": missing,
        "unexpected_keys": unexpected,
        "matched_count": len(matched),
        "missing_count": len(missing),
        "unexpected_count": len(unexpected),
        "counts": {
            "matched": len(matched),
            "missing": len(missing),
            "unexpected": len(unexpected),
        },
    }


def write_manifest(path: Path, manifest: Mapping[str, Any]) -> None:
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=2, sort_keys=True)
        f.write("\n")


def convert_checkpoint(
    checkpoint: Path,
    output: Path,
    manifest_path: Optional[Path] = None,
    expected_keys: Any = None,
    schema: Any = None,
    profile: Mapping[str, Any] = PROFILE,
) -> Dict[str, Any]:
    checkpoint = Path(checkpoint)
    output = Path(output)
    if manifest_path is None:
        manifest_path = output.with_suffix(".manifest.json")

    if schema is None and expected_keys is None:
        schema = V7_SCHEMA

    checkpoint_info = validate_checkpoint_file(checkpoint)
    validate_profile(profile)

    state_dict = torch_load_restricted(checkpoint)
    validation = validate_state_dict(state_dict, expected_keys=expected_keys, schema=schema)

    write_safetensors(output, state_dict)
    manifest = build_manifest(
        checkpoint=checkpoint,
        output=output,
        profile=profile,
        state_dict=state_dict,
        source_sha=checkpoint_info["sha256"],
        source_size=checkpoint_info["size"],
        expected_keys=expected_keys,
        schema=schema,
    )
    manifest["validation"] = validation
    manifest["dtype_conversion"] = "source dtypes -> F32 (writer emits F32 only)"
    write_manifest(manifest_path, manifest)
    return manifest


convert = convert_checkpoint


def main(argv: Optional[Sequence[str]] = None) -> int:
    parser = argparse.ArgumentParser(
        description="Convert/verify a local voc_fv7.ckpt into .safetensors"
    )
    parser.add_argument("checkpoint_positional", nargs="?")
    parser.add_argument("--checkpoint", dest="checkpoint_option")
    parser.add_argument("--output", "-o", dest="output")
    parser.add_argument("--manifest", dest="manifest")
    parser.add_argument("--schema-report", dest="schema_report", action="store_true")
    args = parser.parse_args(argv)

    checkpoint = args.checkpoint_option or args.checkpoint_positional
    try:
        if not checkpoint:
            raise ConverterError("explicit local checkpoint path is required")
        checkpoint_path = Path(checkpoint)
        if not checkpoint_path.is_file():
            raise ConverterError(f"checkpoint not found: {checkpoint_path}")

        output_path = Path(args.output) if args.output else checkpoint_path.with_suffix(".safetensors")
        manifest_path = Path(args.manifest) if args.manifest else output_path.with_suffix(".manifest.json")

        schema = V7_SCHEMA
        expected_keys = None

        if args.schema_report:
            # Read-only: load checkpoint, print diff, write nothing.
            if checkpoint_path.suffix.lower() == ".json":
                state_dict = json.loads(checkpoint_path.read_text(encoding="utf-8"))
            else:
                checkpoint_info = validate_checkpoint_file(checkpoint_path)
                state_dict = torch_load_restricted(checkpoint_path)
            report = schema_report(state_dict, schema=V7_SCHEMA)
            print(json.dumps(report, indent=2, sort_keys=True))
            return 0 if report["match"] else 1

        manifest = convert_checkpoint(
            checkpoint=checkpoint_path,
            output=output_path,
            manifest_path=manifest_path,
            expected_keys=expected_keys,
            schema=schema,
        )
        print(f"converted: {output_path}")
        print(f"manifest: {manifest_path}")
        print(f"keys: {manifest['num_keys']}")
        return 0
    except (ConverterError, OSError, ValueError, KeyError, TypeError, json.JSONDecodeError) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
