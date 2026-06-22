use crate::io::ConnectionContext;
use crate::io::read::MinecraftReadExt;
use crate::packet::PacketRead;

pub struct PongResponsePacket {
    pub timestamp: i64,
}

impl PacketRead for PongResponsePacket {
    fn id(_ctx: &ConnectionContext) -> i32 {
        // Same for all versions
        0x01
    }

    fn read(
        cursor: &mut std::io::Cursor<&[u8]>,
        _ctx: &ConnectionContext,
    ) -> std::io::Result<Self> {
        Ok(Self {
            timestamp: cursor.read_i64()?,
        })
    }
}
