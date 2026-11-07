"""Synthetic refusal tests for tools/convert_voc_fv7.py (stdlib unittest only)."""

import json
import sys
import tempfile
import types
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
TOOLS_DIR = HERE.parent
if str(TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(TOOLS_DIR))

import convert_voc_fv7 as conv  # noqa: E402

CONVERTER_PATH = TOOLS_DIR / "convert_voc_fv7.py"


class TestCheckpointFileRefusals(unittest.TestCase):
    def setUp(self):
        self.td = Path(tempfile.mkdtemp())

    def tearDown(self):
        import shutil
        shutil.rmtree(self.td, ignore_errors=True)

    def test_missing_checkpoint_path(self):
        missing = self.td / "does_not_exist.ckpt"
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.validate_checkpoint_file(missing)
        msg = str(ctx.exception)
        self.assertIn("not found", msg.lower())
        self.assertIn(str(missing), msg)

    def test_wrong_byte_size(self):
        p = self.td / "wrong_size.ckpt"
        p.write_bytes(b"x" * 10)
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.validate_checkpoint_file(p)
        msg = str(ctx.exception)
        self.assertIn("size mismatch", msg.lower())
        self.assertIn("10", msg)

    def test_wrong_sha256(self):
        data = b"hello world"
        p = self.td / "wrong_hash.ckpt"
        p.write_bytes(data)
        orig_size = conv.EXPECTED_SIZE
        orig_sha = conv.EXPECTED_SHA256
        try:
            conv.EXPECTED_SIZE = len(data)
            conv.EXPECTED_SHA256 = "0" * 64
            with self.assertRaises(conv.ConverterError) as ctx:
                conv.validate_checkpoint_file(p)
            msg = str(ctx.exception)
            self.assertIn("sha256", msg.lower())
            self.assertIn("mismatch", msg.lower())
        finally:
            conv.EXPECTED_SIZE = orig_size
            conv.EXPECTED_SHA256 = orig_sha


class TestStateDictRefusals(unittest.TestCase):
    def test_missing_key_names_specific_key(self):
        sd = {"a": {"shape": [2], "dtype": "float32", "data": [1.0, 2.0]}}
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.validate_state_dict(sd, expected_keys={"a", "b"})
        msg = str(ctx.exception)
        self.assertIn("key mismatch", msg.lower())
        self.assertIn("missing", msg.lower())
        self.assertIn("b", msg)

    def test_extra_key_names_specific_key(self):
        sd = {
            "a": {"shape": [2], "dtype": "float32", "data": [1.0, 2.0]},
            "c": {"shape": [1], "dtype": "float32", "data": [3.0]},
        }
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.validate_state_dict(sd, expected_keys={"a", "b"})
        msg = str(ctx.exception)
        self.assertIn("key mismatch", msg.lower())
        self.assertIn("unexpected", msg.lower())
        self.assertIn("c", msg)

    def test_wrong_shape_names_specific_key(self):
        sd = {"bad": {"shape": [-1], "dtype": "float32", "data": [1.0]}}
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.validate_state_dict(sd, expected_keys={"bad"})
        msg = str(ctx.exception)
        self.assertIn("invalid shape", msg.lower())
        self.assertIn("bad", msg)


class TestRestrictedLoad(unittest.TestCase):
    def test_no_weights_only_false_in_source(self):
        text = CONVERTER_PATH.read_text(encoding="utf-8")
        self.assertNotIn("weights_only=False", text)

    def test_load_helper_raises_when_torch_missing(self):
        # torch is not installed in this environment -> typed refusal.
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.torch_load_restricted(Path("nope.ckpt"))
        self.assertIn("torch", str(ctx.exception).lower())

    def test_load_helper_raises_when_restricted_unsupported(self):
        fake_torch = types.ModuleType("torch")

        def fake_load(*args, **kwargs):
            raise TypeError("weights_only unsupported by this torch build")

        fake_torch.load = fake_load
        saved = sys.modules.get("torch", _MISSING)
        sys.modules["torch"] = fake_torch
        try:
            with self.assertRaises(conv.ConverterError) as ctx:
                conv.torch_load_restricted(Path("fake.ckpt"))
            msg = str(ctx.exception)
            self.assertIn("restricted torch.load", msg)
            self.assertIn("weights_only=True", msg)
        finally:
            if saved is _MISSING:
                sys.modules.pop("torch", None)
            else:
                sys.modules["torch"] = saved


