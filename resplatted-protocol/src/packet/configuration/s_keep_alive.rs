use crate::packet::PacketWrite;
use bytes::BufMut;

pub struct SConfigurationKeepAlivePacket {
    pub id: i64,
}

impl PacketWrite for SConfigurationKeepAlivePacket {
    const ID: i32 = 0x04;

    fn write(&self, buf: &mut Vec<u8>) -> std::io::Result<()> {
        buf.put_i64(self.id);
        Ok(())
    }
}
