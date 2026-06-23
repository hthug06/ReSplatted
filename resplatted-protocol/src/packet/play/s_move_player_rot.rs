use crate::io::write::MinecraftWriteExt;
use crate::io::{ConnectionContext, ProtocolVersion};
use crate::packet::PacketWrite;

pub struct MovePlayerRotPacket {
    pub yaw: f32,
    pub pitch: f32,
    /// Bit field: 0x01: on ground, 0x02: pushing against wall.
    pub flags: u8,
}

impl PacketWrite for MovePlayerRotPacket {
    fn id(ctx: &ConnectionContext) -> i32 {
        match ctx.version {
            ProtocolVersion::V1_21_1 => 0x1C,
            ProtocolVersion::V26_1 => 0x20,
        }
    }

    fn write(&self, buf: &mut Vec<u8>, ctx: &ConnectionContext) -> std::io::Result<()> {
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
