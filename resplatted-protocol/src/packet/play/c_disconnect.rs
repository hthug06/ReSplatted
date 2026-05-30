use crate::packet::PacketRead;
use std::io::{Cursor, Read};

pub struct PlayDisconnectPacket {
    pub reason: String,
}

impl PacketRead for PlayDisconnectPacket {
    const ID: i32 = 0x20;

    fn read(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        let mut buf: Vec<u8> = Vec::new();
        cursor.read_to_end(&mut buf)?;

        // Only keep real char, not nbt things
        // TODO: use a nbt reader
        let clean_reason: String = buf
            .into_iter()
            .filter(|&b| b >= 32 && b <= 126) // ASCII table
            .map(|b| b as char)
            .collect();

        Ok(Self {
            reason: clean_reason,
        })
    }
}
