use crate::io::read::MinecraftReadExt;
use std::io::{Error, Read};

/// A game profile with infos of the player
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Game_Profile
#[derive(Debug)]
pub struct Clock {
    pub id: i32,   //VarInt
    pub time: i64, // VarLong
    pub fractional_time: f32,
    pub rate: f32,
}

impl Clock {
    /// Read a Clock (from SetTime packet) from flux
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let id: i32 = reader.read_var_int()?;
        let time: i64 = reader.read_var_long()?;
        let fractional_time: f32 = reader.read_f32()?;
        let rate: f32 = reader.read_f32()?;

        Ok(Clock {
            id,
            time,
            fractional_time,
            rate,
        })
    }
}
