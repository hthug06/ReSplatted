use crate::packet::PacketWrite;
use bytes::BytesMut;

pub struct LoginAcknowledgedPacket;

impl PacketWrite for LoginAcknowledgedPacket {
    const ID: i32 = 0x03;

    fn write(&self, _buf: &mut BytesMut) -> std::io::Result<()> {
        Ok(())
    }
}
