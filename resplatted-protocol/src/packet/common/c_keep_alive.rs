use crate::io::read::MinecraftReadExt;
use crate::io::{ConnectionContext, ProtocolState, ProtocolVersion};
use crate::packet::PacketRead;
use std::io::Cursor;

pub struct CKeepAlivePacket {
    pub id: i64,
}

impl PacketRead for CKeepAlivePacket {
    fn id(ctx: &ConnectionContext) -> i32 {
        match ctx.state {
            ProtocolState::Configuration => 0x04,
            ProtocolState::Play => match ctx.version {
                ProtocolVersion::V1_21_1 => 0x26,
                ProtocolVersion::V26_1 => 0x2C,
            },
            _ => unreachable!(
                "Invalid State for clientbound keep alive packet: {:?}",
                ctx.state
            ),
        }
    }

    fn read(cursor: &mut Cursor<&[u8]>, _ctx: &ConnectionContext) -> std::io::Result<Self> {
        Ok(Self {
            id: cursor.read_i64()?,
        })
    }
}
