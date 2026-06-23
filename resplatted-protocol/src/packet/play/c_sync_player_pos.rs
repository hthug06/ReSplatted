use crate::io::read::MinecraftReadExt;
use crate::io::{ConnectionContext, ProtocolVersion};
use crate::packet::PacketRead;
use std::io::Cursor;

#[derive(Debug)]
pub struct SyncPlayerPos {
    pub teleport_id: i32, // VarInt
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub velocity_x: Option<f64>,
    pub velocity_y: Option<f64>,
    pub velocity_z: Option<f64>,
    pub yaw: f32,
    pub pitch: f32,
    pub teleport_flag: i32, // https://minecraft.wiki/w/Java_Edition_protocol/Packets#Teleport_Flags
}

impl PacketRead for SyncPlayerPos {
    fn id(ctx: &ConnectionContext) -> i32 {
        match ctx.version {
            ProtocolVersion::V1_21_1 => 0x40,
            ProtocolVersion::V26_1 => 0x48,
        }
    }

    fn read(cursor: &mut Cursor<&[u8]>, ctx: &ConnectionContext) -> std::io::Result<Self> {
        // Teleport ID position in the packet change between versions
        let teleport_id_early = if ctx.version == ProtocolVersion::V26_1 {
            Some(cursor.read_var_int()?)
        } else {
            None
        };

        let x = cursor.read_f64()?;
        let y = cursor.read_f64()?;
        let z = cursor.read_f64()?;

        let (velocity_x, velocity_y, velocity_z) = if ctx.version == ProtocolVersion::V26_1 {
            (
                Some(cursor.read_f64()?),
                Some(cursor.read_f64()?),
                Some(cursor.read_f64()?),
            )
        } else {
            (None, None, None)
        };

        let yaw = cursor.read_f32()?;
        let pitch = cursor.read_f32()?;
        let teleport_flag = match ctx.version {
            ProtocolVersion::V1_21_1 => cursor.read_var_int()?,
            ProtocolVersion::V26_1 => cursor.read_i32()?,
        };

        // 1.21.1, last
        let teleport_id = match teleport_id_early {
            Some(id) => id,
            None => cursor.read_var_int()?,
        };

        Ok(Self {
            teleport_id,
            x,
            y,
            z,
            velocity_x,
            velocity_y,
            velocity_z,
            yaw,
            pitch,
            teleport_flag,
        })
    }
}
