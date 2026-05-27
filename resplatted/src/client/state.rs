#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolState {
    Handshake,
    Status,
    Login,
    Play,
}
