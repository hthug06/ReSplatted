use crate::io::write::MinecraftWriteExt;
use crate::io::{ConnectionContext, ProtocolState, ProtocolVersion};
use crate::packet::PacketWrite;

pub struct SKeepAlivePacket {
    pub id: i64,
}

impl PacketWrite for SKeepAlivePacket {
    fn id(ctx: &ConnectionContext) -> i32 {
        match ctx.state {
            ProtocolState::Configuration => 0x04,
            ProtocolState::Play => match ctx.version {
                ProtocolVersion::V1_21_1 => 0x26,
                ProtocolVersion::V26_1 => 0x1C,
            },
            _ => unreachable!(
                "Invalid State for serverbound keep alive packet: {:?}",
                ctx.state
            ),
        }
    }

    fn write(&self, buf: &mut Vec<u8>, _ctx: &ConnectionContext) -> std::io::Result<()> {
        buf.write_primitive_type(self.id);
        Ok(())
    }
}
