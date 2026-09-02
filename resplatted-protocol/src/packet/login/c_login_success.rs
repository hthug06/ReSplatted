use crate::io::read::MinecraftReadExt;
use crate::io::{ConnectionContext, ProtocolVersion};
use crate::packet::PacketRead;
use crate::types::game_profile::GameProfile;
use std::io::Cursor;
use uuid::Uuid;

#[derive(Debug)]
pub struct LoginSuccessPacket {
    pub game_profile: GameProfile,
    /// 26.2 +
    pub uuid: Option<Uuid>,
}

impl PacketRead for LoginSuccessPacket {
    fn id(_ctx: &ConnectionContext) -> i32 {
        // Same for all versions
        0x02
    }

    fn read(cursor: &mut Cursor<&[u8]>, ctx: &ConnectionContext) -> std::io::Result<Self> {
        // read in order
        let game_profile = GameProfile::read(cursor, ctx)?;

        let uuid = if ctx.version >= ProtocolVersion::V26_2 {
            Some(cursor.read_uuid()?)
        } else {
            None
        };

        Ok(Self { game_profile, uuid })
    }
}
