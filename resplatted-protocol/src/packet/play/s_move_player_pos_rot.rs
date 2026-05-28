use crate::packet::PacketWrite;
use bytes::{BufMut, BytesMut};

pub struct MovePlayerPosRotPacket {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    /// Bit field: 0x01: on ground, 0x02: pushing against wall.
    pub flags: u8,
}

impl PacketWrite for MovePlayerPosRotPacket {
    const ID: i32 = 0x1F;

    fn write(&self, buf: &mut BytesMut) -> std::io::Result<()> {
        buf.put_f64(self.x);
        buf.put_f64(self.feet_y);
        buf.put_f64(self.z);
        buf.put_f32(self.yaw);
        buf.put_f32(self.pitch);
        buf.put_u8(self.flags);
        Ok(())
    }
}
