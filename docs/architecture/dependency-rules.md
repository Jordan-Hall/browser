# Dependency direction

The trusted contract layer is intentionally a leaf in the Intent Browser architecture.

## Rules

- `intent-contracts` may use authority-neutral serialization/value libraries, but it must not depend on any other workspace crate.
- `intent-contracts` must not depend on browser engines, GUI frameworks, OS automation crates, provider SDKs, or agent adapters.
- `intent-ipc` may depend on `intent-contracts`.
- Workers and brokers may depend on the IPC/contracts layers, never the reverse.
- Raw browser, operating-system, provider, or process handles must stay behind brokers and must not leak into durable contracts.

`cargo run --locked -p intent-xtask -- arch-check` enforces the contract-layer rule from Cargo metadata. The checker is fail-closed for every declared and resolved direct `intent-contracts` dependency: normal, development, build and target-specific dependencies are all subject to the reviewed allowlist and source policy, including Cargo aliases. Path/workspace and non-crates.io sources are rejected. Expanding the allowlist is an architecture change and must be reviewed.

The command resolves the locked Cargo graph with all features enabled and without a platform filter. It locates `intent-contracts` by workspace membership and package identity, then applies the same package/source policy to the packages actually selected for its direct dependency edges. A registry declaration is not sufficient: `[patch]` and `[replace]` can substitute local or Git code while leaving the declaration unchanged. Missing resolution nodes or package identities fail closed. This resolution step does not execute dependency build scripts.

This command deliberately does **not** claim to audit the complete transitive supply chain. Transitive packages are constrained by the committed `Cargo.lock`; changing them is reviewed through the lockfile diff and locked CI. Supply-chain provenance or vulnerability qualification is a separate gate, not an implicit property of this architecture check.

The original integration regression invokes the real `arch-check` executable against an isolated Cargo workspace containing a target-specific dependency whose package is renamed to an allowed dependency name, and verifies that the checker fails. Unit tests separately exercise unknown package and unreviewed-source rejection.

Six additional command-level regressions substitute a local package named `serde` behind an aliased registry declaration: normal, development, build, inactive-target and optional dependencies, plus the legacy `[replace]` mechanism. Every fixture has an offline-generated lockfile, verifies that the actual checker rejects the resolved source, and checks that the lockfile remains unchanged. Unit tests cover registry acceptance, incomplete graphs, resolved source/name substitution and required workspace membership. The regular CI architecture command remains the positive test against the real workspace.
