# Dependency direction

The trusted contract layer is intentionally a leaf in the Intent Browser architecture.

## Rules

- `intent-contracts` may use authority-neutral serialization/value libraries, but it must not depend on any other workspace crate.
- `intent-contracts` must not depend on browser engines, GUI frameworks, OS automation crates, provider SDKs, or agent adapters.
- `intent-ipc` may depend on `intent-contracts`.
- Workers and brokers may depend on the IPC/contracts layers, never the reverse.
- Raw browser, operating-system, provider, or process handles must stay behind brokers and must not leak into durable contracts.

`cargo run -p intent-xtask -- arch-check` enforces the contract-layer rule from Cargo metadata. The checker fails closed for path/workspace dependencies and known browser/UI/OS/provider dependency families. Its unit tests include deliberately forbidden dependencies so CI proves the rule itself is active.
