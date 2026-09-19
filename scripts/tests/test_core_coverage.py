"""Regression cases for issue identity, publication and acceptance evidence."""
from __future__ import annotations

import copy
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("core_coverage", ROOT / "scripts/check_core_coverage.py")
assert SPEC and SPEC.loader
coverage = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(coverage)


class CoverageTests(unittest.TestCase):
    def setUp(self) -> None:
        self.data = json.loads((ROOT / "docs/core-task-coverage.json").read_text())

    def reject(self, data=None, *, root=ROOT, release=False) -> None:
        with self.assertRaises(ValueError):
            coverage.validate_data(data or self.data, root, require_accepted=release)

    def test_current_inventory_is_consistent_but_not_accepted(self) -> None:
        counts = coverage.validate_data(self.data, ROOT)
        self.assertEqual(sum(counts.values()), 37)
        self.assertEqual(counts.get("accepted", 0), 0)
        self.assertFalse(self.data["production_ready"])

    def test_release_mode_rejects_consistent_unfinished_inventory(self) -> None:
        self.reject(release=True)

    def test_missing_and_duplicate_issues_are_rejected(self) -> None:
        incomplete = copy.deepcopy(self.data)
        incomplete["tasks"].pop()
        self.reject(incomplete)
        self.data["tasks"][1] = copy.deepcopy(self.data["tasks"][0])
        self.reject()

    def test_swapped_identifiers_cannot_preserve_a_false_complete_set(self) -> None:
        first, second = self.data["tasks"][:2]
        first["task"], second["task"] = second["task"], first["task"]
        self.reject()

    def test_wrong_parent_is_rejected(self) -> None:
        self.data["tasks"][0]["parent_issue"] = 3
        self.reject()

    def test_readiness_and_issue_identifiers_are_strictly_typed(self) -> None:
        for value in ["false", "true", 0, 1, [], None]:
            with self.subTest(value=value):
                data = copy.deepcopy(self.data)
                data["production_ready"] = value
                self.reject(data)
        self.data["tasks"][0]["implementation_prs"] = [True]
        self.reject()

    def test_duplicate_pr_reference_is_rejected(self) -> None:
        self.data["tasks"][0]["implementation_prs"] = [785, 785]
        self.reject()

    def test_implementation_cannot_disappear_behind_not_implemented(self) -> None:
        self.data["tasks"][0]["status"] = "not_implemented"
        self.reject()

    def test_partial_implementation_requires_an_actual_source_contribution(self) -> None:
        task = next(t for t in self.data["tasks"] if t["issue"] == 146)
        task.update(status="partially_implemented", implementation_prs=[], implementation_evidence=[])
        self.reject()

    def test_unknown_status_fails_closed(self) -> None:
        self.data["tasks"][0]["status"] = "green_means_done"
        self.reject()

    def test_missing_traversing_and_escaping_symlink_evidence_is_rejected(self) -> None:
        task = next(t for t in self.data["tasks"] if t["issue"] == 137)
        for path in ["crates/missing.rs", "../outside.rs", "/etc/passwd", r"..\outside.rs"]:
            with self.subTest(path=path):
                task["implementation_evidence"][0]["source_files"] = [path]
                self.reject()
        with tempfile.TemporaryDirectory() as directory:
            temp = Path(directory)
            root = temp / "repo"
            root.mkdir()
            target = temp / "outside-target"
            target.mkdir()
            outside = target / "outside.rs"
            outside.write_text("not repository source")
            if os.name == "nt":
                link = root / "outside"
                command = f'mklink /J "{link}" "{target}"'
                subprocess.run(
                    [os.environ.get("COMSPEC", "cmd.exe"), "/d", "/c", command],
                    check=True, capture_output=True, timeout=10,
                )
                try:
                    self.assertTrue(link.is_junction())
                    self.assertTrue((link / "outside.rs").samefile(outside))
                    with self.assertRaises(ValueError):
                        coverage.source_path("outside/outside.rs", root)
                finally:
                    link.rmdir()
            else:
                (root / "link.rs").symlink_to(outside)
                with self.assertRaises(ValueError):
                    coverage.source_path("link.rs", root)

    def test_unpublished_changes_cannot_be_marked_accepted(self) -> None:
        task = next(t for t in self.data["tasks"] if t["issue"] == 137)
        task["implementation_evidence"][0]["publication"] = "unpublished"
        task.update(status="accepted", implementation_prs=[785], remaining="")
        self.reject()

    def test_reference_or_smoke_report_is_not_acceptance(self) -> None:
        task = self.data["tasks"][0]
        task.update(status="accepted", remaining="", acceptance_evidence="PR #785")
        self.reject()
        task["acceptance_evidence"] = {"commit": "a" * 40, "report": "README.md"}
        self.reject()

    def test_structured_acceptance_requires_exact_commit_scope_and_success(self) -> None:
        # This tests schema consistency only, not authenticity of fabricated review/report data.
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            report_path = root / "report.json"
            report = {"commit": "a" * 40, "platform": "test-only", "checks": {"test": "success"},
                      "accepted_task_issues": [121]}
            evidence = {"commit": "a" * 40, "reviewer": "schema-test",
                        "review_url": "https://github.com/Jordan-Hall/browser/pull/785#pullrequestreview-1",
                        "report": "report.json"}
            report_path.write_text(json.dumps(report))
            coverage.acceptance_evidence(evidence, root, 121)
            for key, value in [("commit", "b" * 40), ("accepted_task_issues", []),
                               ("checks", {"test": "skipped"}), ("checks", {}), ("platform", "")]:
                changed = dict(report)
                changed[key] = value
                report_path.write_text(json.dumps(changed))
                with self.subTest(key=key, value=value), self.assertRaises(ValueError):
                    coverage.acceptance_evidence(evidence, root, 121)

    def test_production_flag_cannot_be_raised_by_passing_inventory(self) -> None:
        self.data["production_ready"] = True
        self.reject()


if __name__ == "__main__":
    unittest.main()