_MISSING = object()


class TestManifestEmission(unittest.TestCase):
    def setUp(self):
        import tempfile
        self.td = Path(tempfile.mkdtemp())

    def tearDown(self):
        import shutil
        shutil.rmtree(self.td, ignore_errors=True)

    def test_valid_synthetic_state_dict_emits_safetensors_and_manifest(self):
        sd = {
            "encoder.weight": {
                "shape": [2, 3],
                "dtype": "float32",
                "data": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            },
            "encoder.bias": {
                "shape": [2],
                "dtype": "float32",
                "data": [0.1, 0.2],
            },
        }
        ckpt = self.td / "synthetic.ckpt"
        ckpt.write_bytes(b"synthetic checkpoint bytes")
        source_sha = conv.sha256_file(ckpt)
        source_size = conv.file_size(ckpt)

        out = self.td / "out.safetensors"
        conv.write_safetensors(out, sd)
        self.assertTrue(out.is_file())
        self.assertGreater(out.stat().st_size, 0)

        expected_keys = set(sd.keys())
        manifest = conv.build_manifest(
            checkpoint=ckpt,
            output=out,
            profile=conv.PROFILE,
            state_dict=sd,
            source_sha=source_sha,
            source_size=source_size,
            expected_keys=expected_keys,
        )
        mp = self.td / "out.manifest.json"
        conv.write_manifest(mp, manifest)
        self.assertTrue(mp.is_file())

        loaded = json.loads(mp.read_text(encoding="utf-8"))
        self.assertEqual(loaded["source_sha256"], source_sha)
        self.assertEqual(loaded["source_size"], source_size)
        self.assertEqual(loaded["profile"], dict(conv.PROFILE))

        # per-parameter names/dtypes/shapes
        params = {p["name"]: p for p in loaded["parameters"]}
        self.assertEqual(set(params), set(sd.keys()))
        self.assertEqual(params["encoder.weight"]["dtype"], "float32")
        self.assertEqual(params["encoder.weight"]["shape"], [2, 3])
        self.assertEqual(params["encoder.bias"]["shape"], [2])

        # counts
        self.assertEqual(loaded["matched_count"], 2)
        self.assertEqual(loaded["missing_count"], 0)
        self.assertEqual(loaded["unexpected_count"], 0)
        self.assertEqual(loaded["counts"], {"matched": 2, "missing": 0, "unexpected": 0})
        self.assertEqual(loaded["missing_keys"], [])
        self.assertEqual(loaded["unexpected_keys"], [])


class TestValidateProfile(unittest.TestCase):
    def test_v7_profile_validates(self):
        conv.validate_profile(conv.PROFILE)


class TestPositiveShapeMismatch(unittest.TestCase):
    def test_positive_shape_mismatch_names_key(self):
        sd = {"bad": {"shape": [3], "dtype": "float32", "data": [1.0, 2.0, 3.0]}}
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.validate_state_dict(sd, expected_keys={"bad": [2]})
        msg = str(ctx.exception)
        self.assertIn("shape mismatch", msg.lower())
        self.assertIn("bad", msg)
        self.assertIn("2", msg)

    def test_dtype_mismatch_names_key(self):
        sd = {"bad": {"shape": [2], "dtype": "int64", "data": [1, 2]}}
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.validate_state_dict(sd, expected_keys={"bad": {"shape": [2], "dtype": "float32"}})
        msg = str(ctx.exception)
        self.assertIn("dtype mismatch", msg.lower())
        self.assertIn("bad", msg)


