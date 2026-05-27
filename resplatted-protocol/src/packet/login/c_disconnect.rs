use crate::io::read::MinecraftReadExt;
use crate::packet::PacketRead;
use std::io::Cursor;

pub struct LoginDisconnectPacket {
    pub reason: String,
}

impl PacketRead for LoginDisconnectPacket {
    const ID: i32 = 0x00;

    fn read(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        Ok(Self {
            reason: cursor.read_string()?,
        })
    }
}
