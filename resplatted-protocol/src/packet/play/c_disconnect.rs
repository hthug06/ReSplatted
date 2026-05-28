use crate::io::read::MinecraftReadExt;
use crate::packet::PacketRead;
use std::io::Cursor;

pub struct PlayDisconnectPacket {
    pub reason: String,
}

impl PacketRead for PlayDisconnectPacket {
    const ID: i32 = 0x20;

    fn read(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        Ok(Self {
            reason: cursor.read_string()?,
        })
    }
}
