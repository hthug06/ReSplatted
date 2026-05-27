use crate::io::read::MinecraftReadExt;
use crate::packet::PacketRead;
use std::io::Cursor;

pub struct ConfigurationDisconnectPacket {
    pub reason: String,
}

impl PacketRead for ConfigurationDisconnectPacket {
    const ID: i32 = 0x02;

    fn read(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        Ok(Self {
            reason: cursor.read_string()?,
        })
    }
}