class TestBufferSchema(unittest.TestCase):
    def test_missing_buffer_names_specific_buffer(self):
        sd = {"a": {"shape": [2], "dtype": "float32", "data": [1.0, 2.0]}}
        schema = {
            "version": "v7-test",
            "parameters": {"a": {"shape": [2], "dtype": "float32"}},
            "buffers": ["buf_x"],
        }
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.validate_state_dict(sd, schema=schema)
        msg = str(ctx.exception)
        self.assertIn("key mismatch", msg.lower())
        self.assertIn("buf_x", msg)

    def test_extra_buffer_names_specific_buffer(self):
        sd = {
            "a": {"shape": [2], "dtype": "float32", "data": [1.0, 2.0]},
            "buf_extra": {"shape": [1], "dtype": "float32", "data": [3.0]},
        }
        schema = {
            "version": "v7-test",
            "parameters": {"a": {"shape": [2], "dtype": "float32"}},
            "buffers": [],
        }
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.validate_state_dict(sd, schema=schema)
        msg = str(ctx.exception)
        self.assertIn("key mismatch", msg.lower())
        self.assertIn("buf_extra", msg)

    def test_mandatory_default_schema_rejects_arbitrary_keys(self):
        sd = {"foo": {"shape": [1], "dtype": "float32", "data": [1.0]}}
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.validate_state_dict(sd)
        msg = str(ctx.exception)
        self.assertIn("key mismatch", msg.lower())
        self.assertIn("foo", msg)


class TestElementCount(unittest.TestCase):
    def test_writer_element_count_mismatch(self):
        sd = {"k": {"shape": [2], "dtype": "float32", "data": [1.0]}}
        td = Path(tempfile.mkdtemp())
        try:
            out = td / "x.safetensors"
            with self.assertRaises(conv.ConverterError) as ctx:
                conv.write_safetensors(out, sd)
            msg = str(ctx.exception)
            self.assertIn("numel mismatch", msg.lower())
            self.assertIn("k", msg)
        finally:
            import shutil
            shutil.rmtree(td, ignore_errors=True)


class TestSafetensorsReadback(unittest.TestCase):
    def test_readback_shape_dtype_bytes(self):
        sd = {
            "w": {"shape": [2, 3], "dtype": "float32", "data": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]},
            "b": {"shape": [2], "dtype": "float32", "data": [0.1, 0.2]},
        }
        td = Path(tempfile.mkdtemp())
        try:
            out = td / "out.safetensors"
            conv.write_safetensors(out, sd)
            # Independent parse: 8-byte LE header length + JSON header + data
            raw = out.read_bytes()
            import struct as _s
            (hlen,) = _s.unpack("<Q", raw[:8])
            header = json.loads(raw[8:8 + hlen].decode("utf-8"))
            data = raw[8 + hlen:]
            # account for header padding to multiple of 8
            # safetensors spec: header length covers JSON only; data starts after padded header
            # Our writer pads header_json to multiple of 8 with spaces BEFORE writing length? 
            # Check: length stored is len(header_json) after padding? Let's handle both: try direct.
            # Re-derive: writer writes struct.pack("<Q", len(header_json)) where header_json already padded.
            # So hlen includes padding spaces. json.loads tolerates trailing spaces.
            for name, info in sd.items():
                h = header[name]
                self.assertEqual(h["dtype"], "F32")
                self.assertEqual(h["shape"], info["shape"])
                start, end = h["data_offsets"]
                self.assertEqual(end - start, len(info["data"]) * 4)
                # byte length check against payload slice
                chunk = data[start:end]
                self.assertEqual(len(chunk), (end - start))
        finally:
            import shutil
            shutil.rmtree(td, ignore_errors=True)


class TestWriterDtypeRefusal(unittest.TestCase):
    def test_non_f32_refused(self):
        sd = {"k": {"shape": [2], "dtype": "int64", "data": [1, 2]}}
        td = Path(tempfile.mkdtemp())
        try:
            out = td / "x.safetensors"
            with self.assertRaises(conv.ConverterError) as ctx:
                conv.write_safetensors(out, sd)
            msg = str(ctx.exception)
            self.assertIn("unsupported dtype", msg.lower())
            self.assertIn("k", msg)
        finally:
            import shutil
            shutil.rmtree(td, ignore_errors=True)


