#!/usr/bin/env python3
"""Validate and report the machine-checkable CORE-01 acceptance evidence map."""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from pathlib import Path, PurePosixPath
from typing import Any

MANIFEST = Path("docs/core-01-conformance.json")
VALID_CLASSES = {"unit", "subprocess", "conformance", "platform"}
VALID_STEPS = {"tests", "doctests", "conformance", "architecture", "fuzz_compile"}
TEST_INVENTORY_STEP = "test_inventory"
COMMIT_RE = re.compile(r"^[0-9a-f]{40}$")
MAX_ERROR = 512


class GateError(ValueError):
    pass


def _bounded(message: str) -> str:
    return message[:MAX_ERROR]


def _rust_function_attributes(path: Path, name: str) -> str | None:
    source = path.read_text(encoding="utf-8")
    match = re.search(
        rf"(?P<attrs>(?:^[ \t]*#\[[^\n]+\][ \t]*\n)*)^[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?fn[ \t]+{re.escape(name)}[ \t]*(?:<[^>]*>)?[ \t]*\(",
        source,
        re.MULTILINE,
    )
    return None if match is None else match.group("attrs")


def _repo_file(root: Path, relative: str, evidence_id: str) -> Path:
    candidate = PurePosixPath(relative)
    if candidate.is_absolute() or ".." in candidate.parts or not candidate.parts:
        raise GateError(f"{evidence_id} path escapes repository: {relative}")
    path = root.joinpath(*candidate.parts)
    if not path.is_file():
        raise GateError(f"{evidence_id} file does not exist: {relative}")
    return path


def validate_manifest(manifest: dict[str, Any], root: Path) -> list[dict[str, Any]]:
    if manifest.get("schema_version") != 1:
        raise GateError("unsupported conformance manifest schema_version")
    required = manifest.get("required_invariants")
    invariants = manifest.get("invariants")
    if not isinstance(required, list) or not required or not all(isinstance(x, str) for x in required):
        raise GateError("required_invariants must be a non-empty string list")
    if len(required) != len(set(required)):
        raise GateError("required_invariants contains duplicates")
    if not isinstance(invariants, list):
        raise GateError("invariants must be a list")
    by_id: dict[str, dict[str, Any]] = {}
    evidence_definitions: dict[str, tuple[Any, ...]] = {}
    for invariant in invariants:
        if not isinstance(invariant, dict) or not isinstance(invariant.get("id"), str):
            raise GateError("every invariant must have a string id")
        invariant_id = invariant["id"]
        if invariant_id in by_id:
            raise GateError(f"duplicate invariant {invariant_id}")
        by_id[invariant_id] = invariant
        evidence = invariant.get("evidence")
        required_evidence = invariant.get("required_evidence_ids")
        if not isinstance(evidence, list) or not evidence:
            raise GateError(f"{invariant_id} has no evidence")
        if (
            not isinstance(required_evidence, list)
            or not required_evidence
            or not all(isinstance(item, str) and item for item in required_evidence)
        ):
            raise GateError(f"{invariant_id} has invalid required_evidence_ids")
        if len(required_evidence) != len(set(required_evidence)):
            raise GateError(f"{invariant_id} contains duplicate required_evidence_ids")
        evidence_ids: list[str] = []
        for item in evidence:
            if not isinstance(item, dict):
                raise GateError(f"{invariant_id} contains non-object evidence")
            evidence_id = item.get("id")
            if not isinstance(evidence_id, str) or not evidence_id:
                raise GateError(f"{invariant_id} evidence is missing id")
            evidence_ids.append(evidence_id)
            evidence_class = item.get("evidence_class")
            if evidence_class not in VALID_CLASSES:
                raise GateError(f"{evidence_id} has invalid evidence_class")
            step = item.get("step")
            if step not in VALID_STEPS:
                raise GateError(f"{evidence_id} has invalid step")
            test_file = item.get("test_file")
            test_name = item.get("test_name")
            targets = item.get("supported_targets")
            if not isinstance(test_file, str) or not isinstance(test_name, str):
                raise GateError(f"{evidence_id} must name a test_file and test_name")
            if not isinstance(targets, list) or not targets or not all(isinstance(x, str) for x in targets):
                raise GateError(f"{evidence_id} must name supported_targets")
            file_path = _repo_file(root, test_file, evidence_id)
            attributes = _rust_function_attributes(file_path, test_name)
            if attributes is None:
                raise GateError(f"{evidence_id} test function does not exist: {test_name}")
            if evidence_class in {"unit", "subprocess"}:
                if re.search(r"#\[test\]", attributes) is None:
                    raise GateError(f"{evidence_id} does not reference a #[test] function")
                if re.search(r"#\[ignore(?:\([^]]*\))?\]", attributes) is not None:
                    raise GateError(f"{evidence_id} references an ignored test")
            inventory_name = item.get("inventory_name")
            if evidence_class in {"unit", "subprocess"}:
                if not isinstance(inventory_name, str) or not inventory_name:
                    raise GateError(f"{evidence_id} must name its exact inventory_name")
            elif inventory_name is not None:
                raise GateError(f"{evidence_id} has inventory_name for non-test evidence")
            fixture = item.get("fixture")
            if fixture is not None:
                if not isinstance(fixture, str):
                    raise GateError(f"{evidence_id} has invalid fixture path")
                _repo_file(root, fixture, evidence_id)
            definition = (
                evidence_class, step, test_file, test_name, inventory_name, fixture, tuple(targets)
            )
            previous = evidence_definitions.get(evidence_id)
            if previous is not None and previous != definition:
                raise GateError(f"{evidence_id} is defined inconsistently")
            evidence_definitions[evidence_id] = definition
        if len(evidence_ids) != len(set(evidence_ids)):
            raise GateError(f"{invariant_id} contains duplicate evidence ids")
        missing_evidence = sorted(set(required_evidence) - set(evidence_ids))
        if missing_evidence:
            raise GateError(f"{invariant_id} is missing required evidence: {', '.join(missing_evidence)}")
    missing = sorted(set(required) - set(by_id))
    extra = sorted(set(by_id) - set(required))
    if missing or extra:
        raise GateError(f"invariant set mismatch: missing={missing} extra={extra}")
    return [by_id[invariant_id] for invariant_id in required]


