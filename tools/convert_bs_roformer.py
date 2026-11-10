#!/usr/bin/env python3
"""Offline developer converter for a published BS-RoFormer `.ckpt`.

The runtime never reads a PyTorch pickle: this tool performs the one-time
conversion to safetensors and refuses to write anything unless every tensor is
accounted for.

What it does:

* hashes the source and refuses a file whose size/SHA-256 is not the pinned,
  published one;
* reads the pickle with ``torch.load(..., weights_only=True)``;
* derives the exact key/shape schema from the UVR profile, so a missing, extra
  or reshaped tensor is an error (no silent drop, no rename);
* writes f32 safetensors and emits a manifest with both hashes.

``torch`` and ``safetensors`` are imported lazily; nothing else is required
beyond the standard library (the profile is parsed with a small reader for its
``model:`` block, which also tolerates the ``!!python/tuple`` tag used by the
published UVR profiles).

Usage:
  python tools/convert_bs_roformer.py --ckpt model.ckpt \\
      --config configs/bs_roformer_viperx_1297.yaml --out model.safetensors
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from typing import Any, Dict, List, Tuple

EXPECTED_SHA256 = "5b84f37e8d444c8cb30c79d77f613a41c05868ff9c9ac6c7049c00aefae115aa"
EXPECTED_SIZE = 639331213


def sha256_file(path: str) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def _coerce(value: str) -> Any:
    low = value.lower()
    if low in ("true", "false"):
        return low == "true"
    try:
        return int(value)
    except ValueError:
        pass
    try:
        return float(value)
    except ValueError:
        return value


def parse_model_block(path: str) -> Dict[str, Any]:
    """Read the scalar/list entries of the profile's ``model:`` block."""
    with open(path, "r", encoding="utf-8") as f:
        text = f.read().replace("!!python/tuple", "")
    out: Dict[str, Any] = {}
    section = None
    list_key = None
    for raw in text.splitlines():
        line = raw.split("#", 1)[0].rstrip()
        if not line.strip():
            continue
        indent = len(line) - len(line.lstrip())
        stripped = line.strip()
        if indent == 0 and stripped.endswith(":"):
            section = stripped[:-1]
            list_key = None
            continue
        if section != "model":
            continue
        if stripped.startswith("- "):
            if list_key is None:
                raise SystemExit(f"unexpected list item in model block: {stripped!r}")
            out[list_key].append(int(stripped[2:].strip()))
            continue
        if ":" not in stripped:
            continue
        key, value = stripped.split(":", 1)
        key, value = key.strip(), value.strip()
        list_key = None
        if value == "":
            out[key] = []
            list_key = key
        else:
            out[key] = _coerce(value)
    return out


