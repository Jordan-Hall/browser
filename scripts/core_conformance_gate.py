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
TARGET_PART_RE = re.compile(r"^[A-Za-z0-9_.-]+$")
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


def _validate_target_part(value: Any, field: str, evidence_id: str) -> str:
    if (
        not isinstance(value, str)
        or not value
        or "--" in value
        or TARGET_PART_RE.fullmatch(value) is None
    ):
        raise GateError(f"{evidence_id} has invalid cargo_target.{field}")
    return value


def _cargo_target(item: dict[str, Any], evidence_id: str) -> tuple[str, str, str | None]:
    target = item.get("cargo_target")
    if not isinstance(target, dict):
        raise GateError(f"{evidence_id} must name cargo_target")
    package = _validate_target_part(target.get("package"), "package", evidence_id)
    kind = target.get("kind")
    if kind not in {"lib", "test"}:
        raise GateError(f"{evidence_id} has invalid cargo_target.kind")
    name = target.get("name")
    if kind == "lib":
        if name is not None:
            raise GateError(f"{evidence_id} lib cargo_target must not name a target")
        return package, kind, None
    return package, kind, _validate_target_part(name, "name", evidence_id)


def _target_inventory_id(target: tuple[str, str, str | None]) -> str:
    package, kind, name = target
    if kind == "lib":
        return f"{package}--lib"
    assert name is not None
    return f"{package}--test--{name}"


def validate_manifest(manifest: Any, root: Path) -> list[dict[str, Any]]:
    if not isinstance(manifest, dict):
        raise GateError("manifest must be a JSON object")
    if manifest.get("schema_version") != 1:
        raise GateError("unsupported conformance manifest schema_version")
    parent_issue = manifest.get("parent_issue")
    if isinstance(parent_issue, bool) or not isinstance(parent_issue, int) or parent_issue <= 0:
        raise GateError("parent_issue must be a positive integer")
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
        requirement = invariant.get("requirement")
        if not isinstance(requirement, str) or not requirement.strip():
            raise GateError(f"{invariant_id} must have a non-empty requirement")
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
            if len(targets) != len(set(targets)):
                raise GateError(f"{evidence_id} contains duplicate supported_targets")
            file_path = _repo_file(root, test_file, evidence_id)
            attributes = _rust_function_attributes(file_path, test_name)
            if attributes is None:
                raise GateError(f"{evidence_id} test function does not exist: {test_name}")
            inventory_name = item.get("inventory_name")
            cargo_target: tuple[str, str, str | None] | None = None
            if evidence_class in {"unit", "subprocess"}:
                if re.search(r"#\[test\]", attributes) is None:
                    raise GateError(f"{evidence_id} does not reference a #[test] function")
                if re.search(r"\bignore\b", attributes) is not None:
                    raise GateError(
                        f"{evidence_id} references a conditionally or explicitly ignored test"
                    )
                if not isinstance(inventory_name, str) or not inventory_name:
                    raise GateError(f"{evidence_id} must name its exact inventory_name")
                cargo_target = _cargo_target(item, evidence_id)
            else:
                if inventory_name is not None:
                    raise GateError(f"{evidence_id} has inventory_name for non-test evidence")
                if item.get("cargo_target") is not None:
                    raise GateError(f"{evidence_id} has cargo_target for non-test evidence")
            fixture = item.get("fixture")
            if fixture is not None:
                if not isinstance(fixture, str):
                    raise GateError(f"{evidence_id} has invalid fixture path")
                _repo_file(root, fixture, evidence_id)
            definition = (
                evidence_class,
                step,
                test_file,
                test_name,
                inventory_name,
                cargo_target,
                fixture,
                tuple(targets),
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
    manifest: Any,
    root: Path,
    platform: str,
    commit: str,
    steps: Any,
    test_inventory: dict[str, set[str]],
) -> tuple[dict[str, Any], bool]:
    if not COMMIT_RE.fullmatch(commit):
        raise GateError("commit must be an exact 40-character lowercase SHA")
    if not isinstance(steps, dict):
        raise GateError("CORE_STEP_RESULTS must be a JSON object")
    if not isinstance(test_inventory, dict):
        raise GateError("test inventory must preserve Cargo target provenance")
    invariants = validate_manifest(manifest, root)
    results: list[dict[str, Any]] = []
    overall = True
    for invariant in invariants:
        evidence_results: list[dict[str, Any]] = []
        for evidence in invariant["evidence"]:
            supported = platform in evidence["supported_targets"]
            step_state = steps.get(evidence["step"], {})
            outcome = (
                step_state.get("outcome", "not_run")
                if supported and isinstance(step_state, dict)
                else ("invalid" if supported else "unsupported")
            )
            inventory_result = "not_applicable"
            inventory_target = None
            if supported and evidence["evidence_class"] in {"unit", "subprocess"}:
                target = _cargo_target(evidence, evidence["id"])
                inventory_target = _target_inventory_id(target)
                inventory_step = steps.get(TEST_INVENTORY_STEP, {})
                target_tests = test_inventory.get(inventory_target, set())
                inventory_result = (
                    "present"
                    if isinstance(inventory_step, dict)
                    and inventory_step.get("outcome") == "success"
                    and evidence["inventory_name"] in target_tests
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
                    "inventory_target": inventory_target,
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


def parse_test_inventory(path: Path) -> dict[str, set[str]]:
    if not path.is_dir():
        raise GateError("test inventory path must be a directory")
    inventories: dict[str, set[str]] = {}
    for target_file in sorted(path.glob("*.txt")):
        tests: set[str] = set()
        for line in target_file.read_text(encoding="utf-8").splitlines():
            if not line.endswith(": test"):
                continue
            qualified = line[: -len(": test")].strip()
            if qualified:
                tests.add(qualified)
        if not tests:
            raise GateError(f"test inventory contains no executable tests: {target_file.name}")
        inventories[target_file.stem] = tests
    if not inventories:
        raise GateError("test inventory contains no Cargo targets")
    return inventories


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
