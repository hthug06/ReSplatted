use crate::io::ConnectionContext;
use crate::io::write::MinecraftWriteExt;
use crate::packet::PacketWrite;

pub struct PingRequestPacket {
    pub timestamp: i64,
}

impl PacketWrite for PingRequestPacket {
    fn id(_ctx: &ConnectionContext) -> i32 {
        // Same for all versions
        0x01
    }

    fn write(&self, buf: &mut Vec<u8>, _ctx: &ConnectionContext) -> std::io::Result<()> {
        buf.write_primitive_type(self.timestamp);
        Ok(())
    }
}
