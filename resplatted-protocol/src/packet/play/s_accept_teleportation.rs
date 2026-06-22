use crate::io::ConnectionContext;
use crate::io::write::MinecraftWriteExt;
use crate::packet::PacketWrite;

pub struct AcceptTeleportationPacket {
    pub teleport_id: i32, // Varint
}

impl PacketWrite for AcceptTeleportationPacket {
    fn id(_ctx: &ConnectionContext) -> i32 {
        // Same for all versions
        0x00
    }

    fn write(&self, buf: &mut Vec<u8>, _ctx: &ConnectionContext) -> std::io::Result<()> {
        buf.write_var_int(self.teleport_id);
        Ok(())
    }
}
