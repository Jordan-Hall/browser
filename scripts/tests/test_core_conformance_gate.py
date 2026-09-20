import copy
import importlib.util
import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "core_conformance_gate", ROOT / "scripts/core_conformance_gate.py"
)
assert SPEC and SPEC.loader
GATE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GATE)


class CoreConformanceGateTests(unittest.TestCase):
    def setUp(self):
        self.manifest = json.loads((ROOT / "docs/core-01-conformance.json").read_text())
        self.steps = {
            "tests": {"outcome": "success"},
            "doctests": {"outcome": "success"},
            "conformance": {"outcome": "success"},
            "architecture": {"outcome": "success"},
            "fuzz_compile": {"outcome": "success"},
            "test_inventory": {"outcome": "success"},
        }
        self.commit = "a" * 40
        self.inventory = {}
        for invariant in self.manifest["invariants"]:
            for item in invariant["evidence"]:
                if item["evidence_class"] not in {"unit", "subprocess"}:
                    continue
                target = GATE._cargo_target(item, item["id"])
                key = GATE._target_inventory_id(target)
                self.inventory.setdefault(key, set()).add(item["inventory_name"])

    def test_current_manifest_is_complete_and_success_is_revision_bound(self):
        report, passed = GATE.build_report(
            self.manifest, ROOT, "ubuntu-24.04", self.commit, self.steps, self.inventory
        )
        self.assertTrue(passed)
        self.assertTrue(report["checks_passed"])
        self.assertEqual(len(report["invariants"]), 6)
        revisions = {
            evidence["tested_revision"]
            for invariant in report["invariants"]
            for evidence in invariant["evidence"]
            if evidence["result"] != "unsupported"
        }
        self.assertEqual(revisions, {self.commit})
        test_evidence = [
            evidence
            for invariant in report["invariants"]
            for evidence in invariant["evidence"]
            if evidence["test_inventory"] != "not_applicable"
        ]
        self.assertTrue(test_evidence)
        self.assertTrue(all(item["inventory_target"] for item in test_evidence))

    def test_omitting_a_required_invariant_fails_closed(self):
        manifest = copy.deepcopy(self.manifest)
        manifest["invariants"] = manifest["invariants"][1:]
        with self.assertRaises(GATE.GateError):
            GATE.validate_manifest(manifest, ROOT)

    def test_removing_action_hash_or_queue_limit_regression_fails_closed(self):
        for evidence_id in ["CORE-01.ACTION-HASH-BINDING", "CORE-01.RELIABLE-QUEUE-LIMIT"]:
            manifest = copy.deepcopy(self.manifest)
            for invariant in manifest["invariants"]:
                invariant["evidence"] = [
                    item for item in invariant["evidence"] if item["id"] != evidence_id
                ]
            with self.assertRaises(GATE.GateError, msg=evidence_id):
                GATE.validate_manifest(manifest, ROOT)

    def test_missing_test_function_is_not_evidence(self):
        manifest = copy.deepcopy(self.manifest)
        manifest["invariants"][0]["evidence"][0]["test_name"] = "not_a_real_test"
        with self.assertRaises(GATE.GateError):
            GATE.validate_manifest(manifest, ROOT)

    def test_missing_skipped_and_failed_steps_are_not_passes(self):
        for outcome in [None, "skipped", "failure"]:
            steps = copy.deepcopy(self.steps)
            if outcome is None:
                steps.pop("tests")
            else:
                steps["tests"]["outcome"] = outcome
            report, passed = GATE.build_report(
                self.manifest, ROOT, "ubuntu-24.04", self.commit, steps, self.inventory
            )
            self.assertFalse(passed)
            self.assertFalse(report["checks_passed"])

    def test_platform_with_no_complete_required_evidence_cannot_pass(self):
        report, passed = GATE.build_report(
            self.manifest, ROOT, "plan9", self.commit, self.steps, self.inventory
        )
        self.assertFalse(passed)
        self.assertFalse(report["checks_passed"])
        self.assertTrue(
            all(not invariant["passed"] for invariant in report["invariants"])
        )

    def test_non_test_ignored_and_path_escape_evidence_are_rejected(self):
        manifest = copy.deepcopy(self.manifest)
        evidence = manifest["invariants"][0]["evidence"][1]
        evidence["test_name"] = "new"
        with self.assertRaises(GATE.GateError):
            GATE.validate_manifest(manifest, ROOT)

        manifest = copy.deepcopy(self.manifest)
        manifest["invariants"][0]["evidence"][1]["test_file"] = "../Cargo.toml"
        with self.assertRaises(GATE.GateError):
            GATE.validate_manifest(manifest, ROOT)

        manifest = copy.deepcopy(self.manifest)
        del manifest["invariants"][0]["evidence"][1]["inventory_name"]
        with self.assertRaises(GATE.GateError):
            GATE.validate_manifest(manifest, ROOT)

        manifest = copy.deepcopy(self.manifest)
        del manifest["invariants"][0]["evidence"][1]["cargo_target"]
        with self.assertRaises(GATE.GateError):
            GATE.validate_manifest(manifest, ROOT)

    def test_every_ignore_attribute_form_is_rejected(self):
        for attributes in [
            '#[test]\n#[ignore = "reason"]\n',
            '#[test]\n#[cfg_attr(any(), ignore)]\n',
        ]:
            with self.subTest(attributes=attributes), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                (root / "case.rs").write_text(f"{attributes}fn case() {{}}\n")
                manifest = {
                    "schema_version": 1,
                    "parent_issue": 2,
                    "required_invariants": ["AC"],
                    "invariants": [
                        {
                            "id": "AC",
                            "requirement": "test",
                            "required_evidence_ids": ["E"],
                            "evidence": [
                                {
                                    "id": "E",
                                    "evidence_class": "unit",
                                    "step": "tests",
                                    "test_file": "case.rs",
                                    "test_name": "case",
                                    "supported_targets": ["ubuntu-24.04"],
                                    "inventory_name": "case",
                                    "cargo_target": {"package": "p", "kind": "lib"},
                                }
                            ],
                        }
                    ],
                }
                with self.assertRaises(GATE.GateError):
                    GATE.validate_manifest(manifest, root)

    def test_source_function_is_bound_to_declared_cargo_target_and_inventory(self):
        manifest = copy.deepcopy(self.manifest)
        evidence = next(
            item
            for invariant in manifest["invariants"]
            for item in invariant["evidence"]
            if item["id"] == "CORE-01.ACTION-HASH-BINDING"
        )
        evidence["cargo_target"]["package"] = "intent-ipc"
        with self.assertRaises(GATE.GateError):
            GATE.validate_manifest(manifest, ROOT)

        manifest = copy.deepcopy(self.manifest)
        evidence = next(
            item
            for invariant in manifest["invariants"]
            for item in invariant["evidence"]
            if item["id"] == "CORE-01.ACTION-HASH-BINDING"
        )
        evidence["inventory_name"] = "money_rejects_ambiguous_spellings_and_numeric_json"
        with self.assertRaises(GATE.GateError):
            GATE.validate_manifest(manifest, ROOT)

    @unittest.skipIf(os.name == "nt", "symlink creation is not portable on Windows runners")
    def test_repo_file_rejects_symlink_escape(self):
        with tempfile.TemporaryDirectory() as directory:
            outer = Path(directory)
            root = outer / "repo"
            root.mkdir()
            outside = outer / "outside.txt"
            outside.write_text("external")
            (root / "evidence.txt").symlink_to(outside)
            with self.assertRaises(GATE.GateError):
                GATE._repo_file(root, "evidence.txt", "E")

    def test_inconsistent_reused_evidence_id_is_rejected(self):
        manifest = copy.deepcopy(self.manifest)
        manifest["invariants"][-1]["evidence"][0]["step"] = "tests"
        with self.assertRaises(GATE.GateError):
            GATE.validate_manifest(manifest, ROOT)

    def test_aggregate_success_cannot_hide_a_missing_target_inventory_entry(self):
        inventory = {key: set(values) for key, values in self.inventory.items()}
        key = "intent-contracts--test--hardening"
        name = "proposal_construction_and_wire_enforce_the_same_hash_binding"
        inventory[key].remove(name)
        inventory.setdefault("intent-ipc--test--record_codec", set()).add(name)
        report, passed = GATE.build_report(
            self.manifest, ROOT, "ubuntu-24.04", self.commit, self.steps, inventory
        )
        self.assertFalse(passed)
        action = next(
            evidence
            for invariant in report["invariants"]
            for evidence in invariant["evidence"]
            if evidence["id"] == "CORE-01.ACTION-HASH-BINDING"
        )
        self.assertEqual(action["result"], "success")
        self.assertEqual(action["inventory_target"], key)
        self.assertEqual(action["test_inventory"], "missing")
        self.assertFalse(action["passed"])

    def test_inventory_parser_preserves_target_provenance(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            (path / "intent-ipc--lib.txt").write_text("frame::tests::case: test\n")
            (path / "intent-ipc--test--record_codec.txt").write_text("case: test\n")
            parsed = GATE.parse_test_inventory(path)
            self.assertEqual(parsed["intent-ipc--lib"], {"frame::tests::case"})
            self.assertEqual(parsed["intent-ipc--test--record_codec"], {"case"})
            empty = path / "empty.txt"
            empty.write_text("0 tests, 0 benchmarks\n")
            with self.assertRaises(GATE.GateError):
                GATE.parse_test_inventory(path)

    def test_manifest_shape_errors_are_gate_errors(self):
        with self.assertRaises(GATE.GateError):
            GATE.validate_manifest([], ROOT)
        manifest = copy.deepcopy(self.manifest)
        del manifest["invariants"][0]["requirement"]
        with self.assertRaises(GATE.GateError):
            GATE.validate_manifest(manifest, ROOT)

    def _write_inventory(self, path: Path) -> None:
        path.mkdir(parents=True, exist_ok=True)
        for target, names in self.inventory.items():
            (path / f"{target}.txt").write_text(
                "".join(f"{name}: test\n" for name in sorted(names))
            )

    def test_cli_failure_retains_report_and_nonzero_status(self):
        with tempfile.TemporaryDirectory() as directory:
            inventory = Path(directory) / "tests"
            self._write_inventory(inventory)
            output = Path(directory) / "failure.json"
            steps = copy.deepcopy(self.steps)
            steps["tests"]["outcome"] = "failure"
            env = os.environ.copy()
            env["CORE_STEP_RESULTS"] = json.dumps(steps)
            process = subprocess.run(
                [
                    sys.executable,
                    str(ROOT / "scripts/core_conformance_gate.py"),
                    "--platform",
                    "ubuntu-24.04",
                    "--commit",
                    self.commit,
                    "--test-inventory",
                    str(inventory),
                    "--output",
                    str(output),
                ],
                cwd=ROOT,
                env=env,
                capture_output=True,
                text=True,
                check=False,
            )
            self.assertEqual(process.returncode, 1, process.stderr)
            report = json.loads(output.read_text())
            self.assertFalse(report["checks_passed"])
            self.assertTrue(any(not item["passed"] for item in report["invariants"]))

    def test_cli_wrong_shape_manifest_still_retains_structured_failure(self):
        with tempfile.TemporaryDirectory() as directory:
            inventory = Path(directory) / "tests"
            self._write_inventory(inventory)
            manifest = Path(directory) / "manifest.json"
            manifest.write_text("[]\n")
            output = Path(directory) / "failure.json"
            env = os.environ.copy()
            env["CORE_STEP_RESULTS"] = json.dumps(self.steps)
            process = subprocess.run(
                [
                    sys.executable,
                    str(ROOT / "scripts/core_conformance_gate.py"),
                    "--manifest",
                    str(manifest),
                    "--platform",
                    "ubuntu-24.04",
                    "--commit",
                    self.commit,
                    "--test-inventory",
                    str(inventory),
                    "--output",
                    str(output),
                ],
                cwd=ROOT,
                env=env,
                capture_output=True,
                text=True,
                check=False,
            )
            self.assertEqual(process.returncode, 1, process.stderr)
            report = json.loads(output.read_text())
            self.assertFalse(report["checks_passed"])
            self.assertIn("manifest must be a JSON object", report["error"])

    def test_cli_invalid_utf8_still_retains_structured_failure(self):
        with tempfile.TemporaryDirectory() as directory:
            inventory = Path(directory) / "tests"
            self._write_inventory(inventory)
            manifest = Path(directory) / "manifest.json"
            manifest.write_bytes(b"\xff")
            output = Path(directory) / "failure.json"
            env = os.environ.copy()
            env["CORE_STEP_RESULTS"] = json.dumps(self.steps)
            process = subprocess.run(
                [
                    sys.executable,
                    str(ROOT / "scripts/core_conformance_gate.py"),
                    "--manifest",
                    str(manifest),
                    "--platform",
                    "ubuntu-24.04",
                    "--commit",
                    self.commit,
                    "--test-inventory",
                    str(inventory),
                    "--output",
                    str(output),
                ],
                cwd=ROOT,
                env=env,
                capture_output=True,
                text=True,
                check=False,
            )
            self.assertEqual(process.returncode, 1, process.stderr)
            report = json.loads(output.read_text())
            self.assertFalse(report["checks_passed"])
            self.assertIn("utf-8", report["error"].lower())

    def test_failure_report_is_written_atomically(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "report.json"
            report = {"checks_passed": False, "error": "forced failure"}
            GATE.write_report(path, report)
            self.assertEqual(json.loads(path.read_text()), report)
            self.assertFalse(path.with_suffix(".json.tmp").exists())


if __name__ == "__main__":
    unittest.main()
