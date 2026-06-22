use crate::io::ConnectionContext;
use crate::io::read::MinecraftReadExt;
use crate::packet::PacketRead;
use std::io::Cursor;

pub struct SetCompressionPacket {
    pub threshold: i32, // a VarInt
}

impl PacketRead for SetCompressionPacket {
    fn id(_ctx: &ConnectionContext) -> i32 {
        // Same for all versions
        0x03
    }

    fn read(cursor: &mut Cursor<&[u8]>, _ctx: &ConnectionContext) -> std::io::Result<Self> {
        Ok(Self {
            threshold: cursor.read_var_int()?,
        })
    }
}
