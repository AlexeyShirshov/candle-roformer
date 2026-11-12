#!/usr/bin/env python3
"""Offline converter/verifier for VR Arch (UVR) ``.pth`` checkpoints.

A VR checkpoint is a PyTorch pickle. Runtime code must never deserialise it:
this tool performs the one allowed offline read (``torch.load`` with
``weights_only=True``), validates the pinned checksum, shapes and dtypes, and
writes a ``.safetensors`` artifact that the Rust loader consumes.

Exit codes are part of the contract:

* ``0`` - converted (or verified, with ``--check-only``)
* ``2`` - usage error
* ``3`` - checkpoint missing / unreadable
* ``4`` - checksum mismatch or size mismatch
* ``5`` - structure rejected (not a tensor mapping, unexpected dtype, empty)
* ``6`` - output write failure

The pinned checksum is the single source of truth for the supported checkpoint.
An unknown checkpoint can only be converted when its checksum is supplied
explicitly with ``--expect-sha256``; there is no way to skip validation.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import sys
from pathlib import Path
from typing import Any, Dict, List, Mapping, Tuple

# 2_HP-UVR.pth, a published UVR model.
# https://github.com/TRvlvr/model_repo/releases/download/all_public_uvr_models/2_HP-UVR.pth
EXPECTED_SHA256 = "26d6400d2d2e90811ba2381828c8c3519196097375af524c3d0edd969acf5356"
EXPECTED_SIZE = 126782699

# nn_arch sizes recognised by the UVR loader (``select_model``).
NN_ARCH_SIZES = (31191, 33966, 56817, 123821, 123812, 129605, 218409, 537238, 537227)

# Buffers that are training-only bookkeeping and are dropped from the artifact.
DROPPED_SUFFIXES = (".num_batches_tracked",)

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


def load_state_dict(path: Path) -> Mapping[str, Any]:
    if not path.is_file():
        raise ConversionError(f"checkpoint not found: {path}", EXIT_MISSING)
    try:
        import torch  # imported lazily: only this offline converter may use it
    except Exception as exc:  # pragma: no cover - depends on the environment
        raise ConversionError(
            f"torch is required for the offline conversion step: {exc}", EXIT_MISSING
        ) from exc

    try:
        obj = torch.load(path, map_location="cpu", weights_only=True)
    except Exception as exc:
        raise ConversionError(f"restricted load failed: {exc}", EXIT_STRUCTURE) from exc

    if not isinstance(obj, Mapping):
        raise ConversionError(
            f"checkpoint root must be a tensor mapping, got {type(obj).__name__}",
            EXIT_STRUCTURE,
        )
    return obj


def validate(state: Mapping[str, Any]) -> Tuple[Dict[str, Any], List[str], List[str]]:
    """Return (kept tensors, kept keys, dropped keys) after strict validation."""
    import torch

    if not state:
        raise ConversionError("checkpoint contains no tensors", EXIT_STRUCTURE)

    kept: Dict[str, Any] = {}
    keys: List[str] = []
    dropped: List[str] = []

    for name, value in state.items():
        if not isinstance(name, str):
            raise ConversionError(f"non-string tensor key: {name!r}", EXIT_STRUCTURE)
        if not isinstance(value, torch.Tensor):
            raise ConversionError(
                f"value for {name!r} is not a tensor ({type(value).__name__})",
                EXIT_STRUCTURE,
            )
        if any(name.endswith(suffix) for suffix in DROPPED_SUFFIXES):
            if value.dtype != torch.int64:
                raise ConversionError(
                    f"{name!r} must be int64, got {value.dtype}", EXIT_STRUCTURE
                )
            dropped.append(name)
            continue
        if value.dtype != torch.float32:
            raise ConversionError(
                f"{name!r} must be float32, got {value.dtype}", EXIT_STRUCTURE
            )
        if value.ndim == 0:
            raise ConversionError(f"{name!r} is a scalar, expected a tensor", EXIT_STRUCTURE)
        if not torch.isfinite(value).all():
            raise ConversionError(f"{name!r} contains NaN or Inf", EXIT_STRUCTURE)
        kept[name] = value.contiguous()
        keys.append(name)

    if not kept:
        raise ConversionError("checkpoint contains no float32 tensors", EXIT_STRUCTURE)

    has_low = any(k.startswith("stg1_low_band_net.") for k in keys)
    has_high = any(k.startswith("stg1_high_band_net.") for k in keys)
    if not (has_low and has_high):
        raise ConversionError(
            "checkpoint is not a CascadedASPPNet: stg1_low_band_net/stg1_high_band_net "
            "tensors are missing",
            EXIT_STRUCTURE,
        )
    return kept, keys, dropped


def nn_arch_for(size: int) -> int:
    model_size = math.ceil(size / 1024)
    return min(NN_ARCH_SIZES, key=lambda candidate: abs(candidate - model_size))


def convert(checkpoint: Path, out_dir: Path, expected_sha256: str, check_only: bool) -> Dict[str, Any]:
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

    state = load_state_dict(checkpoint)
    kept, keys, dropped = validate(state)

    manifest: Dict[str, Any] = {
        "schema": "vr-arch-conversion/1",
        "source": {
            "path": checkpoint.name,
            "sha256": sha256,
            "size": size,
            "nn_arch": nn_arch_for(size),
        },
        "artifact": None,
        "tensors": {
            key: {
                "shape": list(kept[key].shape),
                "dtype": "F32",
            }
            for key in keys
        },
        "tensor_count": len(keys),
        "dropped": sorted(dropped),
        "runtime_rule": "the .pth is never read at runtime; only the .safetensors artifact is",
    }

    if check_only:
        return manifest

    out_dir.mkdir(parents=True, exist_ok=True)
    artifact = out_dir / (checkpoint.stem + ".safetensors")
    try:
        from safetensors.torch import save_file

        save_file(kept, str(artifact))
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
        description="Convert a VR Arch .pth checkpoint to safetensors (offline only)."
    )
    parser.add_argument("checkpoint", type=Path, help="path to the .pth checkpoint")
    parser.add_argument(
        "--out",
        type=Path,
        default=None,
        help="output directory (default: alongside the checkpoint)",
    )
    parser.add_argument(
        "--expect-sha256",
        default=EXPECTED_SHA256,
        help="pinned sha256 of the source checkpoint (default: the published 2_HP-UVR.pth)",
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
            args.checkpoint, out_dir, args.expect_sha256.lower(), args.check_only
        )
    except ConversionError as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return exc.code
    print(json.dumps(manifest, indent=2, sort_keys=True))
    return EXIT_OK


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
