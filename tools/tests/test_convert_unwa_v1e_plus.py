import importlib.util
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

TOOLS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(TOOLS))
import convert_unwa_v1e_plus as converter


class UnwaConverterTests(unittest.TestCase):
    def test_profile_schema_is_384_by_6(self):
        schema = converter.build_v7_schema(converter.UNWA_PROFILE)["parameters"]
        self.assertEqual(len(schema), 684)
        self.assertEqual(
            schema["layers.5.1.layers.0.0.to_qkv.weight"]["shape"], [1536, 384]
        )
        self.assertNotIn("layers.6.0.layers.0.0.to_qkv.weight", schema)

    def test_source_checks_size_and_digest_before_loading(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "model.ckpt"
            path.write_bytes(b"test")
            digest = converter.sha256_file(path)
            with self.assertRaisesRegex(ValueError, "size mismatch"):
                converter.validate_source(path, digest)
            with patch.object(converter, "EXPECTED_SIZE", 4):
                self.assertEqual(converter.validate_source(path, digest), digest)
                with self.assertRaisesRegex(ValueError, "SHA-256 mismatch"):
                    converter.validate_source(path, "0" * 64)
            with self.assertRaisesRegex(ValueError, "64-digit"):
                converter.validate_source(path, "invalid")

    @unittest.skipUnless(importlib.util.find_spec("torch"), "torch is not installed")
    def test_bad_keys_and_shapes_are_rejected(self):
        import torch
        with self.assertRaisesRegex(ValueError, "key mismatch"):
            converter.prepare_state({"extra": torch.zeros(1)})
        schema = converter.build_v7_schema(converter.UNWA_PROFILE)["parameters"]
        # Key set is complete, but the first tensor deliberately has wrong shape.
        state = {name: torch.zeros(1) for name in schema}
        with self.assertRaisesRegex(ValueError, "shape mismatch"):
            converter.prepare_state(state)


if __name__ == "__main__":
    unittest.main()
