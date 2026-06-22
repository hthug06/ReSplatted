use crate::io::ConnectionContext;
use crate::packet::PacketWrite;

pub struct SFinishConfigurationPacket;

impl PacketWrite for SFinishConfigurationPacket {
    fn id(_ctx: &ConnectionContext) -> i32 {
        // Same for all versions
        0x03
    }

    fn write(&self, _buf: &mut Vec<u8>, _ctx: &ConnectionContext) -> std::io::Result<()> {
        Ok(())
    }
}
