use crate::peer::{ExpectedPeer, ObservedPeer, PeerCredentialError};
use intent_contracts::{SchemaVersion, WorkerInstanceId};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use subtle::ConstantTimeEq;

pub const BOOTSTRAP_TOKEN_BYTES: usize = 32;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BootstrapToken([u8; BOOTSTRAP_TOKEN_BYTES]);

impl BootstrapToken {
    fn matches(&self, other: &Self) -> bool {
        bool::from(self.0.as_slice().ct_eq(other.0.as_slice()))
    }
}

impl fmt::Debug for BootstrapToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("BootstrapToken([REDACTED])")
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerRole {
    BrowserWorker,
    PolicyBroker,
    StateStore,
    ConnectorHost,
    ModelHost,
    AgentAdapter,
    UiRenderer,
    DesktopBroker,
    ExtensionHost,
    FixtureWorker,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageFamily {
    BrowserObservation,
    PolicyDecision,
    StateMutation,
    ConnectorCall,
    ModelInference,
    AgentSession,
    UiRender,
    DesktopAction,
    ExtensionCall,
    LifecycleControl,
}

impl WorkerRole {
    #[must_use]
    pub fn allows(self, family: MessageFamily) -> bool {
        match self {
            Self::BrowserWorker => matches!(
                family,
                MessageFamily::BrowserObservation | MessageFamily::LifecycleControl
            ),
            Self::PolicyBroker => matches!(
                family,
                MessageFamily::PolicyDecision | MessageFamily::LifecycleControl
            ),
            Self::StateStore => matches!(
                family,
                MessageFamily::StateMutation | MessageFamily::LifecycleControl
            ),
            Self::ConnectorHost => matches!(
                family,
                MessageFamily::ConnectorCall | MessageFamily::LifecycleControl
            ),
            Self::ModelHost => matches!(
                family,
                MessageFamily::ModelInference | MessageFamily::LifecycleControl
            ),
            Self::AgentAdapter => matches!(
                family,
                MessageFamily::AgentSession | MessageFamily::LifecycleControl
            ),
            Self::UiRenderer => {
                matches!(
                    family,
                    MessageFamily::UiRender | MessageFamily::LifecycleControl
                )
            }
            Self::DesktopBroker => matches!(
                family,
                MessageFamily::DesktopAction | MessageFamily::LifecycleControl
            ),
            Self::ExtensionHost => matches!(
                family,
                MessageFamily::ExtensionCall | MessageFamily::LifecycleControl
            ),
            Self::FixtureWorker => family == MessageFamily::LifecycleControl,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkerHello {
    schema_version: SchemaVersion,
    instance_id: WorkerInstanceId,
    role: WorkerRole,
    bootstrap_token: BootstrapToken,
}

impl WorkerHello {
    #[must_use]
    pub const fn new(
        instance_id: WorkerInstanceId,
        role: WorkerRole,
        bootstrap_token: BootstrapToken,
    ) -> Self {
        Self {
            schema_version: SchemaVersion::V1,
            instance_id,
            role,
            bootstrap_token,
        }
    }

    #[must_use]
    pub const fn instance_id(&self) -> WorkerInstanceId {
        self.instance_id
    }

    #[must_use]
    pub const fn role(&self) -> WorkerRole {
        self.role
    }
}

#[derive(Debug)]
struct WorkerLaunch {
    instance_id: WorkerInstanceId,
    role: WorkerRole,
    bootstrap_token: BootstrapToken,
}

/// Issued before spawn, then bound once to the actual child's process policy.
///
/// ```compile_fail
/// use intent_local_transport::UnboundWorkerVerifier;
/// fn duplicate(verifier: UnboundWorkerVerifier) { let _ = verifier.clone(); }
/// ```
/// ```compile_fail
/// use intent_local_transport::UnboundWorkerVerifier;
/// fn deserializable<T: serde::de::DeserializeOwned>() {}
/// deserializable::<UnboundWorkerVerifier>();
/// ```
#[derive(Debug)]
pub struct UnboundWorkerVerifier {
    launch: WorkerLaunch,
}

pub fn issue_worker_authentication(
    instance_id: WorkerInstanceId,
    role: WorkerRole,
) -> Result<(BootstrapToken, UnboundWorkerVerifier), getrandom::Error> {
    let mut bytes = [0_u8; BOOTSTRAP_TOKEN_BYTES];
    getrandom::fill(&mut bytes)?;
    let token = BootstrapToken(bytes);
    let verifier = UnboundWorkerVerifier {
        launch: WorkerLaunch {
            instance_id,
            role,
            bootstrap_token: token.clone(),
        },
    };
    Ok((token, verifier))
}

impl UnboundWorkerVerifier {
    #[must_use]
    pub fn bind(self, expected: ExpectedPeer) -> WorkerVerifier {
        WorkerVerifier {
            launch: Some(self.launch),
            expected,
        }
    }
}

/// The caller retains the connection and decodes its Hello. This result cannot
/// be installed into the supervisor or converted into a runtime lease.
///
/// ```compile_fail
/// use intent_contracts::WorkerInstanceId;
/// use intent_local_transport::{WorkerIdentity, WorkerRole};
/// fn forge(instance_id: WorkerInstanceId, role: WorkerRole) -> WorkerIdentity {
///     WorkerIdentity { instance_id, role }
/// }
/// ```
/// ```compile_fail
/// use intent_local_transport::WorkerIdentity;
/// fn deserializable<T: serde::de::DeserializeOwned>() {}
/// deserializable::<WorkerIdentity>();
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkerIdentity {
    instance_id: WorkerInstanceId,
    role: WorkerRole,
}

impl WorkerIdentity {
    #[must_use]
    pub const fn instance_id(&self) -> WorkerInstanceId {
        self.instance_id
    }

    #[must_use]
    pub const fn role(&self) -> WorkerRole {
        self.role
    }

    #[must_use]
    pub fn allows(&self, family: MessageFamily) -> bool {
        self.role.allows(family)
    }
}

/// Only issuance can create this verifier; wire proof cannot recreate it.
///
/// ```compile_fail
/// use intent_local_transport::WorkerVerifier;
/// fn duplicate(verifier: WorkerVerifier) { let _ = verifier.clone(); }
/// ```
/// ```compile_fail
/// use intent_local_transport::{BootstrapToken, WorkerVerifier};
/// fn recreate(token: BootstrapToken) -> WorkerVerifier { WorkerVerifier::new(token) }
/// ```
/// ```compile_fail
/// use intent_local_transport::WorkerVerifier;
/// fn deserializable<T: serde::de::DeserializeOwned>() {}
/// deserializable::<WorkerVerifier>();
/// ```
#[derive(Debug)]
pub struct WorkerVerifier {
    launch: Option<WorkerLaunch>,
    expected: ExpectedPeer,
}

impl WorkerVerifier {
    /// Checks the connected process before reading Hello without consuming the verifier.
    #[cfg(unix)]
    pub fn check_unix_peer(
        &self,
        stream: &std::os::unix::net::UnixStream,
    ) -> Result<(), AuthenticationError> {
        self.require_active()?;
        self.check_observed(crate::peer::unix_peer_credentials(stream))
    }

    /// Authenticates a Hello decoded from this connection, which the caller must retain.
    /// A matching peer consumes the verifier even when Hello validation fails.
    /// Peer mismatch or observation failure does not consume an active verifier.
    #[cfg(unix)]
    pub fn authenticate_unix(
        &mut self,
        hello: &WorkerHello,
        stream: &std::os::unix::net::UnixStream,
    ) -> Result<WorkerIdentity, AuthenticationError> {
        self.check_unix_peer(stream)?;
        self.consume_hello(hello)
    }

    /// Checks the connected client before reading Hello without consuming the verifier.
    #[cfg(windows)]
    pub fn check_named_pipe_client<H: std::os::windows::io::AsRawHandle>(
        &self,
        pipe: &H,
    ) -> Result<(), AuthenticationError> {
        self.require_active()?;
        self.check_observed(crate::peer::named_pipe_client_credentials(pipe))
    }

    /// Authenticates a Hello decoded from this pipe, which the caller must retain.
    /// A matching peer consumes the verifier even when Hello validation fails.
    /// Peer mismatch or observation failure does not consume an active verifier.
    #[cfg(windows)]
    pub fn authenticate_named_pipe_client<H: std::os::windows::io::AsRawHandle>(
        &mut self,
        hello: &WorkerHello,
        pipe: &H,
    ) -> Result<WorkerIdentity, AuthenticationError> {
        self.check_named_pipe_client(pipe)?;
        self.consume_hello(hello)
    }

    fn require_active(&self) -> Result<(), AuthenticationError> {
        self.launch
            .as_ref()
            .ok_or(AuthenticationError::AlreadyConsumed)
            .map(|_| ())
    }

    fn check_observed(
        &self,
        observed: Result<ObservedPeer, PeerCredentialError>,
    ) -> Result<(), AuthenticationError> {
        let observed = observed.map_err(AuthenticationError::PeerObservation)?;
        if !self.expected.matches(observed) {
            return Err(AuthenticationError::PeerCredentialMismatch);
        }
        Ok(())
    }

    fn consume_hello(
        &mut self,
        hello: &WorkerHello,
    ) -> Result<WorkerIdentity, AuthenticationError> {
        let launch = self
            .launch
            .take()
            .ok_or(AuthenticationError::AlreadyConsumed)?;
        if hello.schema_version != SchemaVersion::V1 {
            return Err(AuthenticationError::UnsupportedSchema);
        }
        let token_matches = launch.bootstrap_token.matches(&hello.bootstrap_token);
        if !token_matches || launch.instance_id != hello.instance_id || launch.role != hello.role {
            return Err(AuthenticationError::LaunchIdentityMismatch);
        }
        Ok(WorkerIdentity {
            instance_id: launch.instance_id,
            role: launch.role,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthenticationError {
    UnsupportedSchema,
    LaunchIdentityMismatch,
    PeerCredentialMismatch,
    PeerObservation(PeerCredentialError),
    AlreadyConsumed,
}

impl fmt::Display for AuthenticationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchema => formatter.write_str("unsupported worker handshake schema"),
            Self::LaunchIdentityMismatch => {
                formatter.write_str("worker launch identity did not match supervisor record")
            }
            Self::PeerCredentialMismatch => {
                formatter.write_str("worker OS peer credentials did not match launch expectation")
            }
            Self::PeerObservation(error) => {
                write!(formatter, "worker OS peer observation failed: {error}")
            }
            Self::AlreadyConsumed => formatter.write_str("worker verifier was already consumed"),
        }
    }
}

impl Error for AuthenticationError {}

#[cfg(test)]
mod tests;
