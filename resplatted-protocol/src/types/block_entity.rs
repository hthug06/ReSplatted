use crate::io::read::MinecraftReadExt;
use fastnbt::Value;
use std::collections::HashMap;
use std::io::{Chain, Cursor, Read};

#[derive(Debug)]
pub struct BlockEntity {
    pub x: u8,
    pub y: i16,
    pub z: u8,
    pub block_entity_type: i32,
    pub data: Value, // A NBT
}

impl BlockEntity {
    pub fn read(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        let packed_xz = cursor.read_u8()?;
        let x = packed_xz >> 4;
        let z = packed_xz & 15;
        let y = cursor.read_i16()?;
        let block_entity_type = cursor.read_var_int()?;

        // Check for NBT
        let data = match cursor.read_u8()? {
            0 => Value::Compound(HashMap::new()), // TAG_END: no NBT data
            10 => {
                // NBT_COMPOUND
                // Because we already read 10, we need to reinject it into fast nbt
                // Also add the size because Minecraft didn't do it but fastnbt need it
                let header: [u8; 3] = [10u8, 0u8, 0u8];

                // Chain with the remaining cursor
                let mut virtual_reader: Chain<&[u8], &mut Cursor<&[u8]>> = header.chain(cursor);

                // read with FastNBT
                fastnbt::from_reader(&mut virtual_reader).unwrap_or_else(|e| {
                    eprintln!("Failed to Parse NBT: {}", e);
                    Value::Compound(HashMap::new())
                })
            }
            tag => {
                eprintln!("Wrong NBT Tag: {}", tag);
                Value::Compound(HashMap::new())
            }
        };

        Ok(Self {
            x,
            y,
            z,
            block_entity_type,
            data,
        })
    }
}
