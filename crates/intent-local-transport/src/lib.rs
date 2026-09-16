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
    OneShotAuthenticator, WorkerHello, WorkerIdentity, WorkerLaunchRecord, WorkerRole,
};
pub use peer::{PeerCredentialError, PeerCredentialEvidence, PeerExpectation};

#[cfg(unix)]
pub use peer::anonymous_unix_channel_pair;
#[cfg(unix)]
pub use peer::unix_peer_credentials;
#[cfg(windows)]
pub use peer::named_pipe_client_credentials;
