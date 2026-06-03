use crate::io::write::MinecraftWriteExt;
use crate::packet::PacketWrite;

pub struct PingRequestPacket {
    pub timestamp: i64,
}

impl PacketWrite for PingRequestPacket {
    const ID: i32 = 0x01;

    fn write(&self, buf: &mut Vec<u8>) -> std::io::Result<()> {
        buf.write_primitive_type(self.timestamp);
        Ok(())
    }
}
