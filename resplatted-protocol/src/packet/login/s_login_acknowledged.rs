use crate::io::ConnectionContext;
use crate::packet::PacketWrite;

pub struct LoginAcknowledgedPacket;

impl PacketWrite for LoginAcknowledgedPacket {
    fn id(_ctx: &ConnectionContext) -> i32 {
        // Same for all versions
        0x03
    }

    fn write(&self, _buf: &mut Vec<u8>, _ctx: &ConnectionContext) -> std::io::Result<()> {
        Ok(())
    }
}
