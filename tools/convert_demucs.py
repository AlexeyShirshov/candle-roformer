#!/usr/bin/env python3
"""Offline converter/verifier for Demucs v4 (``.th``) checkpoints.

A Demucs v4 checkpoint is a PyTorch pickle package (a dict with ``klass``,
``args``, ``kwargs`` and ``state``). Runtime code must never deserialise it:
this tool performs the one allowed offline read, behind an allow-list
unpickler (the same global set audio-separator's ``demucs.states`` permits),
validates the pinned checksum, the package structure, the key set and the
dtypes, resolves legacy weight-norm pairs, upcasts fp16 -> fp32 and writes a
``.safetensors`` artifact that the Rust loader consumes.

Exit codes are part of the contract:

* ``0`` - converted (or verified, with ``--check-only``)
* ``2`` - usage error
* ``3`` - checkpoint missing / unreadable
* ``4`` - checksum mismatch or size mismatch
* ``5`` - structure rejected (non-mapping root, wrong model class, non-float
  dtype, NaN/Inf, unexpected key set)
* ``6`` - output write failure

The pinned checksum and key-set digest are the single source of truth for the
supported checkpoint. Any other checkpoint can only be converted when both are
supplied explicitly with ``--expect-sha256`` / ``--expect-keys-sha256``; there
is no way to skip validation.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import pickle
import sys
from pathlib import Path
from typing import Any, Dict, List, Mapping, Tuple

# https://dl.fbaipublicfiles.com/demucs/hybrid_transformer/955717e8-8726e21a.th
# "Demucs v4: htdemucs" as registered by audio-separator 0.48.0.
EXPECTED_SHA256 = "8726e21a993978c7ba086d3872e7608d7d5bfca646ca4aca459ffda844faa8b4"
EXPECTED_SIZE = 84141911
EXPECTED_TENSOR_COUNT = 533
# sha256 of "\n".join(sorted(state keys)) - pins the exact tensor schema.
EXPECTED_KEYS_SHA256 = "33ddc01ddcd9571a5278491255edc1cd12484d6f6a0c29696823bc1c611e5fbe"
EXPECTED_KLASS = "HTDemucs"
EXPECTED_SOURCES = ("drums", "bass", "other", "vocals")

# Every global a Demucs v4 package references, as (module, name). Mirrors
# audio_separator.separator.uvr_lib_v5.demucs.states._ALLOWED_PACKAGE_GLOBALS.
ALLOWED_GLOBALS = frozenset(
    {
        ("collections", "OrderedDict"),
        ("torch._utils", "_rebuild_tensor_v2"),
        ("torch._utils", "_rebuild_parameter"),
        ("_codecs", "encode"),
        ("fractions", "Fraction"),
        ("numpy", "dtype"),
        ("numpy.core.multiarray", "scalar"),
        ("numpy._core.multiarray", "scalar"),
        ("demucs.demucs", "Demucs"),
        ("demucs.hdemucs", "HDemucs"),
        ("demucs.htdemucs", "HTDemucs"),
    }
)

SOURCE_DTYPES = ("torch.float16", "torch.float32")

EXIT_OK = 0
EXIT_USAGE = 2
EXIT_MISSING = 3
EXIT_CHECKSUM = 4
EXIT_STRUCTURE = 5
EXIT_WRITE = 6


class ConversionError(Exception):
    """A refusal with a stable exit code and a user facing message."""

    def __init__(self, message: str, code: int) -> None:
        super().__init__(message)
        self.code = code


def sha256_file(path: Path, chunk: int = 1 << 20) -> Tuple[str, int]:
    digest = hashlib.sha256()
    size = 0
    with path.open("rb") as handle:
        while True:
            block = handle.read(chunk)
            if not block:
                break
            size += len(block)
            digest.update(block)
    return digest.hexdigest(), size


def keys_digest(keys: List[str]) -> str:
    return hashlib.sha256("\n".join(sorted(keys)).encode("utf-8")).hexdigest()


class _RestrictedUnpickler(pickle.Unpickler):
    def find_class(self, module: str, name: str) -> Any:
        if (module, name) in ALLOWED_GLOBALS or (
            module == "numpy.dtypes" and name.endswith("DType")
        ):
            return super().find_class(module, name)
        raise pickle.UnpicklingError(
            f"refusing to resolve '{module}.{name}' while loading a Demucs package"
        )


class _RestrictedPickleModule:
    """pickle-compatible shim for ``torch.load(pickle_module=...)``."""

    __name__ = "convert_demucs_restricted_pickle"
    Unpickler = _RestrictedUnpickler
    load = staticmethod(lambda f, **kwargs: _RestrictedUnpickler(f, **kwargs).load())


def install_demucs_stubs() -> Dict[str, Any]:
    """Register ``demucs.{demucs,hdemucs,htdemucs}`` stubs so a package can be read.

    The package stores its model class by reference (``demucs.htdemucs.HTDemucs``).
    The class body is never executed and never used: the converter only reads its
    ``__name__``, so a stub class with the right identity is enough. Real modules,
    when importable, are left untouched.
    """
    import types

    classes: Dict[str, Any] = {}
    specs = {
        "demucs": {},
        "demucs.demucs": {"Demucs": "Demucs"},
        "demucs.hdemucs": {"HDemucs": "HDemucs"},
        "demucs.htdemucs": {"HTDemucs": "HTDemucs"},
    }
    for module_name, attrs in specs.items():
        module = sys.modules.get(module_name)
        if module is None:
            try:
                __import__(module_name)
                module = sys.modules[module_name]
            except Exception:
                module = types.ModuleType(module_name)
                sys.modules[module_name] = module
        for attr, klass_name in attrs.items():
            existing = getattr(module, attr, None)
            if existing is None:
                klass = type(klass_name, (), {})
                klass.__module__ = module_name
                klass.__qualname__ = klass_name
                setattr(module, attr, klass)
                existing = klass
            classes[attr] = existing
    return classes


def load_package(path: Path) -> Mapping[str, Any]:
    if not path.is_file():
        raise ConversionError(f"checkpoint not found: {path}", EXIT_MISSING)
    try:
        import torch  # imported lazily: only this offline converter may use it
    except Exception as exc:  # pragma: no cover - depends on the environment
        raise ConversionError(
            f"torch is required for the offline conversion step: {exc}", EXIT_MISSING
        ) from exc

    # The package stores its model class by reference; the stub modules make that
    # reference resolvable in an environment that has no demucs package.
    install_demucs_stubs()

    try:
        package = torch.load(
            path,
            map_location="cpu",
            pickle_module=_RestrictedPickleModule,
            weights_only=False,
        )
    except ConversionError:
        raise
    except Exception as exc:
        raise ConversionError(f"restricted load failed: {exc}", EXIT_STRUCTURE) from exc

    if not isinstance(package, Mapping):
        raise ConversionError(
            f"checkpoint root must be a package mapping, got {type(package).__name__}",
            EXIT_STRUCTURE,
        )
    return package


def validate_package(package: Mapping[str, Any]) -> Mapping[str, Any]:
    """Return the ``state`` mapping after validating the package envelope."""
    for required in ("klass", "args", "kwargs", "state"):
        if required not in package:
            raise ConversionError(
                f"package is not a {EXPECTED_KLASS} model package: missing {required!r} key",
                EXIT_STRUCTURE,
            )
    klass = package.get("klass")
    klass_name = getattr(klass, "__name__", klass)
    if not isinstance(klass_name, str) or klass_name != EXPECTED_KLASS:
        raise ConversionError(
            f"checkpoint is not a {EXPECTED_KLASS} package (klass={klass_name!r})",
            EXIT_STRUCTURE,
        )
    kwargs = package.get("kwargs")
    if not isinstance(kwargs, Mapping):
        raise ConversionError("package kwargs must be a mapping", EXIT_STRUCTURE)
    sources = tuple(kwargs.get("sources") or ())
    if sources != EXPECTED_SOURCES:
        raise ConversionError(
            f"unexpected source list {sources!r}, expected {EXPECTED_SOURCES!r}",
            EXIT_STRUCTURE,
        )
    state = package.get("state")
    if not isinstance(state, Mapping):
        raise ConversionError("package state must be a tensor mapping", EXIT_STRUCTURE)
    return state


def _resolve_weight_norm(state: Mapping[str, Any]) -> Tuple[Dict[str, Any], List[str]]:
    """Replace ``*.weight_g``/``*.weight_v`` pairs by their fused ``*.weight``.

    PyTorch's ``torch.nn.utils.weight_norm`` stores ``g`` and ``v``; the module
    parameter is ``weight = g * v / ||v||`` with the norm taken over every axis
    but the first. The fusion here is deterministic and recorded in the
    manifest. This checkpoint has no such pairs (they are listed as resolved
    only if present), but a checkpoint that does is accepted without ever
    leaving the norm unresolved at runtime.
    """
    import torch

    fused: Dict[str, Any] = {}
    resolved: List[str] = []
    for name, value in state.items():
        fused[name] = value
    for name in list(fused):
        if not name.endswith(".weight_g"):
            continue
        prefix = name[: -len(".weight_g")]
        v_name = prefix + ".weight_v"
        if v_name not in fused:
            raise ConversionError(
                f"weight norm pair is incomplete: {name!r} has no {v_name!r}",
                EXIT_STRUCTURE,
            )
        g = fused.pop(name)
        v = fused.pop(v_name)
        # Fuse in float32: the reference loads fp16 weights into float32
        # parameters, so the fused weight must not carry fp16 rounding.
        g = g.to(torch.float32)
        v = v.to(torch.float32)
        flat_dims = tuple(range(1, v.dim()))
        norm = torch.linalg.vector_norm(v, dim=flat_dims, keepdim=True)
        g = g.reshape((g.shape[0],) + (1,) * (v.dim() - 1))
        fused[prefix + ".weight"] = (v * (g / norm)).contiguous()
        resolved.append(prefix + ".weight")
    return fused, sorted(resolved)


def validate(
    state: Mapping[str, Any],
) -> Tuple[Dict[str, Any], List[str], List[str]]:
    """Public entry point used by the tests: validate tensors of a bare state dict."""
    return validate_tensors(state)


def validate_tensors(
    state: Mapping[str, Any],
) -> Tuple[Dict[str, Any], List[str], List[str]]:
    """Return (float32 tensors, keys, resolved weight-norm names)."""
    import torch

    state, weight_norm_resolved = _resolve_weight_norm(state)
    if not state:
        raise ConversionError("checkpoint contains no tensors", EXIT_STRUCTURE)

    kept: Dict[str, Any] = {}
    keys: List[str] = []
    for name, value in state.items():
        if not isinstance(name, str):
            raise ConversionError(f"non-string tensor key: {name!r}", EXIT_STRUCTURE)
        if name.endswith(".weight_g") or name.endswith(".weight_v"):
            raise ConversionError(
                f"unresolved weight norm tensor {name!r}", EXIT_STRUCTURE
            )
        if not isinstance(value, torch.Tensor):
            raise ConversionError(
                f"value for {name!r} is not a tensor ({type(value).__name__})",
                EXIT_STRUCTURE,
            )
        dtype = str(value.dtype)
        if dtype not in SOURCE_DTYPES:
            raise ConversionError(
                f"{name!r} must be float16 or float32, got {dtype}", EXIT_STRUCTURE
            )
        if value.ndim == 0:
            raise ConversionError(f"{name!r} is a scalar, expected a tensor", EXIT_STRUCTURE)
        if not torch.isfinite(value).all():
            raise ConversionError(f"{name!r} contains NaN or Inf", EXIT_STRUCTURE)
        kept[name] = value.to(torch.float32).contiguous()
        keys.append(name)
    return kept, keys, weight_norm_resolved


def convert(
    checkpoint: Path,
    out_dir: Path,
    expected_sha256: str,
    expected_keys_sha256: str,
    check_only: bool,
) -> Dict[str, Any]:
    if not checkpoint.is_file():
        raise ConversionError(f"checkpoint not found: {checkpoint}", EXIT_MISSING)
    sha256, size = sha256_file(checkpoint)
    if size != EXPECTED_SIZE and expected_sha256 == EXPECTED_SHA256:
        raise ConversionError(
            f"size mismatch for the pinned checkpoint: {size} != {EXPECTED_SIZE}",
            EXIT_CHECKSUM,
        )
    if sha256 != expected_sha256:
        raise ConversionError(
            f"sha256 mismatch: {sha256} != {expected_sha256}", EXIT_CHECKSUM
        )

    package = load_package(checkpoint)
    state = validate_package(package)
    tensors, keys, weight_norm_resolved = validate_tensors(state)

    digest = keys_digest(keys)
    if digest != expected_keys_sha256:
        raise ConversionError(
            f"unexpected key set: {len(keys)} tensors, keys sha256 {digest} != "
            f"{expected_keys_sha256}",
            EXIT_STRUCTURE,
        )
    if expected_sha256 == EXPECTED_SHA256 and len(keys) != EXPECTED_TENSOR_COUNT:
        raise ConversionError(
            f"unexpected tensor count for the pinned checkpoint: {len(keys)} != "
            f"{EXPECTED_TENSOR_COUNT}",
            EXIT_STRUCTURE,
        )

    kwargs = package.get("kwargs") or {}
    manifest: Dict[str, Any] = {
        "schema": "demucs-conversion/1",
        "source": {
            "path": checkpoint.name,
            "sha256": sha256,
            "size": size,
            "klass": EXPECTED_KLASS,
            "sources": list(EXPECTED_SOURCES),
            "samplerate": int(kwargs.get("samplerate", 0)),
            "segment": str(kwargs.get("segment", "")),
            "audio_channels": int(kwargs.get("audio_channels", 0)),
            "source_dtype": sorted({str(v.dtype) for v in state.values() if hasattr(v, "dtype")}),
        },
        "artifact": None,
        "tensors": {
            key: {"shape": list(tensors[key].shape), "dtype": "F32"} for key in keys
        },
        "tensor_count": len(keys),
        "keys_sha256": digest,
        "weight_norm_resolved": weight_norm_resolved,
        "runtime_rule": "the .th is never read at runtime; only the .safetensors artifact is",
    }

    if check_only:
        return manifest

    out_dir.mkdir(parents=True, exist_ok=True)
    artifact = out_dir / (checkpoint.stem + ".safetensors")
    try:
        from safetensors.torch import save_file

        save_file(tensors, str(artifact))
    except Exception as exc:
        raise ConversionError(f"failed to write {artifact}: {exc}", EXIT_WRITE) from exc

    artifact_sha256, artifact_size = sha256_file(artifact)
    manifest["artifact"] = {
        "path": artifact.name,
        "sha256": artifact_sha256,
        "size": artifact_size,
    }
    manifest_path = out_dir / (checkpoint.stem + ".manifest.json")
    manifest_path.write_text(
        json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    return manifest


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Convert a Demucs v4 .th checkpoint to safetensors (offline only)."
    )
    parser.add_argument("checkpoint", type=Path, help="path to the .th checkpoint")
    parser.add_argument(
        "--out",
        type=Path,
        default=None,
        help="output directory (default: alongside the checkpoint)",
    )
    parser.add_argument(
        "--expect-sha256",
        default=EXPECTED_SHA256,
        help="pinned sha256 of the source checkpoint (default: the published htdemucs)",
    )
    parser.add_argument(
        "--expect-keys-sha256",
        default=EXPECTED_KEYS_SHA256,
        help="pinned sha256 of the sorted state-dict key list",
    )
    parser.add_argument(
        "--check-only",
        action="store_true",
        help="validate and print the manifest without writing an artifact",
    )
    return parser


def main(argv: List[str]) -> int:
    args = build_parser().parse_args(argv)
    out_dir = args.out if args.out is not None else args.checkpoint.parent
    try:
        manifest = convert(
            args.checkpoint,
            out_dir,
            args.expect_sha256.lower(),
            args.expect_keys_sha256.lower(),
            args.check_only,
        )
    except ConversionError as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return exc.code
    print(json.dumps(manifest, indent=2, sort_keys=True))
    return EXIT_OK


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