def build_report(
    manifest: dict[str, Any],
    root: Path,
    platform: str,
    commit: str,
    steps: dict[str, Any],
    test_inventory: set[str],
) -> tuple[dict[str, Any], bool]:
    if not COMMIT_RE.fullmatch(commit):
        raise GateError("commit must be an exact 40-character lowercase SHA")
    invariants = validate_manifest(manifest, root)
    results: list[dict[str, Any]] = []
    overall = True
    for invariant in invariants:
        evidence_results: list[dict[str, Any]] = []
        for evidence in invariant["evidence"]:
            supported = platform in evidence["supported_targets"]
            outcome = steps.get(evidence["step"], {}).get("outcome", "not_run") if supported else "unsupported"
            inventory_result = "not_applicable"
            if supported and evidence["evidence_class"] in {"unit", "subprocess"}:
                inventory_outcome = steps.get(TEST_INVENTORY_STEP, {}).get("outcome", "not_run")
                inventory_result = (
                    "present"
                    if inventory_outcome == "success"
                    and evidence["inventory_name"] in test_inventory
                    else "missing"
                )
            passed = (
                supported
                and outcome == "success"
                and inventory_result in {"not_applicable", "present"}
            )
            evidence_results.append(
                {
                    "id": evidence["id"],
                    "class": evidence["evidence_class"],
                    "step": evidence["step"],
                    "test_file": evidence["test_file"],
                    "test_name": evidence["test_name"],
                    "inventory_name": evidence.get("inventory_name"),
                    "fixture": evidence.get("fixture"),
                    "supported_targets": evidence["supported_targets"],
                    "tested_revision": commit if supported else None,
                    "result": outcome,
                    "test_inventory": inventory_result,
                    "passed": passed,
                }
            )
        applicable = [item for item in evidence_results if item["result"] != "unsupported"]
        required_ids = set(invariant["required_evidence_ids"])
        applicable_required = {item["id"] for item in applicable if item["id"] in required_ids}
        invariant_passed = (
            bool(applicable)
            and applicable_required == required_ids
            and all(item["passed"] for item in applicable)
        )
        overall &= invariant_passed
        results.append(
            {
                "id": invariant["id"],
                "requirement": invariant["requirement"],
                "passed": invariant_passed,
                "evidence": evidence_results,
            }
        )
    return (
        {
            "schema_version": 1,
            "scope": "CORE-01 acceptance evidence mapping",
            "parent_issue": manifest["parent_issue"],
            "commit": commit,
            "platform": platform,
            "invariants": results,
            "checks_passed": overall,
            "acceptance": "not_established_by_this_mapping",
        },
        overall,
    )


def parse_test_inventory(path: Path) -> set[str]:
    tests: set[str] = set()
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.endswith(": test"):
            continue
        qualified = line[: -len(": test")].strip()
        if qualified:
            tests.add(qualified)
    if not tests:
        raise GateError("test inventory contains no executable tests")
    return tests


def write_report(path: Path, report: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    temporary.replace(path)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, default=MANIFEST)
    parser.add_argument("--platform", required=True)
    parser.add_argument("--commit", required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--test-inventory", type=Path, required=True)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    root = Path.cwd()
    report: dict[str, Any]
    passed = False
    try:
        manifest = json.loads(args.manifest.read_text(encoding="utf-8"))
        steps = json.loads(os.environ.get("CORE_STEP_RESULTS", "{}"))
        inventory = parse_test_inventory(args.test_inventory)
        report, passed = build_report(
            manifest, root, args.platform, args.commit, steps, inventory
        )
    except (GateError, json.JSONDecodeError, OSError) as error:
        report = {
            "schema_version": 1,
            "scope": "CORE-01 acceptance evidence mapping",
            "commit": args.commit,
            "platform": args.platform,
            "checks_passed": False,
            "acceptance": "not_established_by_this_mapping",
            "error": _bounded(str(error)),
        }
    write_report(args.output, report)
    print(json.dumps(report, indent=2))
    return 0 if passed else 1


if __name__ == "__main__":
    sys.exit(main())
