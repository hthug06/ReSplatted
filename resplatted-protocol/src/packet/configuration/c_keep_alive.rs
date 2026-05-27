use std::io::Cursor;
use crate::io::read::MinecraftReadExt;
use crate::packet::PacketRead;

pub struct CKeepAlive{
    pub id: i64
}

impl PacketRead for CKeepAlive {
    const ID: i32 = 0x04;

    fn read(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        Ok(Self{
            id: cursor.read_i64()?,
        })
    }
}