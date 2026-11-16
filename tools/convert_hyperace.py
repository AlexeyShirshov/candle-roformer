#!/usr/bin/env python3
"""Offline converter for Unwa BS-RoFormer HyperACE v2 Inst.

Export --schema using the checkpoint_schema Rust example first. This uses the
runtime's single schema, not a separately maintained Python approximation.
Requires torch and safetensors. No pickle fallback or download logic.
"""
import argparse
import hashlib
import json
from pathlib import Path

EXPECTED_SIZE = 288725646
EXPECTED_SHA256 = "4d61178ef966d2b4e9ad456ffbbc6fd5b2828df07a7af32931142c1a5ff1fe6f"
SOURCE_REVISION = "5b1f8283125d5e4a3614d0e3635a636e09c84059"


def sha256_file(path):
    h = hashlib.sha256()
    with Path(path).open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def validate_source(path):
    if Path(path).stat().st_size != EXPECTED_SIZE:
        raise ValueError("HyperACE v2 Inst checkpoint size mismatch")
    if sha256_file(path) != EXPECTED_SHA256:
        raise ValueError("HyperACE v2 Inst checkpoint SHA-256 mismatch")


def validate_schema(state, schema):
    if not isinstance(state, dict) or not isinstance(schema, dict):
        raise ValueError("state and schema must be mappings")
    if not any(k.startswith("mask_estimators.0.segm.") for k in schema):
        raise ValueError("schema does not contain the HyperACE head")
    missing = sorted(set(schema) - set(state))
    extra = sorted(set(state) - set(schema))
    if missing or extra:
        raise ValueError(f"key mismatch: missing={missing[:10]}, extra={extra[:10]}")
    for name, shape in schema.items():
        if list(state[name].shape) != shape:
            raise ValueError(f"shape mismatch: {name}: {list(state[name].shape)} != {shape}")


def prepare_state(state, schema):
    import torch

    validate_schema(state, schema)
    converted = {}
    for name, tensor in state.items():
        if not isinstance(tensor, torch.Tensor) or not tensor.is_floating_point():
            raise ValueError(f"non-floating tensor: {name}")
        # Rotary tables are shared between blocks in PyTorch. Safetensors needs
        # one independent storage per key; preserve every alias name and value.
        tensor = tensor.detach().cpu().float().contiguous().clone()
        if not torch.isfinite(tensor).all():
            raise ValueError(f"non-finite tensor: {name}")
        converted[name] = tensor
    return converted


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ckpt", type=Path, required=True)
    parser.add_argument("--schema", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    if args.ckpt.resolve() == args.out.resolve():
        parser.error("source and output must differ")
    validate_source(args.ckpt)
    import torch
    from safetensors.torch import save_file

    state = torch.load(args.ckpt, map_location="cpu", weights_only=True)
    if isinstance(state, dict):
        for wrapper in ("state_dict", "model"):
            if isinstance(state.get(wrapper), dict):
                state = state[wrapper]
                break
    schema = json.loads(args.schema.read_text(encoding="utf-8"))
    converted = prepare_state(state, schema)
    save_file(converted, str(args.out))
    args.out.with_suffix(".manifest.json").write_text(json.dumps({
        "source_sha256": EXPECTED_SHA256,
        "source_bytes": EXPECTED_SIZE,
        "source_revision": SOURCE_REVISION,
        "schema_sha256": sha256_file(args.schema),
        "output_sha256": sha256_file(args.out),
        "tensor_count": len(converted),
        "dtype": "F32",
    }, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
