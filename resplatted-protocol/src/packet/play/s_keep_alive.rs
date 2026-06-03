use crate::io::write::MinecraftWriteExt;
use crate::packet::PacketWrite;

pub struct SPlayKeepAlivePacket {
    pub id: i64,
}

impl PacketWrite for SPlayKeepAlivePacket {
    const ID: i32 = 0x1C;

    fn write(&self, buf: &mut Vec<u8>) -> std::io::Result<()> {
        buf.write_primitive_type(self.id);
        Ok(())
    }
}
