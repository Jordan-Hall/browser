#!/usr/bin/env python3
"""Build and run bounded, seeded parser fuzzing with retained verification inputs."""
from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import subprocess

TOOLCHAIN = "nightly-2026-09-17"
CARGO_FUZZ_VERSION = "0.13.2"
TARGETS = ("frame_decoder", "control_envelope", "record_codec")
FAMILIES = (
    "goal_contract", "workspace", "task", "capability", "observation", "evidence",
    "action_proposal", "approval", "operation", "receipt", "view_definition",
    "memory_record", "artifact_reference",
)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def lockfiles(root: Path) -> dict[str, str]:
    return {name: sha256(root / name) for name in ("Cargo.lock", "fuzz/Cargo.lock")}


def seeds(target: str, fixtures: list[dict]) -> list[bytes]:
    records = {item["family"]: item for item in fixtures}
    if len(records) != len(fixtures) or set(records) != {"intent." + name for name in FAMILIES}:
        raise ValueError("seed fixtures must cover each CORE record family exactly once")
    encoded = lambda value: json.dumps(value, separators=(",", ":"), ensure_ascii=False).encode()
    if target == "record_codec":
        current = [bytes([index]) + encoded(records["intent." + name][shape])
                   for index, name in enumerate(FAMILIES) for shape in ("full", "minimal")]
        future = [bytes([index]) + encoded({**records["intent." + name][shape],
                  "schema_version": {"major": 1, "minor": 1},
                  "future_only": {"unrecognized_permission": False}})
                  for index, name in enumerate(FAMILIES) for shape in ("full", "minimal")]
        unsupported = [bytes([index]) + b'{"schema_version":{"major":2,"minor":0}}'
                       for index in range(len(FAMILIES))]
        duplicate = [bytes([index]) + b'{"schema_version":{"major":1,"minor":1},"x":0,"x":1}'
                     for index in range(len(FAMILIES))]
        wide_number = [bytes([index]) + b'{"future":[1e400,{"negative":-1e400,"tiny":1e-4000}],'
                       b'"schema_version":{"major":1,"minor":1}}'
                       for index in range(len(FAMILIES))]
        return current + future + unsupported + duplicate + wide_number
    envelopes = [encoded({"schema_version": {"major": 1, "minor": 0},
                          "trace_id": "018f47f7-5a86-7c00-8000-000000000099",
                          "message": {"kind": "event"}, "payload": records["intent." + name]["full"]})
                 for name in FAMILIES]
    malformed = [b'{"a":1,"a":2}', b'{"a":1,"\\u0061":2}', b"[" * 100 + b"]" * 100,
                 b'{"schema_version":{"major":2,"minor":0}}', b'{}{}', b'\xff']
    if target == "control_envelope":
        return envelopes + malformed
    if target == "frame_decoder":
        frames = [b"\x01" + len(data).to_bytes(4, "big") + data for data in envelopes]
        return frames + [b"".join(frames), b"\x01\xff\xff\xff\xff", b"\x03\0\0\0\0",
                         b"\x01\0\0\0\0" * 10, b"\x02\0\0\0\x02{}"]
    raise ValueError("unsupported fuzz target")


