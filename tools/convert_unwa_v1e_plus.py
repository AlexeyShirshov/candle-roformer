#!/usr/bin/env python3
"""Offline Unwa Inst v1e Plus conversion. Requires torch and safetensors.

The caller supplies a trusted source SHA-256; no unverified pickle fallback.
Names and shapes must exactly match the supported MelBand 384/6 schema.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path

from convert_voc_fv7 import PROFILE, build_v7_schema, sha256_file

EXPECTED_SIZE = 913090472
UNWA_PROFILE = {**PROFILE, "dim": 384, "depth": 6, "use_torch_checkpoint": False}


def validate_source(path: Path, expected_sha256: str) -> str:
    digest = expected_sha256.lower()
    if len(digest) != 64 or any(c not in "0123456789abcdef" for c in digest):
        raise ValueError("--sha256 must be a trusted 64-digit SHA-256")
    if path.stat().st_size != EXPECTED_SIZE:
        raise ValueError(f"checkpoint size mismatch (expected {EXPECTED_SIZE})")
    actual = sha256_file(path)
    if actual != digest:
        raise ValueError("checkpoint SHA-256 mismatch")
    return actual


def prepare_state(state):
    import torch

    if isinstance(state, dict) and "state_dict" in state:
        state = state["state_dict"]
    if not isinstance(state, dict):
        raise ValueError("checkpoint must contain a state dict")
    schema = build_v7_schema(UNWA_PROFILE)["parameters"]
    if set(state) != set(schema):
        raise ValueError(
            f"key mismatch: missing={sorted(set(schema) - set(state))}, "
            f"unexpected={sorted(set(state) - set(schema))}"
        )
    output = {}
    for name, spec in schema.items():
        tensor = state[name]
        if not isinstance(tensor, torch.Tensor) or list(tensor.shape) != spec["shape"]:
            raise ValueError(f"tensor shape mismatch: {name}")
        if not tensor.is_floating_point() or not torch.isfinite(tensor).all():
            raise ValueError(f"non-floating or non-finite tensor: {name}")
        tensor = tensor.detach().cpu().float().contiguous().clone()
        if not torch.isfinite(tensor).all():
            raise ValueError(f"F32 conversion overflow: {name}")
        output[name] = tensor
    return output


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ckpt", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--sha256", required=True, help="Trusted source checkpoint SHA-256")
    args = parser.parse_args()
    if args.ckpt.resolve() == args.out.resolve():
        parser.error("source and output must differ")
    digest = validate_source(args.ckpt, args.sha256)
    import torch
    from safetensors.torch import save_file

    state = prepare_state(torch.load(args.ckpt, map_location="cpu", weights_only=True))
    save_file(state, str(args.out))
    args.out.with_suffix(".manifest.json").write_text(
        json.dumps({
            "source_sha256": digest,
            "source_size": EXPECTED_SIZE,
            "output_sha256": sha256_file(args.out),
            "profile": UNWA_PROFILE,
            "target_instrument": "other",
            "tensor_count": len(state),
        }, indent=2) + "\n", encoding="utf-8"
    )


if __name__ == "__main__":
    main()
