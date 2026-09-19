# Dependency direction

The trusted contract layer is intentionally a leaf in the Intent Browser architecture.

## Rules

- `intent-contracts` may use authority-neutral serialization/value libraries, but it must not depend on any other workspace crate.
- `intent-contracts` must not depend on browser engines, GUI frameworks, OS automation crates, provider SDKs, or agent adapters.
- `intent-ipc` may depend on `intent-contracts`.
- Workers and brokers may depend on the IPC/contracts layers, never the reverse.
- Raw browser, operating-system, provider, or process handles must stay behind brokers and must not leak into durable contracts.

`cargo run --locked -p intent-xtask -- arch-check` enforces the contract-layer rule from Cargo metadata. The checker is fail-closed for every declared direct `intent-contracts` dependency: normal, development, build and target-specific dependencies are all subject to the reviewed allowlist and source policy, including Cargo aliases. Path/workspace and non-crates.io sources are rejected. Expanding the allowlist is an architecture change and must be reviewed.

This command deliberately does **not** claim to audit the complete transitive supply chain. Transitive packages are constrained by the committed `Cargo.lock`; changing them is reviewed through the lockfile diff and locked CI. Supply-chain provenance or vulnerability qualification is a separate gate, not an implicit property of this architecture check.

The integration regression invokes the real `arch-check` executable against an isolated Cargo workspace containing a target-specific dependency whose package is renamed to an allowed dependency name, and verifies that the checker fails. Unit tests separately exercise unknown package and unreviewed-source rejection.