def expected_schema(cfg: Dict[str, Any]) -> Dict[str, Tuple[int, ...]]:
    """Mirror of ``model::bs_roformer::expected_key_shapes``."""
    dim = int(cfg["dim"])
    depth = int(cfg["depth"])
    channels = 2 if cfg.get("stereo", False) else 1
    freqs_per_bands: List[int] = [int(v) for v in cfg["freqs_per_bands"]]
    heads = int(cfg.get("heads", 8))
    dim_head = int(cfg.get("dim_head", 64))
    ff_mult = int(cfg.get("mlp_expansion_factor", 4))
    hidden = dim * ff_mult
    inner = heads * dim_head
    stems = int(cfg.get("num_stems", 2))
    mask_depth = max(1, int(cfg.get("mask_estimator_depth", 2)))
    time_depth = max(1, int(cfg.get("time_transformer_depth", 2)))
    freq_depth = max(1, int(cfg.get("freq_transformer_depth", 2)))

    schema: Dict[str, Tuple[int, ...]] = {}
    dim_inputs = [2 * f * channels for f in freqs_per_bands]
    for i, dim_in in enumerate(dim_inputs):
        schema[f"band_split.to_features.{i}.0.gamma"] = (dim_in,)
        schema[f"band_split.to_features.{i}.1.weight"] = (dim, dim_in)
        schema[f"band_split.to_features.{i}.1.bias"] = (dim,)

    def transformer(prefix: str, layers: int) -> None:
        for d in range(layers):
            p = f"{prefix}.layers.{d}"
            schema[f"{p}.0.norm.gamma"] = (dim,)
            schema[f"{p}.0.to_qkv.weight"] = (3 * inner, dim)
            schema[f"{p}.0.to_gates.weight"] = (heads, dim)
            schema[f"{p}.0.to_gates.bias"] = (heads,)
            schema[f"{p}.0.to_out.0.weight"] = (dim, inner)
            schema[f"{p}.0.rotary_embed.freqs"] = (dim_head // 2,)
            schema[f"{p}.1.net.0.gamma"] = (dim,)
            schema[f"{p}.1.net.1.weight"] = (dim * ff_mult, dim)
            schema[f"{p}.1.net.1.bias"] = (dim * ff_mult,)
            schema[f"{p}.1.net.4.weight"] = (dim, dim * ff_mult)
            schema[f"{p}.1.net.4.bias"] = (dim,)

    for li in range(depth):
        transformer(f"layers.{li}.0", time_depth)
        transformer(f"layers.{li}.1", freq_depth)
    schema["final_norm.gamma"] = (dim,)
    for s in range(stems):
        for i, dim_in in enumerate(dim_inputs):
            p = f"mask_estimators.{s}.to_freqs.{i}.0"
            for j in range(mask_depth):
                in_dim = dim if j == 0 else hidden
                out_dim = 2 * dim_in if j + 1 == mask_depth else hidden
                schema[f"{p}.{2 * j}.weight"] = (out_dim, in_dim)
                schema[f"{p}.{2 * j}.bias"] = (out_dim,)
    return schema


def unwrap(state: Any) -> Tuple[Any, Dict[str, Any]]:
    if isinstance(state, dict) and isinstance(state.get("state_dict"), dict):
        return "state_dict", state["state_dict"]
    if isinstance(state, dict) and isinstance(state.get("model"), dict):
        return "model", state["model"]
    return None, state


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--ckpt", required=True)
    ap.add_argument("--config", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--manifest", default=None)
    ap.add_argument("--allow-unpinned", action="store_true")
    args = ap.parse_args()

    size = os.path.getsize(args.ckpt)
    digest = sha256_file(args.ckpt)
    if not args.allow_unpinned and (size != EXPECTED_SIZE or digest != EXPECTED_SHA256):
        raise SystemExit(
            "refusing unpinned checkpoint: size %d (expected %d), sha256 %s (expected %s); "
            "pass --allow-unpinned only for a different published file"
            % (size, EXPECTED_SIZE, digest, EXPECTED_SHA256)
        )

    import torch
    from safetensors.torch import save_file

    cfg = parse_model_block(args.config)
    schema = expected_schema(cfg)

    raw = torch.load(args.ckpt, map_location="cpu", weights_only=True)
    wrapper, state = unwrap(raw)
    state = dict(state)
    prefix = None
    heads = {k.split(".", 1)[0] + "." for k in state if "." in k}
    if len(heads) == 1:
        p = next(iter(heads))
        if p in ("model.", "state_dict.", "_orig_mod."):
            prefix = p
            state = {k[len(p):]: v for k, v in state.items()}

    missing = sorted(set(schema) - set(state))
    extra = sorted(set(state) - set(schema))
    bad_shape = sorted(
        f"{k}: {tuple(state[k].shape)} != {schema[k]}"
        for k in set(schema) & set(state)
        if tuple(state[k].shape) != schema[k]
    )
    if missing or extra or bad_shape:
        raise SystemExit(
            "checkpoint does not match the profile schema\n"
            f"missing ({len(missing)}): {missing[:10]}\n"
            f"extra ({len(extra)}): {extra[:10]}\n"
            f"shape ({len(bad_shape)}): {bad_shape[:10]}"
        )

    out_state = {
        k: v.detach().clone().to(torch.float32).contiguous() for k, v in state.items()
    }
    save_file(out_state, args.out)

    manifest = {
        "source": os.path.abspath(args.ckpt),
        "source_sha256": digest,
        "source_bytes": size,
        "config": os.path.abspath(args.config),
        "config_sha256": sha256_file(args.config),
        "wrapper": wrapper,
        "stripped_prefix": prefix,
        "key_count": len(out_state),
        "param_count": int(sum(v.numel() for v in out_state.values())),
        "out": os.path.abspath(args.out),
        "out_sha256": sha256_file(args.out),
        "out_bytes": os.path.getsize(args.out),
        "torch": torch.__version__,
    }
    if args.manifest:
        with open(args.manifest, "w", encoding="utf-8") as f:
            json.dump(manifest, f, indent=2)
    print(json.dumps(manifest, indent=2))


if __name__ == "__main__":
    main()
