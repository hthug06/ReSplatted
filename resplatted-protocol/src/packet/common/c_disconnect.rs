use crate::io::{ConnectionContext, ProtocolState, ProtocolVersion};
use crate::packet::PacketRead;
use fastnbt::Value;
use std::io::{Cursor, Read};

pub struct DisconnectPacket {
    pub reason: String,
}

impl PacketRead for DisconnectPacket {
    fn id(ctx: &ConnectionContext) -> i32 {
        match ctx.state {
            ProtocolState::Login => 0x00,
            ProtocolState::Configuration => 0x2,
            ProtocolState::Play => match ctx.version {
                ProtocolVersion::V1_21_1 => 0x1d,
                ProtocolVersion::V26_1 => 0x20,
            },
            _ => unreachable!("Invalid State for disconnect packet: {:?}", ctx.state),
        }
    }

    fn read(cursor: &mut Cursor<&[u8]>, _ctx: &ConnectionContext) -> std::io::Result<Self> {
        let mut buf: Vec<u8> = Vec::new();
        cursor.read_to_end(&mut buf)?;

        // If the first byte us 10, it's a TAG_Compound, so it's a nbt, we inject an empty name for fastnbt to read it
        let parsed_reason: String = if buf.first() == Some(&0x0A) {
            buf.insert(1, 0x00); // first byte length = 0
            buf.insert(2, 0x00); // second byte length = 0

            match fastnbt::from_bytes::<Value>(&buf) {
                Ok(nbt) => {
                    format!("{:?}", nbt)
                }
                Err(e) => format!("NBT Error: {}", e),
            }
        } else {
            let clean_reason: String = buf
                .into_iter()
                .filter(|&b| (32..=126).contains(&b)) // ASCII table
                .map(|b| b as char)
                .collect::<String>();
            clean_reason
        };

        Ok(Self {
            reason: parsed_reason,
        })
    }
}
