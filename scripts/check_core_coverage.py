#!/usr/bin/env python3
"""Validate recorded CORE traceability; inventory consistency is not acceptance."""
from __future__ import annotations

import argparse
import json
import re
from collections import Counter
from pathlib import Path, PurePosixPath
from typing import Any
from urllib.parse import urlparse

EXPECTED = {
    121 + (family - 1) * 8 + number - 1: (f"CORE-{family:02d}.T{number:02d}", family + 1)
    for family in range(1, 5)
    for number in range(1, 9)
}
EXPECTED.update({113: ("PROGRAMME.T03", 1)})
EXPECTED.update({210 + number: (f"EPIC-CORE.T{number:02d}", 13) for number in range(1, 5)})
STATUSES = {
    "not_implemented", "partially_implemented", "implemented_pending_acceptance",
    "blocked", "accepted",
}
SHA = re.compile(r"[0-9a-f]{40}")


def fail(message: str) -> None:
    raise ValueError(message)


def nonempty(value: Any, label: str) -> None:
    if not isinstance(value, str) or not value.strip():
        fail(f"{label} must be a nonempty string")


def positive_integer(value: Any, label: str) -> None:
    if type(value) is not int or value <= 0:
        fail(f"{label} must be a positive integer, not a Boolean")


def source_path(value: Any, root: Path) -> Path:
    nonempty(value, "evidence path")
    path = PurePosixPath(value)
    if path.is_absolute() or ".." in path.parts or "\\" in value:
        fail(f"evidence path must stay inside the repository: {value}")
    resolved = (root / path).resolve()
    if not resolved.is_relative_to(root.resolve()) or not resolved.is_file():
        fail(f"evidence path does not identify a repository file: {value}")
    return resolved


def implementation_evidence(evidence: Any, root: Path, issue: int) -> None:
    if not isinstance(evidence, dict):
        fail(f"task {issue} implementation evidence must be an object")
    if (not isinstance(evidence.get("publication"), str)
            or evidence["publication"] not in {"unpublished", "published"}):
        fail(f"task {issue} must distinguish unpublished from published source")
    nonempty(evidence.get("scope"), f"task {issue} implementation scope")
    for key in ("source_files", "test_files"):
        files = evidence.get(key)
        if not isinstance(files, list) or not files:
            fail(f"task {issue} must name actual {key}")
        for file in files:
            source_path(file, root)
    if evidence["publication"] == "published":
        commit = evidence.get("commit")
        if not isinstance(commit, str) or not SHA.fullmatch(commit):
            fail(f"task {issue} published source needs an exact commit")


def acceptance_evidence(evidence: Any, root: Path, issue: int) -> None:
    if not isinstance(evidence, dict):
        fail(f"task {issue} acceptance evidence must be structured, not just a reference")
    commit = evidence.get("commit")
    if not isinstance(commit, str) or not SHA.fullmatch(commit):
        fail(f"task {issue} acceptance needs an exact tested commit")
    nonempty(evidence.get("reviewer"), f"task {issue} acceptance reviewer")
    review = evidence.get("review_url")
    nonempty(review, f"task {issue} review URL")
    parsed = urlparse(review)
    if (parsed.scheme != "https" or parsed.netloc != "github.com"
            or not parsed.path.startswith("/Jordan-Hall/browser/")
            or not parsed.fragment.startswith(("pullrequestreview-", "issuecomment-"))):
        fail(f"task {issue} must identify the actual acceptance review")
    report = json.loads(source_path(evidence.get("report"), root).read_text(encoding="utf-8"))
    if not isinstance(report, dict) or report.get("commit") != commit:
        fail(f"task {issue} report must match the accepted commit")
    nonempty(report.get("platform"), f"task {issue} verification platform")
    checks = report.get("checks")
    if not isinstance(checks, dict) or not checks or any(v != "success" for v in checks.values()):
        fail(f"task {issue} has missing, skipped or failed verification checks")
    covered = report.get("accepted_task_issues")
    if (not isinstance(covered, list) or any(type(value) is not int for value in covered)
            or issue not in covered):
        fail(f"task {issue} generic smoke evidence is not task acceptance")