class TestValidatorDiscrimination(unittest.TestCase):
    """Validator discrimination tests (not compatibility proof)."""

    def _tiny_schema(self):
        # Hand-written tiny key list derived by reading upstream source:
        # one linear (weight+bias) + one rotary freqs per attention.
        return {
            "version": "tiny-derived",
            "parameters": {
                "layers.0.0.layers.0.0.to_qkv.weight": {"shape": [6, 2], "dtype": "float32"},
                "layers.0.0.layers.0.0.to_qkv.bias": {"shape": [6], "dtype": "float32"},
                "layers.0.0.layers.0.0.rotary_embed.freqs": {"shape": [2], "dtype": "float32"},
            },
            "buffers": [],
        }

    def _valid_sd(self):
        return {
            "layers.0.0.layers.0.0.to_qkv.weight": {"shape": [6, 2], "dtype": "float32", "data": [0.0] * 12},
            "layers.0.0.layers.0.0.to_qkv.bias": {"shape": [6], "dtype": "float32", "data": [0.0] * 6},
            "layers.0.0.layers.0.0.rotary_embed.freqs": {"shape": [2], "dtype": "float32", "data": [1.0, 2.0]},
        }

    def test_valid_passes(self):
        conv.validate_state_dict(self._valid_sd(), schema=self._tiny_schema())

    def test_missing_key(self):
        sd = self._valid_sd()
        del sd["layers.0.0.layers.0.0.to_qkv.bias"]
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.validate_state_dict(sd, schema=self._tiny_schema())
        self.assertIn("missing", str(ctx.exception).lower())

    def test_extra_key(self):
        sd = self._valid_sd()
        sd["extra"] = {"shape": [1], "dtype": "float32", "data": [1.0]}
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.validate_state_dict(sd, schema=self._tiny_schema())
        self.assertIn("unexpected", str(ctx.exception).lower())

    def test_wrong_shape(self):
        sd = self._valid_sd()
        sd["layers.0.0.layers.0.0.to_qkv.weight"] = {"shape": [5, 2], "dtype": "float32", "data": [0.0] * 10}
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.validate_state_dict(sd, schema=self._tiny_schema())
        self.assertIn("shape mismatch", str(ctx.exception).lower())

    def test_wrong_dtype(self):
        sd = self._valid_sd()
        sd["layers.0.0.layers.0.0.to_qkv.weight"] = {"shape": [6, 2], "dtype": "int64", "data": [0] * 12}
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.validate_state_dict(sd, schema=self._tiny_schema())
        self.assertIn("dtype mismatch", str(ctx.exception).lower())

    def test_wrong_depth_per_layer_count(self):
        # depth=2 would add an extra layer key -> unexpected
        sd = self._valid_sd()
        sd["layers.0.0.layers.1.0.to_qkv.weight"] = {"shape": [6, 2], "dtype": "float32", "data": [0.0] * 12}
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.validate_state_dict(sd, schema=self._tiny_schema())
        self.assertIn("unexpected", str(ctx.exception).lower())

    def test_wrong_rotary_count(self):
        sd = self._valid_sd()
        # add a second rotary key
        sd["layers.0.0.layers.0.1.rotary_embed.freqs"] = {"shape": [2], "dtype": "float32", "data": [1.0, 2.0]}
        with self.assertRaises(conv.ConverterError) as ctx:
            conv.validate_state_dict(sd, schema=self._tiny_schema())
        self.assertIn("unexpected", str(ctx.exception).lower())


class TestContiguousSafetensors(unittest.TestCase):
    def test_no_per_tensor_padding(self):
        sd = {
            "a": {"shape": [1], "dtype": "float32", "data": [1.0]},
            "b": {"shape": [3], "dtype": "float32", "data": [2.0, 3.0, 4.0]},
        }
        td = Path(tempfile.mkdtemp())
        try:
            out = td / "out.safetensors"
            conv.write_safetensors(out, sd)
            raw = out.read_bytes()
            import struct as _s
            (hlen,) = _s.unpack("<Q", raw[:8])
            header = json.loads(raw[8:8 + hlen].decode("utf-8"))
            # contiguous: b starts exactly where a ends
            a_start, a_end = header["a"]["data_offsets"]
            b_start, b_end = header["b"]["data_offsets"]
            self.assertEqual(a_start, 0)
            self.assertEqual(a_end, 4)
            self.assertEqual(b_start, 4)
            self.assertEqual(b_end, 16)
            # no holes
            self.assertEqual(b_start - a_end, 0)
        finally:
            import shutil
            shutil.rmtree(td, ignore_errors=True)


