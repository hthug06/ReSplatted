use crate::packet::PacketWrite;
use bytes::BufMut;

pub struct PingRequestPacket {
    pub timestamp: i64,
}

impl PacketWrite for PingRequestPacket {
    const ID: i32 = 0x01;

    fn write(&self, buf: &mut Vec<u8>) -> std::io::Result<()> {
        buf.put_i64(self.timestamp);
        Ok(())
    }
}
