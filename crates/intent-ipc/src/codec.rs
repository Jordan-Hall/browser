use crate::{
    Envelope, Frame, NegotiatedProtocol, NegotiationError, ProtocolOffer, ProtocolRange, WireError,
    WireLimits, decode_control, encode_control, negotiate_protocol,
};
use serde::{Serialize, de::DeserializeOwned};

/// A selected, implemented control codec. This is not an authentication credential.
/// The generic range negotiator must not be used to advertise unimplemented codecs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlCodec {
    protocol: NegotiatedProtocol,
}

impl ControlCodec {
    pub fn negotiate(remote: &ProtocolOffer) -> Result<Self, NegotiationError> {
        let range = ProtocolRange::try_new(1, 0, 0).map_err(|_| NegotiationError::InvalidOffer)?;
        let local =
            ProtocolOffer::try_new(vec![range], []).map_err(|_| NegotiationError::InvalidOffer)?;
        let protocol = negotiate_protocol(&local, remote)?;
        Ok(Self { protocol })
    }

    #[must_use]
    pub const fn protocol(&self) -> &NegotiatedProtocol {
        &self.protocol
    }

    pub fn encode<T: Serialize>(
        &self,
        envelope: &Envelope<T>,
        limits: WireLimits,
    ) -> Result<Frame, WireError> {
        encode_control(envelope, limits)
    }

    pub fn decode<T: DeserializeOwned>(
        &self,
        frame: &Frame,
        limits: WireLimits,
    ) -> Result<Envelope<T>, WireError> {
        decode_control(frame, limits)
    }
}
