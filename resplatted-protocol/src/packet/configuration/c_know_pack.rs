use crate::packet::PacketRead;
use std::io::Cursor;

pub struct CKnownPackPacket;

impl PacketRead for CKnownPackPacket {
    const ID: i32 = 0x0E;

    fn read(_cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        Ok(Self)
    }
}
