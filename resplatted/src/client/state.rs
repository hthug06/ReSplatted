#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
pub enum ProtocolState {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
}
