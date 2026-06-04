use crate::io::write::MinecraftWriteExt;
use crate::packet::PacketWrite;

pub struct MovePlayerRotPacket {
    pub yaw: f32,
    pub pitch: f32,
    /// Bit field: 0x01: on ground, 0x02: pushing against wall.
    pub flags: u8,
}

impl PacketWrite for MovePlayerRotPacket {
    const ID: i32 = 0x20;

    fn write(&self, buf: &mut Vec<u8>) -> std::io::Result<()> {
        buf.write_primitive_type(self.yaw);
        buf.write_primitive_type(self.pitch);
        buf.write_primitive_type(self.flags);
        Ok(())
    }
}
