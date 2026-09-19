#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TOOLCHAIN_FILE = ROOT / "rust-toolchain.toml"
WORKSPACE_FILE = ROOT / "Cargo.toml"
EXACT_VERSION = re.compile(r"\d+\.\d+\.\d+\Z")
RUSTC_VERSION = re.compile(r"rustc (\d+\.\d+\.\d+)(?:\s.*)?\Z")


def load_pinned_toolchain(
    toolchain_file: Path = TOOLCHAIN_FILE,
    workspace_file: Path = WORKSPACE_FILE,
) -> str:
    with toolchain_file.open("rb") as handle:
        toolchain = tomllib.load(handle)
    channel = toolchain.get("toolchain", {}).get("channel")
    if not isinstance(channel, str) or EXACT_VERSION.fullmatch(channel) is None:
        raise ValueError("rust-toolchain.toml must pin an exact x.y.z channel")

    with workspace_file.open("rb") as handle:
        workspace = tomllib.load(handle)
    rust_version = workspace.get("workspace", {}).get("package", {}).get("rust-version")
    if not isinstance(rust_version, str) or not channel.startswith(f"{rust_version}."):
        raise ValueError(
            "Cargo.toml workspace.package.rust-version must match the pinned toolchain major.minor"
        )
    return channel


def parse_rustc_version(output: str) -> str:
    match = RUSTC_VERSION.fullmatch(output.strip())
    if match is None:
        raise ValueError(f"unexpected rustc --version output: {output.strip()!r}")
    return match.group(1)


def verify_active_toolchain(channel: str) -> None:
    result = subprocess.run(
        ["rustc", "--version"],
        check=True,
        capture_output=True,
        text=True,
    )
    active = parse_rustc_version(result.stdout)
    if active != channel:
        raise RuntimeError(f"active rustc {active} does not match pinned toolchain {channel}")


def main() -> int:
    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--channel", action="store_true")
    group.add_argument("--verify-active", action="store_true")
    args = parser.parse_args()

    try:
        channel = load_pinned_toolchain()
        if args.channel:
            print(channel)
        else:
            verify_active_toolchain(channel)
    except (OSError, subprocess.CalledProcessError, RuntimeError, ValueError) as error:
        print(error, file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
