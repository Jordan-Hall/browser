# Bootstrap storage disposal

Issued and deserialized bootstrap tokens erase their owned arrays on drop.
Entropy fills an already guarded token; partial token deserialization owns the
same erasing value before accepting its first byte. Existing verifier ownership
therefore disposes of consumed, rejected and abandoned credentials.

Bootstrap and Hello output use one bounded, preallocated SecretFrame. It retains
ownership through a consuming write and erases storage on success or error.
Serialization uses the existing strict validation and frame header construction.
It cannot grow the output allocation after secret serialization starts.

The supervisor consumes decoded frame payloads through the same strict parser,
then erases the input. Blocking reads guard partially received payloads from
allocation onward. Incremental decoders erase partial payloads on reset or drop.
Socket buffers erase consumed ranges promptly and preserve unread bytes: a
Ready message coalesced after Hello remains available for normal admission.
Stopping an unauthenticated lane drops its socket and partial input immediately;
authenticated cancellation and draining continue through the existing path.
Reaping an exited child releases both lane verifiers and sockets immediately,
including a child that exits before either handshake finishes.

The private erasing byte owner uses zeroize for initialized bytes and the full
current vector capacity. Test-only callbacks inspect live initialized storage
after erasure and before deallocation; they retain only byte counts and whether
the bytes were zero. They never read freed allocations. Capacity erasure follows
the primitive's implementation contract rather than a claim to measure freed
memory. See [zeroize's guarantees and limitations](https://docs.rs/zeroize/1.9.0/zeroize/).

This is best-effort erasure of these application-owned arrays and wire buffers.
The existing JSON Value tree, serde temporaries, diagnostics for malformed input,
compiler moves and spills, other allocator history, kernel pipe/socket buffers,
swap and dumps remain outside that claim. Generic public serialization and
fixture replay modes can create further copies. No new parser or all-memory
erasure guarantee is introduced.

Full Windows/macOS supervisor authentication and independent CORE-01.T05
acceptance remain open. Local verification and published CI are tracked
separately; neither a token unit test nor a portable job qualifies an absent
platform supervisor.

Local verification includes 218 passing Linux test results, 59 native Windows
unit tests, real Windows pipe cases, strict Clippy and compile-fail API checks.
Removing token or receive erasure makes its live-storage test fail; erasing the
whole socket input breaks the following Ready bytes. All three focused cases
pass with the exact production source restored. See [the scoped report](verification/core-bootstrap-secret-disposal-20260920.json).

The full Linux run preceded the final direct-child-exit cleanup. Its regression
first failed on retained credentials after a real child exit. The final
supervisor and fixture run checks that cleanup and the existing process lifecycle
behavior; the report distinguishes the source covered by each run.

Published source `f455135` in PR #856 passed all four CI jobs in run
35528056971. Its 303 archived source files and all native allocation artifacts
match the qualified source. Run 35528057025 completed three bounded ASan parser
fuzz executions. See [the published report](verification/core-bootstrap-secret-disposal-ci-20260920.json).
