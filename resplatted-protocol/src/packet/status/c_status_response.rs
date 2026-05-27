use crate::io::read::MinecraftReadExt;
use crate::packet::PacketRead;
use std::io::Cursor;

pub struct StatusResponsePacket {
    pub response: String,
}

impl PacketRead for StatusResponsePacket {
    const ID: i32 = 0x00;

    fn read(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        Ok(Self {
            response: cursor.read_string()?,
        })
    }
}
