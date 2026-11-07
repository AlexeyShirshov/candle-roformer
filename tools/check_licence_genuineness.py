#!/usr/bin/env python3
"""Re-derive licence evidence files from the local cargo registry and compare."""
import hashlib
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EV = ROOT / "docs" / "dependency-audit-evidence"
INDEX = EV / "s1-licence-audit.txt"


def find_source_dir(crate, version):
    roots = [Path.home() / ".cargo" / "registry" / "src"]
    roots += list((Path.home() / ".cargo" / "git" / "checkouts").glob("*"))
    for root in roots:
        if not root.exists():
            continue
        for d in root.rglob(f"{crate}-{version}"):
            if d.is_dir():
                return d
    return None


def main():
    byte_genuine = 0
    metadata_only = 0
    unverified_offline = 0
    mismatch = 0
    missing = 0
    details = []

    rows = [ln for ln in INDEX.read_text(encoding="utf-8").splitlines() if ln.strip() and not ln.startswith("crate\t") and not ln.startswith("#")]
    for row in rows:
        parts = row.split("\t")
        if len(parts) < 4:
            missing += 1
            details.append(f"{row}: malformed index row")
            continue
        crate, version, license_expr, evidence = parts[0], parts[1], parts[2], parts[3]
        license_expr = license_expr.strip()

        if evidence.strip() in ("metadata_only", "unverified_offline"):
            if evidence.strip() == "metadata_only":
                if not license_expr:
                    missing += 1
                    details.append(f"{crate} {version}: metadata_only without expression")
                    continue
                metadata_only += 1
            else:
                unverified_offline += 1
            # no fabricated text possible (no evidence file)
            continue

        ev_path = ROOT / "docs" / evidence
        if not ev_path.is_file():
            missing += 1
            details.append(f"{crate} {version}: missing evidence file {evidence}")
            continue

        # fabricated-text guard for any present file
        raw = ev_path.read_bytes()
        text = raw.decode("utf-8", errors="replace")
        if "provenance padding" in text:
            mismatch += 1
            details.append(f"{crate} {version}: fabricated padding text")
            continue

        src_dir = None
        m_src = re.search(r"source_dir:\s*(.+)", text)
        if m_src:
            src_dir = Path(m_src.group(1).strip())
        if not src_dir or not src_dir.exists():
            src_dir = find_source_dir(crate, version)

        # 1) whole-file byte-identical copy of a licence-like source file
        genuine = False
        if src_dir and src_dir.exists():
            for lf in src_dir.rglob("*"):
                if lf.is_file() and lf.name.upper().split(".")[0] in {"LICENSE", "COPYING", "NOTICE", "UNLICENSE"} or lf.name.upper().startswith(("LICENSE", "COPYING", "NOTICE", "UNLICENSE")):
                    try:
                        if lf.read_bytes() == raw:
                            genuine = True
                            break
                    except OSError:
                        pass

        # 2) excerpt-level composite (legacy 153)
        if not genuine and "--- license file excerpt" in text:
            m_lf = re.search(r"license_file:\s*(.+)", text)
            lf_path = None
            if m_lf:
                cand = m_lf.group(1).strip()
                if cand and cand != "none found":
                    lf_path = Path(cand)
            if lf_path and lf_path.is_file():
                idx = text.find("--- license file excerpt")
                excerpt = text[idx:].splitlines()[1:]
                while excerpt and excerpt[-1].strip() == "":
                    excerpt.pop()
                src_lines = lf_path.read_text(encoding="utf-8", errors="replace").splitlines()
                if excerpt == src_lines[:len(excerpt)]:
                    genuine = True

        if genuine:
            byte_genuine += 1
        else:
            mismatch += 1
            details.append(f"{crate} {version}: excerpt/byte mismatch")

    print(f"byte_genuine={byte_genuine} metadata_only={metadata_only} unverified_offline={unverified_offline} mismatch={mismatch} missing={missing}")
    for d in details:
        print(d)

    # assertions
    assert mismatch == 0, f"{mismatch} mismatches"
    assert missing == 0, f"{missing} missing"
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
