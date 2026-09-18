use crate::peer::{PeerCredentialEvidence, PeerExpectation};
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
    pub fn generate() -> Result<Self, getrandom::Error> {
        let mut bytes = [0_u8; BOOTSTRAP_TOKEN_BYTES];
        getrandom::fill(&mut bytes)?;
        Ok(Self(bytes))
    }

    #[must_use]
    pub const fn from_bytes(bytes: [u8; BOOTSTRAP_TOKEN_BYTES]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub fn matches(&self, other: &Self) -> bool {
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

/// A launch record is a move-only input; one-use registry enforcement is still required.
///
/// ```compile_fail
/// use intent_local_transport::WorkerLaunchRecord;
/// fn duplicate(record: WorkerLaunchRecord) { let _ = record.clone(); }
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct WorkerLaunchRecord {
    instance_id: WorkerInstanceId,
    role: WorkerRole,
    bootstrap_token: BootstrapToken,
    peer_expectation: PeerExpectation,
}

impl WorkerLaunchRecord {
    #[must_use]
    pub const fn new(
        instance_id: WorkerInstanceId,
        role: WorkerRole,
        bootstrap_token: BootstrapToken,
        peer_expectation: PeerExpectation,
    ) -> Self {
        Self {
            instance_id,
            role,
            bootstrap_token,
            peer_expectation,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkerIdentity {
    instance_id: WorkerInstanceId,
    role: WorkerRole,
    peer: PeerCredentialEvidence,
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
    pub const fn peer(&self) -> PeerCredentialEvidence {
        self.peer
    }

    #[must_use]
    pub fn allows(&self, family: MessageFamily) -> bool {
        self.role.allows(family)
    }
}

#[derive(Debug)]
pub struct OneShotAuthenticator {
    launch: WorkerLaunchRecord,
}

impl OneShotAuthenticator {
    #[must_use]
    pub const fn new(launch: WorkerLaunchRecord) -> Self {
        Self { launch }
    }

    pub fn authenticate(
        self,
        hello: &WorkerHello,
        peer: PeerCredentialEvidence,
    ) -> Result<WorkerIdentity, AuthenticationError> {
        if hello.schema_version != SchemaVersion::V1 {
            return Err(AuthenticationError::UnsupportedSchema);
        }

        let token_matches = self.launch.bootstrap_token.matches(&hello.bootstrap_token);
        let launch_matches = token_matches
            && self.launch.instance_id == hello.instance_id
            && self.launch.role == hello.role;
        if !launch_matches {
            return Err(AuthenticationError::LaunchIdentityMismatch);
        }

        if !self.launch.peer_expectation.matches(peer) {
            return Err(AuthenticationError::PeerCredentialMismatch);
        }

        Ok(WorkerIdentity {
            instance_id: hello.instance_id,
            role: hello.role,
            peer,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthenticationError {
    UnsupportedSchema,
    LaunchIdentityMismatch,
    PeerCredentialMismatch,
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
        }
    }
}

impl Error for AuthenticationError {}

#[cfg(test)]
mod tests {
    use super::{
        AuthenticationError, BootstrapToken, MessageFamily, OneShotAuthenticator, WorkerHello,
        WorkerLaunchRecord, WorkerRole,
    };
    use crate::peer::{PeerCredentialEvidence, PeerExpectation};
    use intent_contracts::WorkerInstanceId;
    use std::error::Error;
    use std::str::FromStr;

    fn instance() -> Result<WorkerInstanceId, Box<dyn Error>> {
        Ok(WorkerInstanceId::from_str(
            "018f47f7-5a86-7c00-8000-000000000501",
        )?)
    }

    fn peer() -> PeerCredentialEvidence {
        PeerCredentialEvidence::Unix {
            pid: Some(123),
            uid: 1000,
            gid: 1000,
        }
    }

    #[test]
    fn debug_output_never_contains_bootstrap_secret() {
        let token = BootstrapToken::from_bytes([0xabu8; 32]);
        let debug = format!("{token:?}");
        assert_eq!(debug, "BootstrapToken([REDACTED])");
        assert!(!debug.contains("171"));
    }

    #[test]
    fn incorrect_token_fails_as_launch_identity_mismatch() -> Result<(), Box<dyn Error>> {
        let expected = BootstrapToken::from_bytes([1_u8; 32]);
        let supplied = BootstrapToken::from_bytes([2_u8; 32]);
        let launch = WorkerLaunchRecord::new(
            instance()?,
            WorkerRole::BrowserWorker,
            expected,
            PeerExpectation::exact(peer()),
        );
        let hello = WorkerHello::new(instance()?, WorkerRole::BrowserWorker, supplied);
        let Err(error) = OneShotAuthenticator::new(launch).authenticate(&hello, peer()) else {
            return Err("incorrect token unexpectedly authenticated".into());
        };
        assert_eq!(error, AuthenticationError::LaunchIdentityMismatch);
        Ok(())
    }

    #[test]
    fn claimed_role_must_match_supervisor_launch_record() -> Result<(), Box<dyn Error>> {
        let token = BootstrapToken::from_bytes([3_u8; 32]);
        let launch = WorkerLaunchRecord::new(
            instance()?,
            WorkerRole::PolicyBroker,
            token.clone(),
            PeerExpectation::exact(peer()),
        );
        let hello = WorkerHello::new(instance()?, WorkerRole::BrowserWorker, token);
        let Err(error) = OneShotAuthenticator::new(launch).authenticate(&hello, peer()) else {
            return Err("role mismatch unexpectedly authenticated".into());
        };
        assert_eq!(error, AuthenticationError::LaunchIdentityMismatch);
        Ok(())
    }

    #[test]
    fn peer_credentials_are_checked_after_primary_secret() -> Result<(), Box<dyn Error>> {
        let token = BootstrapToken::from_bytes([4_u8; 32]);
        let launch = WorkerLaunchRecord::new(
            instance()?,
            WorkerRole::BrowserWorker,
            token.clone(),
            PeerExpectation::exact(peer()),
        );
        let hello = WorkerHello::new(instance()?, WorkerRole::BrowserWorker, token);
        let different_peer = PeerCredentialEvidence::Unix {
            pid: Some(124),
            uid: 1000,
            gid: 1000,
        };
        let Err(error) = OneShotAuthenticator::new(launch).authenticate(&hello, different_peer)
        else {
            return Err("peer mismatch unexpectedly authenticated".into());
        };
        assert_eq!(error, AuthenticationError::PeerCredentialMismatch);
        Ok(())
    }

    #[test]
    fn successful_authentication_binds_role_and_instance() -> Result<(), Box<dyn Error>> {
        let token = BootstrapToken::from_bytes([5_u8; 32]);
        let launch = WorkerLaunchRecord::new(
            instance()?,
            WorkerRole::BrowserWorker,
            token.clone(),
            PeerExpectation::exact(peer()),
        );
        let hello = WorkerHello::new(instance()?, WorkerRole::BrowserWorker, token);
        let identity = OneShotAuthenticator::new(launch).authenticate(&hello, peer())?;

        assert_eq!(identity.instance_id(), instance()?);
        assert_eq!(identity.role(), WorkerRole::BrowserWorker);
        assert!(identity.allows(MessageFamily::BrowserObservation));
        assert!(!identity.allows(MessageFamily::PolicyDecision));
        Ok(())
    }
}
