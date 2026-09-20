import copy
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("followup_gate", ROOT / "scripts/core_conformance_gate.py")
GATE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GATE)


class GateFollowupTests(unittest.TestCase):
    def test_nested_bad_shapes_retain_structured_cli_failure(self):
        original = json.loads((ROOT / "docs/core-01-conformance.json").read_text())
        cases = []
        for field in ["evidence_class", "step"]:
            value = copy.deepcopy(original)
            value["invariants"][0]["evidence"][0][field] = []
            cases.append(value)
        value = copy.deepcopy(original)
        value["invariants"][0]["evidence"][1]["cargo_target"]["kind"] = {}
        cases.append(value)
        with tempfile.TemporaryDirectory() as directory:
            directory = Path(directory)
            inventory = directory / "inventory"
            inventory.mkdir()
            (inventory / "p--lib.txt").write_text("case: test\n1 test, 0 benchmarks\n")
            (inventory / "p--lib.ignored").write_text("0 tests, 0 benchmarks\n")
            for index, value in enumerate(cases):
                with self.subTest(index=index):
                    manifest = directory / "manifest.json"
                    manifest.write_text(json.dumps(value))
                    report = directory / f"report-{index}.json"
                    process = subprocess.run(
                        [sys.executable, str(ROOT / "scripts/core_conformance_gate.py"),
                         "--manifest", str(manifest), "--platform", "ubuntu-24.04",
                         "--commit", "a" * 40, "--test-inventory", str(inventory),
                         "--output", str(report)], cwd=ROOT, capture_output=True,
                        text=True, timeout=10, check=False,
                        env={**os.environ, "CORE_STEP_RESULTS": "{}"},
                    )
                    self.assertEqual(process.returncode, 1)
                    self.assertNotIn("Traceback", process.stderr)
                    self.assertTrue(report.is_file(), process.stderr)
                    self.assertFalse(json.loads(report.read_text())["checks_passed"])

    def test_missing_or_contradictory_ignored_inventory_fails_closed(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "p--lib.txt").write_text("case: test\n1 test, 0 benchmarks\n")
            with self.assertRaises(GATE.GateError):
                GATE.parse_test_inventory(root)
            (root / "p--lib.ignored").write_text("other: test\n1 test, 0 benchmarks\n")
            with self.assertRaises(GATE.GateError):
                GATE.parse_test_inventory(root)

    @unittest.skipUnless(shutil.which("rustc"), "requires Rust to verify effective ignores")
    def test_real_harness_ignored_forms_are_never_credited(self):
        source = r'''#[test] fn active() {}
#[ignore]
#[test]
fn bare() {}
#[ignore = "reason"]
#[test]
fn reason() {}
#[cfg_attr(all(), ignore)]
#[test]
fn conditional() {}
#[cfg_attr(all(), cfg_attr(all(), ignore = "reason"))]
#[test]
fn nested() {}
#[cfg_attr(
    all(),
    ignore = "multiline reason"
)]
#[test]
fn multiline() {}
'''
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / "cases.rs"
            path.write_text(source)
            binary = root / ("cases.exe" if os.name == "nt" else "cases")
            subprocess.run(["rustc", "--test", str(path), "-o", str(binary)],
                           check=True, capture_output=True, text=True, timeout=30)
            inventory = root / "inventory"
            inventory.mkdir()
            for suffix, flags in [("txt", []), ("ignored", ["--ignored"])]:
                process = subprocess.run([str(binary), "--list", *flags],
                                         capture_output=True, text=True, check=True, timeout=10)
                (inventory / f"p--lib.{suffix}").write_text(process.stdout)
            self.assertEqual(GATE.parse_test_inventory(inventory), {"p--lib": {"active"}})
            process = subprocess.run([str(binary)], capture_output=True, text=True,
                                     check=True, timeout=10)
            self.assertIn("1 passed; 0 failed; 5 ignored", process.stdout)

    def test_inventory_name_cannot_point_to_another_source_function(self):
        manifest = json.loads((ROOT / "docs/core-01-conformance.json").read_text())
        manifest["invariants"][0]["evidence"][1]["inventory_name"] = "unrelated::other"
        with self.assertRaises(GATE.GateError):
            GATE.validate_manifest(manifest, ROOT)

    def test_truncated_or_unrecognized_ignored_listing_fails_closed(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "p--lib.txt").write_text("case: test\n1 test, 0 benchmarks\n")
            for listing in ["", "error\n", "case: test\n", "case: test\n0 tests, 0 benchmarks\n"]:
                (root / "p--lib.ignored").write_text(listing)
                with self.subTest(listing=listing), self.assertRaises(GATE.GateError):
                    GATE.parse_test_inventory(root)
