use crate::io::read::MinecraftReadExt;
use std::io::{Cursor, Error};

/// A chunk section is a 16x16x16 cube of blocks.
/// A chunk section contain MAXIMUM 4096 block
#[derive(Debug)]
pub struct ChunkSection {
    pub block_count: i16,
    pub liquid_count: i16, // Only in 26.1 +
    pub block_state: PalettedContainer,
    pub biomes: PalettedContainer,
}

impl ChunkSection {
    /// Read a chunk section
    pub fn read(cursor: &mut Cursor<&[u8]>) -> Result<ChunkSection, Error> {
        // No VarInt Size since 1.21.5
        let block_count = cursor.read_i16()?;
        let liquid_count = cursor.read_i16()?;
        let block_state = PalettedContainer::read(cursor, 4096, 4, 8)?;
        let biomes = PalettedContainer::read(cursor, 64, 1, 3)?;

        Ok(ChunkSection {
            block_count,
            liquid_count,
            block_state,
            biomes,
        })
    }
}

/// A Paletted Container is a palette-based storage of entries.
/// Paletted Containers have an associated global palette (either block states or biomes as of now), where values are mapped from.
#[derive(Debug)]
pub struct PalettedContainer {
    pub data: Vec<i64>,
}

/// A Palette is used to know what data is inside a palette container
#[derive(Debug)]
enum Palette {
    /// Only one block inside the palette
    SingleValued { global_id: i64 },
    /// Multiple blocks, stored as local indices mapped to global IDs via the palette
    Indirect { mapping: Vec<i64> },
    /// All possible values are used, local index directly equals the global ID
    Direct,
}

impl PalettedContainer {
    pub fn read(
        cursor: &mut Cursor<&[u8]>,
        expected_entries: usize,
        min_indirect: u8,
        max_indirect: u8,
    ) -> std::io::Result<Self> {
        // How many byte are used to stock all the different block?
        // 0 byte = 2**0 = 1
        // 1 byte = 2**1 = 2
        // 2 byte = 2**2 = 4
        // 4 byte = 2**3 = 16
        // ...
        let bits_per_entry = cursor.read_u8()?;

        // Read palette
        // 0 bpe => 1 block | 1 biome for the whole section
        let palette = if bits_per_entry == 0 {
            Some(Palette::SingleValued {
                global_id: cursor.read_var_int()? as i64,
            })
        }
        // 4-8 bpe for block | 1-3 bpe for biome => a palette of block / biome
        else if bits_per_entry >= min_indirect && bits_per_entry <= max_indirect {
            let palette_length = cursor.read_var_int()?;
            let mut palette = Vec::with_capacity(palette_length as usize);
            for _ in 0..palette_length {
                palette.push(cursor.read_var_int()? as i64);
            }
            Some(Palette::Indirect { mapping: palette })
        }
        // 15 bpe for block | 7 bpe for biome => no palette, all possible values are used
        // Also if we fall on an unknow number like 9 bpe for block, we consider it as a direct palette, (this should never happen)
        else {
            Some(Palette::Direct)
        };

        // As 1.21.5+, the length isn't sent
        // We need to calculate it manually
        let mut raw_data = Vec::new();

        if bits_per_entry > 0 {
            let bpe = bits_per_entry as usize;

            // From the wiki, equal to ceil(log2(world_height + 1))
            let entries_per_long = 64 / bpe;
            let data_length = expected_entries.div_ceil(entries_per_long);

            raw_data.reserve(data_length);
            for _ in 0..data_length {
                raw_data.push(cursor.read_i64()?);
            }
        }

        // Decompress everything
        // 4096 for block and 64 for biomes
        let mut final_data = Vec::with_capacity(expected_entries);

        match palette {
            Some(Palette::SingleValued { global_id }) => {
                final_data.resize(expected_entries, global_id);
            }

            Some(ref p) => {
                // use this: https://minecraft.wiki/w/Java_Edition_protocol/Chunk_format#Hints_for_implementers
                let bpe = bits_per_entry as usize;
                let entries_per_long = 64 / bpe;
                let mask = (1 << bpe) - 1;

                for i in 0..expected_entries {
                    let long_index = i / entries_per_long;
                    let bit_index = (i % entries_per_long) * bpe;

                    let local_id = ((raw_data[long_index]) >> bit_index) & mask;

                    let global_id = match p {
                        Palette::Indirect { mapping } => mapping[local_id as usize],
                        Palette::Direct => local_id,
                        _ => unreachable!(),
                    };

                    final_data.push(global_id);
                }
            }
            None => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid bits per entry in PalettedContainer",
            ))?,
        }

        Ok(Self { data: final_data })
    }
}