class TestSchemaReport(unittest.TestCase):
    def test_schema_report_diff_format(self):
        sd = {
            "a": {"shape": [1], "dtype": "float32", "data": [1.0]},
        }
        report = conv.schema_report(sd, schema={
            "version": "tiny",
            "parameters": {"a": {"shape": [1], "dtype": "float32"}, "b": {"shape": [2], "dtype": "float32"}},
            "buffers": [],
        })
        self.assertFalse(report["match"])
        self.assertIn("b", report["missing"])
        self.assertEqual(report["actual_count"], 1)
        self.assertIn("rotary", report)

class TestRotaryBitwise(unittest.TestCase):
    def _schema(self):
        return {
            "version": "tiny",
            "parameters": {
                "r": {"shape": [2], "dtype": "float32"},
            },
            "buffers": [],
        }

    def test_identical_rotary_values_match(self):
        sd = {
            "r": {"shape": [2], "dtype": "float32", "data": [1.0, 2.0]},
        }
        # single rotary key -> count matches (expected 1), bitwise trivially true
        report = conv.schema_report(sd, schema={
            "version": "tiny",
            "parameters": {"r": {"shape": [2], "dtype": "float32"}},
            "buffers": [],
        })
        # no rotary keys ending in rotary_embed.freqs, so match depends on keys/shapes
        self.assertTrue(report["match"])

    def test_rotary_value_mismatch_match_false(self):
        sd = {
            "layers.0.0.layers.0.0.rotary_embed.freqs": {"shape": [2], "dtype": "float32", "data": [1.0, 2.0]},
            "layers.0.0.layers.0.1.rotary_embed.freqs": {"shape": [2], "dtype": "float32", "data": [1.0, 3.0]},
        }
        schema = {
            "version": "tiny",
            "parameters": {
                "layers.0.0.layers.0.0.rotary_embed.freqs": {"shape": [2], "dtype": "float32"},
                "layers.0.0.layers.0.1.rotary_embed.freqs": {"shape": [2], "dtype": "float32"},
            },
            "buffers": [],
        }
        report = conv.schema_report(sd, schema=schema)
        self.assertFalse(report["match"])
        self.assertFalse(report["rotary"]["bitwise_identical"])

    def test_negative_zero_vs_positive_zero_detected(self):
        sd = {
            "layers.0.0.layers.0.0.rotary_embed.freqs": {"shape": [1], "dtype": "float32", "data": [-0.0]},
            "layers.0.0.layers.0.1.rotary_embed.freqs": {"shape": [1], "dtype": "float32", "data": [0.0]},
        }
        schema = {
            "version": "tiny",
            "parameters": {
                "layers.0.0.layers.0.0.rotary_embed.freqs": {"shape": [1], "dtype": "float32"},
                "layers.0.0.layers.0.1.rotary_embed.freqs": {"shape": [1], "dtype": "float32"},
            },
            "buffers": [],
        }
        report = conv.schema_report(sd, schema=schema)
        self.assertFalse(report["match"])
        self.assertFalse(report["rotary"]["bitwise_identical"])
        # validate_state_dict must also raise
        with self.assertRaises(conv.ConverterError):
            conv.validate_state_dict(sd, schema=schema)



