use crate::io::read::MinecraftReadExt;
use std::collections::HashMap;
use std::io::Cursor;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum HeightmapType {
    WorldSurface = 1,
    MotionBlocking = 4,
    MotionBlockingNoLeaves = 5,
}

impl HeightmapType {
    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            1 => Some(HeightmapType::WorldSurface),
            4 => Some(HeightmapType::MotionBlocking),
            5 => Some(HeightmapType::MotionBlockingNoLeaves),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Heightmaps {
    pub heightmaps: HashMap<HeightmapType, Vec<u16>>,
}

impl Heightmaps {
    /// Unpack the bit-packing from the flux
    /// https://minecraft.wiki/w/Java_Edition_protocol/Chunk_format#Hints_for_implementers
    fn unpack_data(raw_data: &[i64], bits_per_value: usize) -> Vec<u16> {
        // A chunk is 16 x 16
        let mut heights = Vec::with_capacity(16 * 16);
        let values_per_long = 64 / bits_per_value;
        let mask = (1 << bits_per_value) - 1;

        for &long in raw_data {
            let mut current_long = long as u64;

            for _ in 0..values_per_long {
                // We need to read 256 row
                // BUT Minecrafdt compress these value in i64. and 64/9 = 7
                // So the server need to send 256 / 7 = 36.5 (floor it) = 37
                // BUT 37 * 7 = 259. So we just need to avoid reading the last 3 bytes
                if heights.len() >= 256 {
                    break;
                }

                let height = (current_long & mask) as u16;
                heights.push(height);

                current_long >>= bits_per_value;
            }
        }

        heights
    }

    pub fn read(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        let mut parsed_heightmaps = HashMap::new();

        // Number of heightmaps type
        let size = cursor.read_var_int()?;

        // Read everything
        for _ in 0..size {
            let heightmap_type = HeightmapType::from_id(cursor.read_var_int()?);

            // Extract data (i64)
            let data_length = cursor.read_var_int()?;
            let mut raw_data = Vec::with_capacity(data_length as usize);
            for _ in 0..data_length {
                raw_data.push(cursor.read_i64()?);
            }

            // Decompress
            if let Some(t) = heightmap_type {
                parsed_heightmaps.insert(t, Heightmaps::unpack_data(&raw_data, 9));
            } else {
                println!(
                    "Unknown heightmap type with id {:?}, skipping...",
                    heightmap_type
                );
            }
        }
        Ok(Self {
            heightmaps: parsed_heightmaps,
        })
    }
}
