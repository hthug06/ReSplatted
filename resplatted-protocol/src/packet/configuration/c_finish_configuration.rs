use crate::packet::PacketRead;
use std::io::Cursor;

pub struct CFinishConfigurationPacket;

impl PacketRead for CFinishConfigurationPacket {
    const ID: i32 = 0x03;

    fn read(_cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        Ok(Self)
    }
}
