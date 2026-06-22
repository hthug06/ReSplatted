use crate::io::ConnectionContext;
use crate::packet::PacketWrite;

pub struct StatusRequestPacket;

impl PacketWrite for StatusRequestPacket {
    fn id(_ctx: &ConnectionContext) -> i32 {
        // Same for all versions
        0x00
    }

    fn write(&self, _buf: &mut Vec<u8>, _ctx: &ConnectionContext) -> std::io::Result<()> {
        Ok(())
    }
}
