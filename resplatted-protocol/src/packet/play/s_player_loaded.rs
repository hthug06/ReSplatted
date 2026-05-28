use bytes::BytesMut;
use crate::packet::PacketWrite;

/// This packet is sent when the loading screen has closed
pub struct PlayerLoadedPacket;

impl PacketWrite for PlayerLoadedPacket {
    const ID: i32 = 0x2B;

    fn write(&self, _buf: &mut BytesMut) -> std::io::Result<()> {
        Ok(())
    }
}