def validate_data(data: Any, root: Path, *, require_accepted: bool = False) -> dict[str, int]:
    if not isinstance(data, dict):
        fail("CORE ledger must be an object")
    if type(data.get("schema_version")) is not int or data["schema_version"] != 2:
        fail("CORE ledger schema_version must be 2")
    for field, expected in (("programme_issue", 1), ("epic_issue", 13), ("tracking_pr", 802)):
        if type(data.get(field)) is not int or data[field] != expected:
            fail(f"incorrect {field}")
    parents = data.get("parent_issues")
    if (not isinstance(parents, list) or any(type(item) is not int for item in parents)
            or parents != [2, 3, 4, 5]):
        fail("incorrect CORE parent requirements")
    if type(data.get("production_ready")) is not bool:
        fail("production_ready must be a JSON Boolean")
    tasks = data.get("tasks")
    if not isinstance(tasks, list) or len(tasks) != len(EXPECTED):
        fail("CORE ledger must contain all 37 distinct task issues")
    found = set()
    counts: Counter[str] = Counter()
    for task in tasks:
        if not isinstance(task, dict):
            fail("each CORE task must be an object")
        issue = task.get("issue")
        positive_integer(issue, "task issue")
        if issue not in EXPECTED or issue in found:
            fail(f"unexpected or duplicate CORE issue: {issue}")
        found.add(issue)
        identifier, parent = EXPECTED[issue]
        if (task.get("task") != identifier or type(task.get("parent_issue")) is not int
                or task["parent_issue"] != parent):
            fail(f"task {issue} does not match its stable identifier/parent")
        nonempty(task.get("title"), f"task {issue} title")
        status = task.get("status")
        if not isinstance(status, str) or status not in STATUSES:
            fail(f"unsupported task status: {issue}")
        counts[status] += 1
        prs = task.get("implementation_prs")
        if not isinstance(prs, list):
            fail(f"task {issue} implementation PRs must be an array")
        for pr in prs:
            positive_integer(pr, f"task {issue} PR")
        if len(prs) != len(set(prs)):
            fail(f"task {issue} has duplicate PR references")
        remaining = task.get("remaining")
        if not isinstance(remaining, str):
            fail(f"task {issue} remaining work must be a string")
        sources = task.get("implementation_evidence", [])
        if not isinstance(sources, list):
            fail(f"task {issue} implementation evidence must be an array")
        for evidence in sources:
            implementation_evidence(evidence, root, issue)
        if status == "not_implemented" and (prs or sources):
            fail(f"task {issue} source contributions cannot be labelled not implemented")
        if status == "partially_implemented" and not (prs or sources):
            fail(f"task {issue} partial implementation needs source evidence")
        if status in {"implemented_pending_acceptance", "accepted"} and not prs:
            fail(f"task {issue} needs a published implementation PR")
        if status == "accepted":
            if remaining.strip() or any(e["publication"] == "unpublished" for e in sources):
                fail(f"task {issue} cannot be accepted with unfinished/unpublished work")
            acceptance_evidence(task.get("acceptance_evidence"), root, issue)
        else:
            nonempty(remaining, f"task {issue} remaining acceptance work")
    if found != set(EXPECTED):
        fail("CORE issue inventory is incomplete")
    if (data["production_ready"] or require_accepted) and counts["accepted"] != len(EXPECTED):
        fail(f"CORE release gate failed: {len(EXPECTED) - counts['accepted']} tasks remain unaccepted")
    return dict(counts)


def validate(path: Path, *, root: Path | None = None, require_accepted: bool = False) -> dict[str, int]:
    root = root or path.resolve().parents[1]
    data = json.loads(path.read_text(encoding="utf-8"))
    counts = validate_data(data, root, require_accepted=require_accepted)
    print(f"CORE task ledger valid: {sum(counts.values())} tasks; production_ready={data['production_ready']}")
    print(json.dumps(counts, sort_keys=True))
    return counts


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--require-accepted", action="store_true", help="fail on any unfinished acceptance")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    try:
        validate(root / "docs/core-task-coverage.json", root=root, require_accepted=args.require_accepted)
    except (ValueError, OSError) as error:
        print(str(error))
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
