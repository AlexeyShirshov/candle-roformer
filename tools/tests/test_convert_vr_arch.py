#!/usr/bin/env python3
"""Tests for ``tools/convert_vr_arch.py`` (offline VR Arch converter).

The refusal paths run without ``torch``; the structural cases and the pinned
checksum case are skipped when ``torch`` is not installed, mirroring the
``test_convert_voc_fv7.py`` approach.
"""

from __future__ import annotations

import hashlib
import importlib.util
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CONVERTER = ROOT / "tools" / "convert_vr_arch.py"
EVIDENCE_CHECKPOINT = (
    ROOT
    / ".pdca"
    / "open-issues-r5"
    / "rv1"
    / "item-3-vr-uvr"
    / "models"
    / "2_HP-UVR.pth"
)


def load_module():
    spec = importlib.util.spec_from_file_location("convert_vr_arch", CONVERTER)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


def run_converter(*args: str) -> subprocess.CompletedProcess:
    return subprocess.run(
        [sys.executable, str(CONVERTER), *args],
        capture_output=True,
        text=True,
    )


class RefusalTests(unittest.TestCase):
    def test_missing_checkpoint_is_refused(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            missing = Path(tmp) / "nope.pth"
            result = run_converter(str(missing))
            self.assertEqual(result.returncode, 3, result.stderr)
            self.assertIn("checkpoint not found", result.stderr)

    def test_checksum_mismatch_is_refused(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fake = Path(tmp) / "fake.pth"
            fake.write_bytes(b"not a checkpoint")
            result = run_converter(
                str(fake),
                "--out",
                tmp,
                "--expect-sha256",
                "00" * 32,
            )
            self.assertEqual(result.returncode, 4, result.stderr)
            self.assertIn("sha256 mismatch", result.stderr)
            self.assertFalse((Path(tmp) / "fake.safetensors").exists())

    def test_expected_checksum_is_not_optional(self) -> None:
        module = load_module()
        self.assertEqual(len(module.EXPECTED_SHA256), 64)
        self.assertEqual(module.EXPECTED_SHA256, module.EXPECTED_SHA256.lower())

    def test_no_runtime_torch_load(self) -> None:
        offenders = []
        for directory in ("src", "examples"):
            for path in (ROOT / directory).rglob("*.rs"):
                text = path.read_text(encoding="utf-8", errors="ignore")
                if "torch.load" in text:
                    offenders.append(str(path.relative_to(ROOT)))
        self.assertEqual(offenders, [], f"runtime code must not read pickles: {offenders}")


class StructuralTests(unittest.TestCase):
    def setUp(self) -> None:
        try:
            import torch  # noqa: F401
        except Exception:  # pragma: no cover - depends on the environment
            self.skipTest("torch is not installed")
        self.module = load_module()

    def test_non_mapping_root_is_refused(self) -> None:
        import torch

        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "list.pth"
            torch.save([1, 2, 3], path)
            digest = hashlib.sha256(path.read_bytes()).hexdigest()
            result = run_converter(
                str(path), "--out", tmp, "--expect-sha256", digest
            )
            self.assertEqual(result.returncode, 5, result.stderr)
            self.assertIn("must be a tensor mapping", result.stderr)

    def test_non_float32_tensor_is_refused(self) -> None:
        import torch

        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "int.pth"
            torch.save(
                {
                    "stg1_low_band_net.enc1.conv1.conv.0.weight": torch.zeros(
                        (1, 1, 1, 1), dtype=torch.int32
                    )
                },
                path,
            )
            digest = hashlib.sha256(path.read_bytes()).hexdigest()
            result = run_converter(
                str(path), "--out", tmp, "--expect-sha256", digest
            )
            self.assertEqual(result.returncode, 5, result.stderr)
            self.assertIn("must be float32", result.stderr)

    def test_missing_cascade_prefix_is_refused(self) -> None:
        import torch

        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "other.pth"
            torch.save({"some.weight": torch.zeros((2, 2))}, path)
            digest = hashlib.sha256(path.read_bytes()).hexdigest()
            result = run_converter(
                str(path), "--out", tmp, "--expect-sha256", digest
            )
            self.assertEqual(result.returncode, 5, result.stderr)
            self.assertIn("not a CascadedASPPNet", result.stderr)

    def test_pinned_checkpoint_is_deterministic(self) -> None:
        if not EVIDENCE_CHECKPOINT.exists():
            self.skipTest("pinned 2_HP-UVR.pth is not present")
        with tempfile.TemporaryDirectory() as tmp:
            result = run_converter(
                str(EVIDENCE_CHECKPOINT), "--out", tmp, "--check-only"
            )
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertIn('"nn_arch": 123812', result.stdout)
            self.assertNotIn("safetensors", os.listdir(tmp))


if __name__ == "__main__":
    unittest.main()
