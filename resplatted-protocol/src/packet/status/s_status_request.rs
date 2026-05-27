use crate::packet::PacketWrite;

pub struct StatusRequestPacket;

impl PacketWrite for StatusRequestPacket {
    const ID: i32 = 0x00;

    fn write(&self, _buf: &mut bytes::BytesMut) -> std::io::Result<()> {
        Ok(())
    }
}
