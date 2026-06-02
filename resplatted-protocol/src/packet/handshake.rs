use super::PacketWrite;
use crate::io::write::MinecraftWriteExt;
use bytes::BufMut;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum HandshakeNextState {
    Status = 1,
    Login = 2,
}

/// ## HandShake Packet
/// The first packet sent, used to ping the server or connect to it
pub struct HandshakePacket {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: HandshakeNextState,
}

impl PacketWrite for HandshakePacket {
    const ID: i32 = 0x00;

    fn write(&self, buf: &mut Vec<u8>) -> std::io::Result<()> {
        buf.write_var_int(self.protocol_version);
        buf.write_string(&self.server_address)?;
        buf.put_u16(self.server_port);
        buf.write_var_int(self.next_state as i32);
        Ok(())
    }
}
