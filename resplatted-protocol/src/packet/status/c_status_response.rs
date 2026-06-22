use crate::io::ConnectionContext;
use crate::io::read::MinecraftReadExt;
use crate::packet::PacketRead;
use std::io::Cursor;

pub struct StatusResponsePacket {
    pub response: String,
}

impl PacketRead for StatusResponsePacket {
    fn id(_ctx: &ConnectionContext) -> i32 {
        // Same for all versions
        0x00
    }

    fn read(cursor: &mut Cursor<&[u8]>, _ctx: &ConnectionContext) -> std::io::Result<Self> {
        Ok(Self {
            response: cursor.read_string()?,
        })
    }
}
