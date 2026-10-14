use std::io::{Cursor, Read};
use crate::io::{ConnectionContext, ProtocolVersion};
use crate::io::read::MinecraftReadExt;
use crate::packet::PacketRead;
use crate::types::clock::Clock;

#[derive(Debug)]
pub struct SetTimePacket {
    pub world_age: i64,
    pub clocks: Vec<Clock>
}

impl PacketRead for SetTimePacket {
    fn id(ctx: &ConnectionContext) -> i32 {
        // Same for all versions
        match ctx.version {
            ProtocolVersion::V1_21_1 => 0x64,
            ProtocolVersion::V26_1 | ProtocolVersion::V26_2 => 0x71,
        }
    }

    fn read(cursor: &mut Cursor<&[u8]>, ctx: &ConnectionContext) -> std::io::Result<Self> {
        let world_age: i64 = cursor.read_i64()?;

        let clocks_size: i32 = cursor.read_var_int()?;
        let mut clocks: Vec<Clock> = Vec::with_capacity(clocks_size as usize);
        for _ in 0..clocks_size {
            clocks.push(Clock::read(cursor)?);
        }

        Ok(SetTimePacket {
            world_age,
            clocks
        })
    }
}
