use crate::io::ConnectionContext;
use crate::io::read::MinecraftReadExt;
use crate::packet::PacketRead;
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct PluginMessagePacket {
    pub channel: String,
    pub data: Vec<u8>,
}

impl PacketRead for PluginMessagePacket {
    fn id(_ctx: &ConnectionContext) -> i32 {
        // Same for all versions
        0x01
    }

    fn read(cursor: &mut Cursor<&[u8]>, _ctx: &ConnectionContext) -> std::io::Result<Self> {
        // Read the channel
        let channel = cursor.read_string()?;

        // And the rest
        let mut data = Vec::new();
        cursor.read_to_end(&mut data)?;

        Ok(Self { channel, data })
    }
}
