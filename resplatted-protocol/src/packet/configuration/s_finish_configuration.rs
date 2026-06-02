use crate::packet::PacketWrite;

pub struct SFinishConfigurationPacket;

impl PacketWrite for SFinishConfigurationPacket {
    const ID: i32 = 0x03;

    fn write(&self, _buf: &mut Vec<u8>) -> std::io::Result<()> {
        Ok(())
    }
}
