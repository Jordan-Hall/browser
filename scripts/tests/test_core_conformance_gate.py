import copy
import importlib.util
import json
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
        }
        self.commit = "a" * 40

    def test_current_manifest_is_complete_and_success_is_revision_bound(self):
        report, passed = GATE.build_report(
            self.manifest, ROOT, "ubuntu-24.04", self.commit, self.steps
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
                self.manifest, ROOT, "ubuntu-24.04", self.commit, steps
            )
            self.assertFalse(passed)
            self.assertFalse(report["checks_passed"])

    def test_platform_with_no_complete_required_evidence_cannot_pass(self):
        report, passed = GATE.build_report(
            self.manifest, ROOT, "plan9", self.commit, self.steps
        )
        self.assertFalse(passed)
        self.assertFalse(report["checks_passed"])
        self.assertTrue(
            all(not invariant["passed"] for invariant in report["invariants"])
        )

    def test_failure_report_is_written_atomically(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "report.json"
            report = {"checks_passed": False, "error": "forced failure"}
            GATE.write_report(path, report)
            self.assertEqual(json.loads(path.read_text()), report)
            self.assertFalse(path.with_suffix(".json.tmp").exists())


if __name__ == "__main__":
    unittest.main()
