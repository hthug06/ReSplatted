use crate::io::{ConnectionContext, ProtocolVersion};
use crate::packet::PacketWrite;

/// This packet is sent when the loading screen has closed
pub struct PlayerLoadedPacket;

impl PacketWrite for PlayerLoadedPacket {
    fn id(ctx: &ConnectionContext) -> i32 {
        if ctx.version == ProtocolVersion::V26_1 {
            0x2B
        } else {
            unreachable!(
                "PlayerLoadedPacket is only valid for protocol version 26.1, but got {:?}",
                ctx.version
            );
        }
    }

    fn write(&self, _buf: &mut Vec<u8>, _ctx: &ConnectionContext) -> std::io::Result<()> {
        Ok(())
    }
}
