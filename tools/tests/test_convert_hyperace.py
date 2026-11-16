from pathlib import Path
import importlib.util
from types import SimpleNamespace
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import convert_hyperace as converter


class HyperAceConverterTests(unittest.TestCase):
    @unittest.skipUnless(importlib.util.find_spec("torch"), "torch is not installed")
    def test_shared_rotary_tables_keep_all_keys_with_independent_storage(self):
        import torch
        a = "mask_estimators.0.segm.test"
        b = "layers.0.rotary_embed.freqs"
        shared = torch.ones(4)
        state = converter.prepare_state({a: shared, b: shared}, {a: [4], b: [4]})
        self.assertEqual(set(state), {a, b})
        self.assertNotEqual(state[a].data_ptr(), state[b].data_ptr())
        self.assertTrue(torch.equal(state[a], state[b]))
        with self.assertRaisesRegex(ValueError, "non-finite"):
            converter.prepare_state({a: torch.tensor([float("nan")])}, {a: [1]})

    def test_source_refuses_wrong_size_or_hash(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "model.ckpt"
            path.write_bytes(b"bad")
            with self.assertRaisesRegex(ValueError, "size mismatch"):
                converter.validate_source(path)
            with patch.object(converter, "EXPECTED_SIZE", 3):
                with self.assertRaisesRegex(ValueError, "SHA-256 mismatch"):
                    converter.validate_source(path)

    def test_schema_requires_head_and_exact_shapes_and_keys(self):
        name = "mask_estimators.0.segm.backbone.stem.dwconv.weight"
        state = {name: SimpleNamespace(shape=(256, 1, 3, 3))}
        schema = {name: [256, 1, 3, 3]}
        converter.validate_schema(state, schema)
        with self.assertRaisesRegex(ValueError, "shape mismatch"):
            converter.validate_schema(state, {name: [1]})
        with self.assertRaisesRegex(ValueError, "key mismatch"):
            converter.validate_schema({}, schema)
        with self.assertRaisesRegex(ValueError, "key mismatch"):
            converter.validate_schema({**state, "extra": SimpleNamespace(shape=())}, schema)
        with self.assertRaisesRegex(ValueError, "does not contain"):
            converter.validate_schema({}, {})
