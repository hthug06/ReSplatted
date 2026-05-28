use crate::io::write::MinecraftWriteExt;
use crate::packet::PacketWrite;
use bytes::BytesMut;

pub struct AcceptTeleportationPacket {
    pub teleport_id: i32, // Varint
}

impl PacketWrite for AcceptTeleportationPacket {
    const ID: i32 = 0x00;

    fn write(&self, buf: &mut BytesMut) -> std::io::Result<()> {
        buf.write_var_int(self.teleport_id);
        Ok(())
    }
}
