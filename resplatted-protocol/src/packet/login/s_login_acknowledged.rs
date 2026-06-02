use crate::packet::PacketWrite;

pub struct LoginAcknowledgedPacket;

impl PacketWrite for LoginAcknowledgedPacket {
    const ID: i32 = 0x03;

    fn write(&self, _buf: &mut Vec<u8>) -> std::io::Result<()> {
        Ok(())
    }
}
