from __future__ import annotations

import importlib.util
import tempfile
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).resolve().parents[1] / "rust_toolchain.py"
SPEC = importlib.util.spec_from_file_location("rust_toolchain", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
rust_toolchain = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(rust_toolchain)


class RustToolchainTests(unittest.TestCase):
    def write_config(self, root: Path, channel: str, rust_version: str) -> tuple[Path, Path]:
        toolchain = root / "rust-toolchain.toml"
        workspace = root / "Cargo.toml"
        toolchain.write_text(
            f'[toolchain]\nchannel = "{channel}"\nprofile = "minimal"\n', encoding="utf-8"
        )
        workspace.write_text(
            f'[workspace]\n[workspace.package]\nrust-version = "{rust_version}"\n',
            encoding="utf-8",
        )
        return toolchain, workspace

    def test_exact_channel_matches_workspace_major_minor(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            toolchain, workspace = self.write_config(Path(directory), "1.98.1", "1.98")
            self.assertEqual(
                rust_toolchain.load_pinned_toolchain(toolchain, workspace), "1.98.1"
            )

    def test_rejects_floating_toolchain_channel(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            toolchain, workspace = self.write_config(Path(directory), "stable", "1.98")
            with self.assertRaisesRegex(ValueError, "exact x.y.z"):
                rust_toolchain.load_pinned_toolchain(toolchain, workspace)

    def test_rejects_workspace_toolchain_drift(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            toolchain, workspace = self.write_config(Path(directory), "1.99.0", "1.98")
            with self.assertRaisesRegex(ValueError, "must match"):
                rust_toolchain.load_pinned_toolchain(toolchain, workspace)

    def test_parses_exact_rustc_version(self) -> None:
        self.assertEqual(
            rust_toolchain.parse_rustc_version("rustc 1.98.1 (abc123 2026-09-01)\n"),
            "1.98.1",
        )


if __name__ == "__main__":
    unittest.main()
