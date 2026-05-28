use crate::io::write::MinecraftWriteExt;
use crate::packet::PacketWrite;
use bytes::BytesMut;

/// For this packet, we're just going to send a varint with 0 to say: 'I don't know any pack'
pub struct SKnownPackPacket;

impl PacketWrite for SKnownPackPacket {
    const ID: i32 = 0x07;

    fn write(&self, buf: &mut BytesMut) -> std::io::Result<()> {
        buf.write_var_int(0);
        Ok(())
    }
}
