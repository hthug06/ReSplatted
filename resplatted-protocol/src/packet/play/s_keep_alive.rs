use crate::packet::PacketWrite;
use bytes::{BufMut, BytesMut};

pub struct SPlayKeepAlivePacket {
    pub id: i64,
}

impl PacketWrite for SPlayKeepAlivePacket {
    const ID: i32 = 0x1B;

    fn write(&self, buf: &mut BytesMut) -> std::io::Result<()> {
        buf.put_i64(self.id);
        Ok(())
    }
}
