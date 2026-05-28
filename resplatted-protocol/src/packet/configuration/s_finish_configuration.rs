use crate::packet::PacketWrite;
use bytes::BytesMut;

pub struct SFinishConfigurationPacket;

impl PacketWrite for SFinishConfigurationPacket {
    const ID: i32 = 0x03;

    fn write(&self, _buf: &mut BytesMut) -> std::io::Result<()> {
        Ok(())
    }
}
