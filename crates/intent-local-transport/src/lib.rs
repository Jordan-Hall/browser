#![cfg_attr(not(windows), forbid(unsafe_code))]
#![deny(unsafe_op_in_unsafe_fn)]
#![doc = r#"Supervisor-bound local worker authentication for Intent Browser.

The supervisor-issued one-time bootstrap token plus exact worker instance/role is the primary
channel-binding mechanism. OS peer credentials are corroborating evidence and defense in depth;
same-user credentials alone are not treated as a strong sandbox boundary.
"#]

mod auth;
mod peer;

pub use auth::{
    AuthenticationError, BOOTSTRAP_TOKEN_BYTES, BootstrapToken, MessageFamily,
    UnboundWorkerVerifier, WorkerHello, WorkerIdentity, WorkerRole, WorkerVerifier,
    issue_worker_authentication,
};
pub use peer::{ExpectedPeer, PeerCredentialError};

#[cfg(windows)]
pub use peer::verify_named_pipe_server;
