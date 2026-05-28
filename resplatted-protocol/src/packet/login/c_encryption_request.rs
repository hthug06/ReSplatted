use crate::packet::PacketRead;
use std::io::Cursor;

/// Used to check if the server is in online mode
pub struct EncryptionRequestPacket;

impl PacketRead for EncryptionRequestPacket {
    const ID: i32 = 0x01;

    fn read(_cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        Ok(Self)
    }
}