class TestSchemaReportMutations(unittest.TestCase):
    """One test per mutation for the fail-closed schema report."""

    def _schema(self):
        return {
            "version": "tiny",
            "parameters": {
                "a": {"shape": [2], "dtype": "float32"},
                "b": {"shape": [2], "dtype": "float32"},
                "layers.0.0.layers.0.0.rotary_embed.freqs": {"shape": [2], "dtype": "float32"},
                "layers.0.0.layers.0.1.rotary_embed.freqs": {"shape": [2], "dtype": "float32"},
            },
            "buffers": [],
        }

    def _base(self):
        return {
            "a": {"shape": [2], "dtype": "float32", "data": [1.0, 2.0]},
            "b": {"shape": [2], "dtype": "float32", "data": [3.0, 4.0]},
            "layers.0.0.layers.0.0.rotary_embed.freqs": {"shape": [2], "dtype": "float32", "data": [1.0, 2.0]},
            "layers.0.0.layers.0.1.rotary_embed.freqs": {"shape": [2], "dtype": "float32", "data": [1.0, 2.0]},
        }

    def test_positive_control_complete_data(self):
        r = conv.schema_report(self._base(), schema=self._schema())
        self.assertTrue(r["match"])
        self.assertIsNone(r["reason"])
        self.assertTrue(r["rotary"]["bitwise_identical"])

    def test_dtype_missing_one_tensor(self):
        sd = self._base()
        sd["a"] = {"shape": [2], "data": [1.0, 2.0]}  # no dtype
        r = conv.schema_report(sd, schema=self._schema())
        self.assertFalse(r["match"])
        self.assertEqual(r["reason"], "missing_dtype")

    def test_dtype_missing_all(self):
        sd = self._base()
        for k in sd:
            del sd[k]["dtype"]
        r = conv.schema_report(sd, schema=self._schema())
        self.assertFalse(r["match"])
        self.assertEqual(r["reason"], "missing_dtype")

    def test_rotary_absent(self):
        sd = self._base()
        del sd["layers.0.0.layers.0.1.rotary_embed.freqs"]
        r = conv.schema_report(sd, schema=self._schema())
        self.assertFalse(r["match"])
        self.assertEqual(r["reason"], "exact_key_set")

    def test_rotary_nan(self):
        sd = self._base()
        sd["layers.0.0.layers.0.1.rotary_embed.freqs"] = {"shape": [2], "dtype": "float32", "data": [float("nan"), 2.0]}
        r = conv.schema_report(sd, schema=self._schema())
        self.assertFalse(r["match"])
        # nan payload differs by bits from 1.0/2.0 -> bitwise mismatch
        self.assertEqual(r["reason"], "rotary_bitwise_mismatch")

    def test_negative_zero_vs_positive_zero(self):
        sd = self._base()
        sd["layers.0.0.layers.0.1.rotary_embed.freqs"] = {"shape": [2], "dtype": "float32", "data": [-0.0, 2.0]}
        r = conv.schema_report(sd, schema=self._schema())
        self.assertFalse(r["match"])
        self.assertEqual(r["reason"], "rotary_bitwise_mismatch")

    def test_extra_key(self):
        sd = self._base()
        sd["extra"] = {"shape": [1], "dtype": "float32", "data": [1.0]}
        r = conv.schema_report(sd, schema=self._schema())
        self.assertFalse(r["match"])
        self.assertEqual(r["reason"], "exact_key_set")

    def test_missing_key(self):
        sd = self._base()
        del sd["b"]
        r = conv.schema_report(sd, schema=self._schema())
        self.assertFalse(r["match"])
        self.assertEqual(r["reason"], "exact_key_set")

    def test_wrong_dtype(self):
        sd = self._base()
        sd["a"] = {"shape": [2], "dtype": "int64", "data": [1, 2]}
        r = conv.schema_report(sd, schema=self._schema())
        self.assertFalse(r["match"])
        self.assertEqual(r["reason"], "dtype_mismatch")

    def test_key_shape_only_never_match_true(self):
        sd = {
            "a": {"shape": [2]},
            "b": {"shape": [2]},
            "layers.0.0.layers.0.0.rotary_embed.freqs": {"shape": [2]},
            "layers.0.0.layers.0.1.rotary_embed.freqs": {"shape": [2]},
        }
        r = conv.schema_report(sd, schema=self._schema())
        self.assertFalse(r["match"])
        # data missing or dtype missing must be the reason
        self.assertIn(r["reason"], ("missing_dtype", "data_missing"))

    def test_rotary_branch_exercised(self):
        # ensure rotary keys actually end with rotary_embed.freqs in schema
        r = conv.schema_report(self._base(), schema=self._schema())
        self.assertEqual(r["rotary"]["count"], 2)
        self.assertEqual(r["rotary"]["expected_count"], 2)


