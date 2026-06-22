use crate::io::ConnectionContext;
use crate::io::write::MinecraftWriteExt;
use crate::packet::PacketWrite;

/// For this packet, we're just going to send a varint with 0 to say: 'I don't know any pack'
pub struct SKnownPackPacket;

impl PacketWrite for SKnownPackPacket {
    fn id(_ctx: &ConnectionContext) -> i32 {
        // Same for all versions
        0x07
    }

    fn write(&self, buf: &mut Vec<u8>, _ctx: &ConnectionContext) -> std::io::Result<()> {
        buf.write_var_int(0);
        Ok(())
    }
}
