# CORE v1 wire compatibility

The v1 framing and JSON contracts are explicit. The current implementation accepts
valid existing fixtures while narrowing malformed or ambiguous input. This is
pre-freeze schema 1.0 work; stricter rejection is not transparent compatibility
with an older parser that silently discarded fields or resumed a damaged stream.

## Frame and stream contract

A frame starts with one lane byte followed by a four-byte, unsigned big-endian
payload length. Lane 1 is control; lane 2 is artifact. Other lanes fail with
`InvalidLane`. The decoder validates the lane-specific length before reserving
payload storage. It emits complete frames only, reports the consumed input offset
and stops when the configured per-feed frame budget is reached.

Callers must retain unconsumed input and call `finish` at EOF. A partial header or
payload at EOF is `InvalidEnvelope`. A framing failure poisons the decoder: the
connection must be discarded. If one push contains valid, malformed and trailing
valid data, the failed push exposes no successful batch. Earlier successful calls
are not rolled back. Older implementations lacking poison or EOF handling do not
satisfy this receiver contract even though valid frame bytes remain unchanged.

Default limits are 1 MiB per control payload, 16 MiB per artifact payload, depth
64, 16,384 entries per collection, 65,536 JSON nodes and 256 frames per feed.
These are independent ceilings, not a guarantee that every value below its byte
ceiling fits all structural budgets. Supervisor polling additionally shares a
32 KiB read allowance across ready connections. That scheduling budget is not a
wire-format field or an aggregate process memory limit.

## Envelope and JSON contract

Control envelopes retain schema version, trace identity, message kind and typed
payload. Request and response kinds include `request_id`; events have no request
identity. Optional cancellation identity and deadline retain their existing
default-to-absent behavior. Transporting a deadline or correlation identifier does
not authorize execution or prove a response belongs to an outstanding request;
the caller owns those runtime checks.

The exact kind forms remain `{"kind":"request","request_id":"..."}`,
`{"kind":"response","request_id":"..."}` and `{"kind":"event"}`. Event
imports now reject additional message members, including a nested `request_id`,
instead of discarding them. The public Rust `EnvelopeKind::Event` unit variant
and its serialization remain unchanged. The private strict decoder follows the
same boundary pattern used by deterministic evidence.

Byte/depth/collection/node checks and strict JSON parsing precede typed payload
conversion. Schema classification precedes conversion to the requested payload
type. Duplicate keys, including equivalent escaped spellings, trailing JSON and
unknown envelope/message fields are rejected. This does not promise that every
arbitrary caller-defined payload type rejects unknown fields.

Wire error codes serialize as the exact textual names in
`fixtures/core/wire-errors-v1.json`. Their Rust numeric discriminants are local
diagnostics, not another accepted JSON representation. Money minor units retain
their textual i128 representation; the envelope codec preserves the full domain.

## Reviewed evidence and limits

The frame, error and limit implementations remain unchanged by the event parser
fix. Existing record/error fixture tests and exact kind-form regressions exercise
the supported v1 corpus. This is not a claim of interoperability with an arbitrary
historical binary or transparent acceptance of previously ignored extensions.

`frame.rs` tests cover fragmented input, one-over lengths, lane isolation, poisoned
batches and truncated EOF. `hardening.rs` covers exact byte limits, schema-first
errors, duplicate keys, every message kind and strict nested event fields.
`encoding_limits.rs` checks encoder/decoder agreement under structural limits.
`record_codec.rs` exercises the full/minimal record corpus and pinned error names.
The separate allocation probe retains native measured bounds and a materializing
positive control; its scope is documented in `core-encoder-allocation.md`.

These deterministic checks and finite sanitizer fuzz runs support CORE-01.T04
acceptance. They do not exhaust all inputs, contain arbitrary allocations inside
caller serializers, establish hostile-worker isolation or establish production
readiness.