class TestSharedValidation(unittest.TestCase):
    def _schema(self):
        return {"version": "tiny", "parameters": {"t": {"shape": [2], "dtype": "float32"}}, "buffers": []}

    def _writer_accepts(self, sd):
        import tempfile
        from pathlib import Path
        td = Path(tempfile.mkdtemp())
        try:
            try:
                conv.write_safetensors(td / "out.safetensors", sd)
                return True
            except conv.ConverterError:
                return False
        finally:
            import shutil
            shutil.rmtree(td, ignore_errors=True)

    def test_data_empty_match_false_nonzero(self):
        sd = {"t": {"shape": [2], "dtype": "float32", "data": []}}
        r = conv.schema_report(sd, schema=self._schema())
        self.assertFalse(r["match"])
        self.assertIsNotNone(r["reason"])
        # reason names tensor and counts
        self.assertIn("t", r["reason"])
        self.assertIn("2", r["reason"])
        self.assertIn("0", r["reason"])

    def test_one_value_short_rejected(self):
        sd = {"t": {"shape": [2], "dtype": "float32", "data": [1.0]}}
        self.assertFalse(self._writer_accepts(sd))
        r = conv.schema_report(sd, schema=self._schema())
        self.assertFalse(r["match"])

    def test_one_value_over_rejected(self):
        sd = {"t": {"shape": [2], "dtype": "float32", "data": [1.0, 2.0, 3.0]}}
        self.assertFalse(self._writer_accepts(sd))
        r = conv.schema_report(sd, schema=self._schema())
        self.assertFalse(r["match"])

    def test_scalar_shape_one_value_passes(self):
        schema = {"version": "tiny", "parameters": {"t": {"shape": [], "dtype": "float32"}}, "buffers": []}
        sd = {"t": {"shape": [], "dtype": "float32", "data": [1.0]}}
        self.assertTrue(self._writer_accepts(sd))
        r = conv.schema_report(sd, schema=schema)
        self.assertTrue(r["match"])

    def test_bool_shape_rejected(self):
        sd = {"t": {"shape": [True], "dtype": "float32", "data": [1.0]}}
        self.assertFalse(self._writer_accepts(sd))
        r = conv.schema_report(sd, schema={"version": "t", "parameters": {"t": {"shape": [1], "dtype": "float32"}}, "buffers": []})
        self.assertFalse(r["match"])

    def test_negative_shape_rejected(self):
        sd = {"t": {"shape": [-1], "dtype": "float32", "data": [1.0]}}
        self.assertFalse(self._writer_accepts(sd))

    def test_non_integer_shape_rejected(self):
        sd = {"t": {"shape": [2.0], "dtype": "float32", "data": [1.0, 2.0]}}
        self.assertFalse(self._writer_accepts(sd))
        r = conv.schema_report(sd, schema=self._schema())
        self.assertFalse(r["match"])

    def test_differential_table(self):
        schema = self._schema()
        cases = [
            ({"t": {"shape": [2], "dtype": "float32", "data": [1.0, 2.0]}}, False),
            ({"t": {"shape": [2], "dtype": "float32", "data": [1.0]}}, True),
            ({"t": {"shape": [2], "dtype": "float32", "data": [1.0, 2.0, 3.0]}}, True),
            ({"t": {"shape": [2], "dtype": "int64", "data": [1, 2]}}, True),
            ({"t": {"shape": [2], "data": [1.0, 2.0]}}, True),
            ({"t": {"shape": [], "dtype": "float32", "data": [1.0]}}, False),
            # nested data (canonical flatten) -- must agree between writer and report
            ({"t": {"shape": [2], "dtype": "float32", "data": [[1.0], [2.0]]}}, False),
            ({"t": {"shape": [2], "dtype": "float32", "data": [[1.0, 2.0, 3.0]]}}, True),
        ]
        for sd, expect_reject in cases:
            writer_rejects = not self._writer_accepts(sd)
            report_match = conv.schema_report(sd, schema=schema if sd["t"]["shape"] != [] else {"version": "tiny", "parameters": {"t": {"shape": [], "dtype": "float32"}}, "buffers": []})["match"]
            self.assertEqual(writer_rejects, (not report_match), f"mismatch on {sd}: writer_rejects={writer_rejects} report_match={report_match}")


if __name__ == "__main__":
    unittest.main()
