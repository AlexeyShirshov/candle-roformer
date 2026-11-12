#!/usr/bin/env python3
"""Tests for ``tools/convert_demucs.py`` (offline Demucs v4 converter).

The refusal paths run without ``torch``; the structural cases and the pinned
checksum case are skipped when ``torch`` is not installed, mirroring the
``test_convert_vr_arch.py`` approach.
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
CONVERTER = ROOT / "tools" / "convert_demucs.py"
EVIDENCE_CHECKPOINT = (
    ROOT
    / ".pdca"
    / "open-issues-r5"
    / "rv1"
    / "item-4-demucs"
    / "models"
    / "955717e8-8726e21a.th"
)


def load_module():
    spec = importlib.util.spec_from_file_location("convert_demucs", CONVERTER)
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


class HTDemucs:  # module-level stand-in so torch.save can pickle it by reference
    pass


class RefusalTests(unittest.TestCase):
    def test_missing_checkpoint_is_refused(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            missing = Path(tmp) / "nope.th"
            result = run_converter(str(missing))
            self.assertEqual(result.returncode, 3, result.stderr)
            self.assertIn("checkpoint not found", result.stderr)

    def test_checksum_mismatch_is_refused(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            fake = Path(tmp) / "fake.th"
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

    def test_pinned_checksum_is_not_optional(self) -> None:
        module = load_module()
        self.assertEqual(len(module.EXPECTED_SHA256), 64)
        self.assertEqual(module.EXPECTED_SHA256, module.EXPECTED_SHA256.lower())
        self.assertEqual(len(module.EXPECTED_KEYS_SHA256), 64)
        self.assertEqual(module.EXPECTED_TENSOR_COUNT, 533)

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

    def _package(self, path: Path, state, *, klass=None, kwargs=None) -> str:
        import torch

        # The package stores its model class by reference; the converter installs
        # `demucs.*` stubs for the same reason, so register them here as well.
        stub = self.module.install_demucs_stubs()["HTDemucs"]
        torch.save(
            {
                "klass": klass or stub,
                "args": (),
                "kwargs": kwargs
                or {
                    "sources": ["drums", "bass", "other", "vocals"],
                    "samplerate": 44100,
                    "audio_channels": 2,
                    "segment": "39/5",
                    "nfft": 4096,
                },
                "metrics": [],
                "state": state,
                "training_args": {},
            },
            path,
        )
        return hashlib.sha256(path.read_bytes()).hexdigest()

    def test_non_mapping_root_is_refused(self) -> None:
        import torch

        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "list.th"
            torch.save([1, 2, 3], path)
            digest = hashlib.sha256(path.read_bytes()).hexdigest()
            result = run_converter(str(path), "--out", tmp, "--expect-sha256", digest)
            self.assertEqual(result.returncode, 5, result.stderr)
            self.assertIn("must be a package mapping", result.stderr)

    def test_missing_package_envelope_is_refused(self) -> None:
        import torch

        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "envelope.th"
            torch.save({"state": {"a.weight": torch.zeros((2, 2), dtype=torch.float16)}}, path)
            digest = hashlib.sha256(path.read_bytes()).hexdigest()
            result = run_converter(str(path), "--out", tmp, "--expect-sha256", digest)
            self.assertEqual(result.returncode, 5, result.stderr)
            self.assertIn("missing 'klass' key", result.stderr)

    def test_non_float_tensor_is_refused(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "int.th"
            digest = self._package(path, {"a.weight": __import__("torch").zeros((2, 2), dtype=__import__("torch").int32)})
            result = run_converter(str(path), "--out", tmp, "--expect-sha256", digest)
            self.assertEqual(result.returncode, 5, result.stderr)
            self.assertIn("must be float16 or float32", result.stderr)

    def test_unexpected_key_set_is_refused(self) -> None:
        import torch

        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "other.th"
            digest = self._package(path, {"some.weight": torch.zeros((2, 2), dtype=torch.float16)})
            result = run_converter(str(path), "--out", tmp, "--expect-sha256", digest)
            self.assertEqual(result.returncode, 5, result.stderr)
            self.assertIn("unexpected key set", result.stderr)

    def test_weight_norm_pair_is_resolved(self) -> None:
        import torch

        module = self.module
        g = torch.tensor([[1.0], [2.0]], dtype=torch.float16)
        v = torch.tensor([[[3.0, 4.0]], [[0.0, 5.0]]], dtype=torch.float16)
        kept, keys, resolved = module.validate_tensors(
            {
                "x.weight_g": g,
                "x.weight_v": v,
                "x.bias": torch.zeros(2, dtype=torch.float16),
            }
        )
        self.assertEqual(resolved, ["x.weight"])
        self.assertNotIn("x.weight_g", keys)
        self.assertNotIn("x.weight_v", keys)
        # ||v|| per output channel, in float32, matching torch.nn.utils.weight_norm
        v32 = v.to(torch.float32)
        g32 = g.to(torch.float32).reshape((2, 1, 1))
        expected = v32 * (g32 / torch.norm(v32, dim=(1, 2), keepdim=True))
        self.assertTrue(torch.allclose(kept["x.weight"], expected))
        self.assertEqual(str(kept["x.weight"].dtype), "torch.float32")

    def test_pinned_checkpoint_is_deterministic(self) -> None:
        if not EVIDENCE_CHECKPOINT.exists():
            self.skipTest("pinned 955717e8-8726e21a.th is not present")
        with tempfile.TemporaryDirectory() as tmp:
            result = run_converter(
                str(EVIDENCE_CHECKPOINT), "--out", tmp, "--check-only"
            )
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertIn('"tensor_count": 533', result.stdout)
            self.assertIn('"schema": "demucs-conversion/1"', result.stdout)
            self.assertNotIn("safetensors", os.listdir(tmp))


if __name__ == "__main__":
    unittest.main()


