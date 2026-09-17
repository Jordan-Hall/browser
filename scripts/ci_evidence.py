#!/usr/bin/env python3
"""Retain CI outcomes without mistaking skipped checks for passing checks."""

from __future__ import annotations

import hashlib
import json
import os
import platform
import shutil
import subprocess
from pathlib import Path

REQUIRED_CHECKS = (
    "checkout",
    "toolchain",
    "dependencies",
    "lockfiles",
    "format",
    "clippy",
    "tests",
    "doctests",
    "fuzz_compile",
    "architecture",
    "conformance",
)


def main() -> None:
    output = Path("target/conformance")
    output.mkdir(parents=True, exist_ok=True)
    steps = json.loads(os.environ.get("CORE_STEP_RESULTS", "{}"))
    checks = {
        name: steps.get(name, {}).get("outcome", "not_run")
        for name in REQUIRED_CHECKS
    }
    locks = {}
    for source, name in (
        (Path("Cargo.lock"), "workspace-Cargo.lock"),
        (Path("fuzz/Cargo.lock"), "fuzz-Cargo.lock"),
    ):
        if source.is_file():
            data = source.read_bytes()
            locks[str(source)] = hashlib.sha256(data).hexdigest()
            shutil.copyfile(source, output / name)
        else:
            locks[str(source)] = None
    commit = subprocess.check_output(
        ["git", "rev-parse", "HEAD"], text=True, timeout=10
    ).strip()
    report = {
        "schema_version": 1,
        "scope": "core_contract_and_state_checks",
        "commit": commit,
        "run_id": os.environ.get("GITHUB_RUN_ID"),
        "run_attempt": os.environ.get("GITHUB_RUN_ATTEMPT"),
        "platform": platform.platform(),
        "rust_toolchain": "1.98.1",
        "checks": checks,
        "checks_passed": all(value == "success" for value in checks.values()),
        "production_readiness": "not_established_by_this_smoke_suite",
        "lockfile_sha256": locks,
    }
    temporary = output / "verification.json.tmp"
    temporary.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    temporary.replace(output / "verification.json")
    subprocess.run(
        ["git", "archive", "--format=tar", "-o", str(output / "source.tar"), "HEAD"],
        check=True,
        timeout=30,
    )
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
