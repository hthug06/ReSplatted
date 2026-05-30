use crate::packet::PacketWrite;
use bytes::{BufMut, BytesMut};

pub struct MovePlayerRotPacket {
    pub yaw: f32,
    pub pitch: f32,
    /// Bit field: 0x01: on ground, 0x02: pushing against wall.
    pub flags: u8,
}

impl PacketWrite for MovePlayerRotPacket {
    const ID: i32 = 0x20;

    fn write(&self, buf: &mut BytesMut) -> std::io::Result<()> {
        buf.put_f32(self.yaw);
        buf.put_f32(self.pitch);
        buf.put_u8(self.flags);
        Ok(())
    }
}
