pub mod read;
pub mod write;
mod write_primitive;

/// The protocol version of the client
/// Used to handle packets differently
/// ex: The chunk packet is different in 1.21.1 and 26.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ProtocolVersion {
    V1_21_1 = 767,
    V26_1 = 775,
}

impl ProtocolVersion {
    pub fn from_protocol_version(version: i32) -> Self {
        match version {
            767 => ProtocolVersion::V1_21_1,
            775 => ProtocolVersion::V26_1,
            _ => unimplemented!("Protocol version {} is not supported", version),
        }
    }
}

/// The state of the connection
/// Used for common packets like the disconnect packet
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
}

/// Global context of the connection
#[derive(Debug, Clone)]
pub struct ConnectionContext {
    pub version: ProtocolVersion,
    pub state: ProtocolState,
}
