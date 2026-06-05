use crate::io::read::MinecraftReadExt;
use std::io::{Cursor, Error};

/// A chunk section is a 16x16x16 cube of blocks.
/// A chunk section contain MAXIMUM 4096 block
#[derive(Debug)]
pub struct ChunkSection {
    pub block_count: i16,
    pub block_state: PalettedContainer,
    pub biomes: PalettedContainer,
}

impl ChunkSection {
    /// Read a chunk section
    pub fn read(cursor: &mut Cursor<&[u8]>) -> Result<ChunkSection, Error> {
        // 100% OKAY
        let size = cursor.read_var_int()?;
        println!("Chunk section size: {}", size);
        let block_count = cursor.read_i16()?;
        println!("Chunk section block count: {}", block_count);

        //  TODO: find the bug here
        let block_state = PalettedContainer::read(cursor, 4096, 4, 8)?;

        let biomes = PalettedContainer::read(cursor, 64, 1, 3)?;

        Ok(ChunkSection {
            block_count,
            block_state,
            biomes,
        })
    }
}

/// A Paletted Container is a palette-based storage of entries.
/// Paletted Containers have an associated global palette (either block states or biomes as of now), where values are mapped from.
#[derive(Debug)]
struct PalettedContainer {
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
        let bits_per_entry = cursor.read_u8()?;
        println!("bits_per_entry: {}", bits_per_entry);
        println!(
            "expected_entries: {}, min_indirect: {}, max_indirect: {}",
            expected_entries, min_indirect, max_indirect
        );

        // read palette
        let palette = if bits_per_entry == 0 {
            Some(Palette::SingleValued {
                global_id: cursor.read_var_int()? as i64,
            })
        } else if bits_per_entry >= min_indirect && bits_per_entry <= max_indirect {
            let palette_length = cursor.read_var_int()?;
            let mut palette = Vec::with_capacity(palette_length as usize);
            for _ in 0..palette_length {
                palette.push(cursor.read_var_int()? as i64);
            }
            Some(Palette::Indirect { mapping: palette })
        } else {
            Some(Palette::Direct)
        };

        println!("Palette: {:?}", palette);

        // Get all the data (it's an array of long)
        // But as 1.21.5+, the lenght isn't sent
        // We need to calculate it manually
        let mut raw_data = Vec::new();

        if bits_per_entry > 0 {
            let bpe = bits_per_entry as usize;

            let entries_per_long = 64 / bpe;
            let data_length = (expected_entries + entries_per_long - 1) / entries_per_long;

            raw_data.reserve(data_length);
            for _ in 0..data_length {
                raw_data.push(cursor.read_i64()?);
            }
        }

        // println!("bits per entry: {}, palette: {:?}, raw data length: {}", bits_per_entry, palette, raw_data.len());

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

        println!("final data: {:?}", final_data);

        Ok(Self { data: final_data })
    }
}
