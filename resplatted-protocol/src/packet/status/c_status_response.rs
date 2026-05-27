use crate::packet::PacketRead;
use std::io::Cursor;

pub struct StatusRequestPacket {
    pub response: String,
}

impl PacketRead for StatusRequestPacket {
    const ID: i32 = 0x00;

    fn read(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        Ok(Self {
            response: cursor.read_string()?,
        })
    }
}
