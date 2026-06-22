use crate::io::ConnectionContext;
use crate::packet::PacketRead;
use std::io::Cursor;

pub struct CKnownPackPacket;

impl PacketRead for CKnownPackPacket {
    fn id(_ctx: &ConnectionContext) -> i32 {
        // Same for all versions
        0x0E
    }

    fn read(_cursor: &mut Cursor<&[u8]>, _ctx: &ConnectionContext) -> std::io::Result<Self> {
        Ok(Self)
    }
}
