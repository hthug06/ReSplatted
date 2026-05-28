use crate::io::read::MinecraftReadExt;
use crate::packet::PacketRead;
use std::io::Cursor;

pub struct CPlayKeepAlivePacket {
    pub id: i64,
}

impl PacketRead for CPlayKeepAlivePacket {
    const ID: i32 = 0x2C;

    fn read(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        Ok(Self {
            id: cursor.read_i64()?,
        })
    }
}
