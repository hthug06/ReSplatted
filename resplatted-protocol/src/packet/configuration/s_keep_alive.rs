use crate::packet::PacketWrite;
use bytes::{BufMut, BytesMut};

pub struct SKeepAlive {
    pub id: i64
}

impl PacketWrite for SKeepAlive {
    const ID: i32 = 0x04;

    fn write(&self, buf: &mut BytesMut) -> std::io::Result<()> {
        buf.put_i64(self.id);
        Ok(())
    }
}