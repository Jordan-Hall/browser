#!/usr/bin/env python3
"""Validate the CORE coverage ledger without equating references with acceptance."""
from __future__ import annotations
import json
from pathlib import Path


def validate(path: Path) -> None:
    data = json.loads(path.read_text(encoding="utf-8"))
    expected = set(range(121, 153)) | {113, 211, 212, 213, 214}
    tasks = data["tasks"]
    found = [task["issue"] for task in tasks]
    if len(found) != len(set(found)) or set(found) != expected:
        raise ValueError("CORE ledger must contain all 37 distinct task issues")
    identifiers = [task["task"] for task in tasks]
    expected_ids = {f"CORE-{family:02d}.T{number:02d}" for family in range(1, 5) for number in range(1, 9)} | {"PROGRAMME.T03"} | {f"EPIC-CORE.T{number:02d}" for number in range(1, 5)}
    if set(identifiers) != expected_ids or len(identifiers) != len(set(identifiers)):
        raise ValueError("CORE ledger task identifiers are incomplete or duplicated")
    for task in tasks:
        if task["status"] not in {"not_implemented", "implemented_pending_acceptance", "blocked", "accepted"}:
            raise ValueError(f"Unsupported task status: {task['issue']}")
        if task["status"] == "accepted" and (task["remaining"] or not task.get("acceptance_evidence")):
            raise ValueError(f"Task {task['issue']} lacks acceptance evidence")
        if task["status"] == "implemented_pending_acceptance" and not task["implementation_prs"]:
            raise ValueError(f"Task {task['issue']} lacks an implementation PR")
    if data["production_ready"] and any(task["status"] != "accepted" for task in tasks):
        raise ValueError("Production readiness cannot be asserted with unfinished tasks")
    print(f"CORE task ledger valid: {len(tasks)} tasks; production_ready={data['production_ready']}")


if __name__ == "__main__":
    validate(Path(__file__).resolve().parents[1] / "docs/core-task-coverage.json")
