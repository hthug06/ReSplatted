use crate::io::read::MinecraftReadExt;
use crate::packet::PacketRead;
use bytes::Buf;
use std::io::Cursor;

#[derive(Debug)]
pub struct SyncPlayerPos {
    pub teleport_id: i32, // VarInt
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub velocity_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub teleport_flag: i32, // https://minecraft.wiki/w/Java_Edition_protocol/Packets#Teleport_Flags
}

impl PacketRead for SyncPlayerPos {
    const ID: i32 = 0x48;

    fn read(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        Ok(Self {
            teleport_id: cursor.read_var_int()?,
            x: cursor.get_f64(),
            y: cursor.get_f64(),
            z: cursor.get_f64(),
            velocity_x: cursor.get_f64(),
            velocity_y: cursor.get_f64(),
            velocity_z: cursor.get_f64(),
            yaw: cursor.get_f32(),
            pitch: cursor.get_f32(),
            teleport_flag: cursor.get_i32(),
        })
    }
}
