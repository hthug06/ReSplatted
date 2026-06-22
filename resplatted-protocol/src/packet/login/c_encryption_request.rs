use crate::io::ConnectionContext;
use crate::packet::PacketRead;
use std::io::Cursor;

/// Used to check if the server is in online mode
pub struct EncryptionRequestPacket;

impl PacketRead for EncryptionRequestPacket {
    fn id(_ctx: &ConnectionContext) -> i32 {
        // Same for all versions
        0x01
    }

    fn read(_cursor: &mut Cursor<&[u8]>, _ctx: &ConnectionContext) -> std::io::Result<Self> {
        Ok(Self)
    }
}
