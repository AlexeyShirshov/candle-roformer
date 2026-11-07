"""Docs evidence reproducibility check (S1-F10)."""
import hashlib
import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def parse_cargo_lock(text):
    blocks = re.findall(r"\[\[package\]\]\n(.*?)(?=\n\[\[|\Z)", text, re.S)
    out = []
    for b in blocks:
        mn = re.search(r'^name = "([^"]+)"', b, re.M)
        mv = re.search(r'^version = "([^"]+)"', b, re.M)
        ms = re.search(r'^source = "([^"]+)"', b, re.M)
        if not mn or not mv:
            continue
        out.append((mn.group(1), mv.group(1), ms.group(1) if ms else ""))
    return out


def parse_index(text):
    rows = []
    for ln in text.splitlines():
        if not ln.strip() or ln.startswith("#") or ln.startswith("crate\t"):
            continue
        parts = ln.split("\t")
        if len(parts) < 5:
            continue
        rows.append((parts[0], parts[1], parts[4]))
    return rows


def lock_third_party(lock_text):
    return [e for e in parse_cargo_lock(lock_text) if e[2]]


def cargo_lock_sha256():
    return hashlib.sha256((ROOT / "Cargo.lock").read_bytes()).hexdigest()


class TestDocsEvidence(unittest.TestCase):
    def _lock_text(self):
        return (ROOT / "Cargo.lock").read_text(encoding="utf-8")

    def _index_text(self):
        return (ROOT / "docs" / "dependency-audit-evidence" / "s1-licence-audit.txt").read_text(encoding="utf-8")

    def test_dependency_audit_cited_paths_exist(self):
        p = ROOT / "docs" / "dependency-audit.md"
        self.assertTrue(p.is_file())
        text = p.read_text(encoding="utf-8")
        for ref in re.findall(r"`([^`]+)`", text):
            if "/" in ref and not ref.startswith("http"):
                if " " in ref:
                    continue
                if not ref.startswith(("docs/", ".pdca/", "tools/", "src/", "tests/", "Cargo")):
                    continue
                cand = ROOT / ref
                if cand.exists():
                    continue
                self.assertTrue(cand.exists(), f"missing cited path: {ref}")

    def test_gates_report_cited_paths_exist(self):
        p = ROOT / "docs" / "gates-report.md"
        self.assertTrue(p.is_file())
        text = p.read_text(encoding="utf-8")
        for ref in re.findall(r"`([^`]+)`", text):
            if "/" in ref and not ref.startswith("http"):
                if " " in ref:
                    continue
                if not ref.startswith(("docs/", ".pdca/", "tools/", "src/", "tests/", "Cargo")):
                    continue
                cand = ROOT / ref
                if ref.endswith((".md", ".py", ".txt", ".log", ".json", ".rs", ".toml", ".yaml", ".ckpt")):
                    self.assertTrue(cand.exists(), f"missing cited path: {ref}")

    def test_index_set_equality_with_lock(self):
        lock_entries = lock_third_party(self._lock_text())
        index_entries = parse_index(self._index_text())
        lock_set = set(lock_entries)
        index_set = set(index_entries)
        self.assertEqual(lock_set - index_set, set(), f"missing from index: {sorted(lock_set - index_set)}")
        self.assertEqual(index_set - lock_set, set(), f"stale in index: {sorted(index_set - lock_set)}")
        self.assertEqual(len(lock_entries), len(set(lock_entries)), "duplicate lock keys")
        self.assertEqual(len(index_entries), len(set(index_entries)), "duplicate index keys")

    def test_index_has_source_column_and_sha_header(self):
        text = self._index_text()
        self.assertTrue(any(ln.startswith("# cargo_lock_sha256:") for ln in text.splitlines()))
        header = [ln for ln in text.splitlines() if ln.startswith("crate\t")][0]
        self.assertIn("source", header.split("\t"))
        sha_line = [ln for ln in text.splitlines() if ln.startswith("# cargo_lock_sha256:")][0]
        recorded = sha_line.split(":", 1)[1].strip()
        self.assertEqual(recorded, cargo_lock_sha256())

    def test_evidence_dir_has_one_file_per_verified_row(self):
        d = ROOT / "docs" / "dependency-audit-evidence"
        self.assertTrue(d.is_dir())
        files = [f for f in d.iterdir() if f.suffix == ".txt" and f.name != "s1-licence-audit.txt"]
        rows = [ln for ln in self._index_text().splitlines() if ln.strip() and not ln.startswith("#") and not ln.startswith("crate\t")]
        verified = [r for r in rows if r.split("\t")[3].strip() not in ("metadata_only", "unverified_offline")]
        self.assertEqual(len(files), len(verified))

    def test_metadata_only_crates_explicit(self):
        allowed = {
            "block2 0.6.2",
            "candle-nn 0.11.1",
            "cudaforge 0.1.6",
            "dispatch2 0.3.1",
            "objc2 0.6.5",
            "objc2-core-foundation 0.3.2",
            "objc2-encode 4.1.0",
            "objc2-metal 0.3.2",
            "pulp-wasm-simd-flag 0.1.1",
            "r-efi 5.3.0",
            "r-efi 6.0.0",
            "winapi-i686-pc-windows-gnu 0.4.0",
            "winapi-x86_64-pc-windows-gnu 0.4.0",
        }
        rows = [ln for ln in self._index_text().splitlines() if ln.strip() and not ln.startswith("#") and not ln.startswith("crate\t")]
        metadata_only = set()
        for row in rows:
            parts = row.split("\t")
            if len(parts) >= 4 and parts[3].strip() == "metadata_only":
                metadata_only.add(f"{parts[0]} {parts[1]}")
        self.assertEqual(metadata_only, allowed)

    def test_byte_genuine_absence_fails(self):
        rows = [ln for ln in self._index_text().splitlines() if ln.strip() and not ln.startswith("#") and not ln.startswith("crate\t")]
        missing = []
        for row in rows:
            parts = row.split("\t")
            if len(parts) < 4:
                continue
            ev = parts[3].strip()
            if ev in ("metadata_only", "unverified_offline"):
                continue
            if not (ROOT / "docs" / ev).is_file():
                missing.append(ev)
        self.assertEqual(missing, [], f"missing byte_genuine files: {missing}")

    def test_evidence_files_sha256_recorded(self):
        import json, hashlib
        prov = ROOT / "docs" / "provenance" / "PROVENANCE.json"
        d = json.loads(prov.read_text(encoding="utf-8"))
        pdir = ROOT / "docs" / "provenance"
        files = sorted(f for f in pdir.iterdir() if f.is_file() and f.name != "PROVENANCE.json")
        self.assertGreater(len(files), 0)
        for f in files:
            self.assertIn(f.name, d, f"missing provenance entry for {f.name}")
            h = hashlib.sha256(f.read_bytes()).hexdigest()
            self.assertEqual(len(d[f.name]["sha256"]), 64)
            self.assertEqual(d[f.name]["sha256"], h, f"sha256 mismatch for {f.name}")
            self.assertEqual(d[f.name].get("sha256_local", d[f.name]["sha256"]), h)

    def test_gates_report_g3_silence_threshold_literal(self):
        p = ROOT / "docs" / "gates-report.md"
        text = p.read_text(encoding="utf-8")
        self.assertIn("max_abs <= 1e-3", text)
        self.assertIn("1e-3", text)

    def test_settings_excerpt_removed(self):
        p = ROOT / "docs" / "provenance" / "settings_excerpt.py"
        self.assertFalse(p.exists(), "fabricated settings_excerpt.py must be removed")

    def test_provenance_entries_classified(self):
        import json
        p = ROOT / "docs" / "provenance" / "PROVENANCE.json"
        d = json.loads(p.read_text(encoding="utf-8"))
        self.assertNotIn("settings_excerpt.py", d)
        for name, info in d.items():
            self.assertIn("class", info)
            self.assertIn(info["class"], {"verbatim", "authored", "unverified_offline", "derived"})
            self.assertIn("verification", info)
            self.assertIn(info["verification"], {"unverified_offline", "verified_factory"})
            if info["class"] in {"verbatim", "derived"}:
                self.assertIn("sha256", info)
                f = ROOT / "docs" / "provenance" / name
                self.assertTrue(f.is_file(), f"missing provenance file: {name}")
                import hashlib
                h = hashlib.sha256(f.read_bytes()).hexdigest()
                self.assertEqual(info["sha256"], h)
            self.assertNotIn("excerpt", info.get("path", "").lower())
            self.assertNotIn("line", info.get("path", "").lower())

    def test_no_padding_comments_in_provenance(self):
        d = ROOT / "docs" / "provenance"
        for f in d.iterdir():
            if f.suffix == ".py":
                text = f.read_text(encoding="utf-8", errors="replace")
                self.assertNotIn("provenance padding", text)

    # Synthetic lock tests
    def test_synthetic_lock_drop_row_bites(self):
        lock_text = self._lock_text()
        index_text = self._index_text()
        blocks = re.findall(r"\[\[package\]\]\n.*?(?=\n\[\[|\Z)", lock_text, re.S)
        self.assertGreater(len(blocks), 0)
        modified = lock_text.replace(blocks[0], "", 1)
        lock_entries = [e for e in parse_cargo_lock(modified) if e[2]]
        index_entries = parse_index(index_text)
        self.assertNotEqual(set(lock_entries), set(index_entries))

    def test_synthetic_lock_add_stale_row_bites(self):
        lock_text = self._lock_text()
        index_text = self._index_text()
        modified_index = index_text + "\nstale-crate\t9.9.9\tMIT\tmetadata_only\tregistry+https://example.com\n"
        index_entries = parse_index(modified_index)
        lock_entries = lock_third_party(lock_text)
        self.assertNotEqual(set(lock_entries), set(index_entries))

    def test_synthetic_lock_change_version_bites(self):
        lock_text = self._lock_text()
        index_text = self._index_text()
        modified = lock_text.replace('name = "anyhow"\nversion = "1.0.104"', 'name = "anyhow"\nversion = "9.9.9"', 1)
        lock_entries = [e for e in parse_cargo_lock(modified) if e[2]]
        index_entries = parse_index(index_text)
        self.assertNotEqual(set(lock_entries), set(index_entries))


if __name__ == "__main__":
    unittest.main()
