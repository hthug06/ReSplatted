use crate::io::write::MinecraftWriteExt;
use crate::io::{ConnectionContext, ProtocolVersion};
use crate::packet::PacketWrite;

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
    fn id(ctx: &ConnectionContext) -> i32 {
        match ctx.version {
            ProtocolVersion::V1_21_1 => 0x1B,
            ProtocolVersion::V26_1 => 0x1F,
        }
    }

    fn write(&self, buf: &mut Vec<u8>, ctx: &ConnectionContext) -> std::io::Result<()> {
        buf.write_primitive_type(self.x);
        buf.write_primitive_type(self.feet_y);
        buf.write_primitive_type(self.z);
        buf.write_primitive_type(self.yaw);
        buf.write_primitive_type(self.pitch);
        match ctx.version {
            // Only the onGround is used in 1.21.1 (bool)
            ProtocolVersion::V1_21_1 => buf.write_primitive_type((self.flags & 0x01) != 0),
            ProtocolVersion::V26_1 => buf.write_primitive_type(self.flags),
        }
        Ok(())
    }
}
