use crate::packet::PacketRead;

pub struct PongResponsePacket {
    pub timestamp: i64,
}

impl PacketRead for PongResponsePacket {
    const ID: i32 = 0x01;

    fn read(cursor: &mut std::io::Cursor<&[u8]>) -> std::io::Result<Self> {
        Ok(Self {
            timestamp: cursor.read_i64()?,
        })
    }
}