def run(command: list[str], log: Path, *, timeout: int, env: dict[str, str]) -> int:
    with log.open("wb") as stream:
        stream.write(("$ " + " ".join(command) + "\n").encode())
        stream.flush()
        process = subprocess.Popen(command, stdout=stream, stderr=subprocess.STDOUT,
                                   env=env, start_new_session=True)
        try:
            return process.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            try:
                os.killpg(process.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            process.wait(timeout=10)
            stream.write(b"\nVerification process group exceeded its wall-time budget.\n")
            return 124


def result_summary(text: str, returncode: int) -> dict:
    units = re.findall(r"stat::number_of_executed_units:\s*(\d+)", text)
    coverage = [int(value) for value in re.findall(r"\bcov:\s*(\d+)", text)]
    instrumented = bool(re.search(r"INFO: Loaded .*\b(?:counters|PCs)\b", text))
    executed = int(units[-1]) if units else 0
    passed = returncode == 0 and instrumented and executed > 0 and bool(coverage) and max(coverage) > 0
    return {"returncode": returncode, "instrumentation_observed": instrumented,
            "executed_units": executed, "max_reported_coverage": max(coverage, default=0),
            "result": "success" if passed else "failure"}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("target", choices=TARGETS)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    os.chdir(root)
    output = root / "target/fuzz-evidence" / args.target
    corpus, crashes = output / "corpus", output / "crashes"
    corpus.mkdir(parents=True, exist_ok=True)
    crashes.mkdir(parents=True, exist_ok=True)
    before = lockfiles(root)
    report = {"schema_version": 1, "scope": "bounded_parser_fuzz_smoke", "target": args.target,
              "started_utc": dt.datetime.now(dt.timezone.utc).isoformat(),
              "toolchain": TOOLCHAIN, "cargo_fuzz_version": CARGO_FUZZ_VERSION,
              "sanitizer": "address", "seconds": 60, "seed": 183726,
              "max_input_bytes": 65536, "rss_limit_mb": 1024, "input_timeout_seconds": 5,
              "checks": {"build": "not_run", "fuzz": "not_run", "locks_unchanged": "not_run"},
              "production_ready": False, "lockfile_sha256": before}
    env = dict(os.environ, CARGO_NET_OFFLINE="true", RUSTUP_TOOLCHAIN=TOOLCHAIN)
    try:
        report["commit"] = subprocess.check_output(["git", "rev-parse", "HEAD"], text=True, timeout=10).strip()
        report["rustc"] = subprocess.check_output(["rustc", f"+{TOOLCHAIN}", "-Vv"], env=env, text=True, timeout=30).strip()
        report["cargo_fuzz"] = subprocess.check_output(["cargo", f"+{TOOLCHAIN}", "fuzz", "--version"], env=env, text=True, timeout=30).strip()
        if report["cargo_fuzz"] != f"cargo-fuzz {CARGO_FUZZ_VERSION}":
            raise ValueError("unexpected cargo-fuzz version")
        fixture_path = root / "fixtures/core/records-v1.json"
        report["fixture_sha256"] = sha256(fixture_path)
        for index, data in enumerate(seeds(args.target, json.loads(fixture_path.read_text()))):
            (corpus / f"seed-{index:02d}").write_bytes(data)
        report["seed_sha256"] = {path.name: sha256(path) for path in sorted(corpus.iterdir()) if path.is_file()}
        build_root = root / "target/core-fuzz-build"
        code = run(["cargo", f"+{TOOLCHAIN}", "fuzz", "build", args.target,
                    "--sanitizer", "address", "--target", "x86_64-unknown-linux-gnu",
                    "--target-dir", str(build_root)], output / "build.log", timeout=600, env=env)
        report["checks"]["build"] = "success" if code == 0 else "failure"
        if code != 0:
            return 1
        if lockfiles(root) != before:
            raise ValueError("fuzz build changed a committed lockfile")
        binary = build_root / "x86_64-unknown-linux-gnu/release" / args.target
        report["binary_sha256"] = sha256(binary)
        command = [str(binary), str(corpus), "-max_total_time=60", "-timeout=5",
                   "-rss_limit_mb=1024", "-malloc_limit_mb=128", "-max_len=65536",
                   "-seed=183726", "-print_final_stats=1", f"-artifact_prefix={crashes}/"]
        code = run(command, output / "fuzz.log", timeout=100, env=env)
        result = result_summary((output / "fuzz.log").read_text(errors="replace"), code)
        report["execution"] = result
        report["checks"]["fuzz"] = result["result"]
        if result["result"] != "success":
            inputs = sorted(path for path in crashes.iterdir() if path.is_file() and path.name.startswith("crash-"))
            if inputs:
                report["minimization_exit_code"] = run(
                    [str(binary), str(inputs[0]), "-minimize_crash=1", "-max_total_time=15", "-timeout=5",
                     "-rss_limit_mb=1024", f"-exact_artifact_path={crashes}/minimized"],
                    output / "minimization.log", timeout=30, env=env)
            return 1
        return 0
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        report["error"] = str(error)[:2048]
        return 1
    finally:
        unchanged = lockfiles(root) == before
        report["checks"]["locks_unchanged"] = "success" if unchanged else "failure"
        report["finished_utc"] = dt.datetime.now(dt.timezone.utc).isoformat()
        report["checks_passed"] = all(value == "success" for value in report["checks"].values())
        (output / "verification.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
        print(json.dumps(report, indent=2))
        if not unchanged:
            raise RuntimeError("fuzz verification changed a committed lockfile")


if __name__ == "__main__":
    raise SystemExit(main())
