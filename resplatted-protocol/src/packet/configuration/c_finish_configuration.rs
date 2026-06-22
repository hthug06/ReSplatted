use crate::io::ConnectionContext;
use crate::packet::PacketRead;
use std::io::Cursor;

pub struct CFinishConfigurationPacket;

impl PacketRead for CFinishConfigurationPacket {
    fn id(_ctx: &ConnectionContext) -> i32 {
        // Same for all versions
        0x03
    }

    fn read(_cursor: &mut Cursor<&[u8]>, _ctx: &ConnectionContext) -> std::io::Result<Self> {
        Ok(Self)
    }
}
