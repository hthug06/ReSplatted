use crate::io::write::MinecraftWriteExt;
use crate::packet::PacketWrite;

pub struct SConfigurationKeepAlivePacket {
    pub id: i64,
}

impl PacketWrite for SConfigurationKeepAlivePacket {
    const ID: i32 = 0x04;

    fn write(&self, buf: &mut Vec<u8>) -> std::io::Result<()> {
        buf.write_primitive_type(self.id);
        Ok(())
    }
}
