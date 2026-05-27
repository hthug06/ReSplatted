use crate::packet::PacketRead;
use crate::types::game_profile::GameProfile;
use std::io::Cursor;

#[derive(Debug)]
pub struct LoginSuccessPacket {
    pub game_profile: GameProfile,
}

impl PacketRead for LoginSuccessPacket {
    const ID: i32 = 0x02;

    fn read(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        Ok(Self {
            game_profile: GameProfile::read(cursor)?,
        })
    }
}
