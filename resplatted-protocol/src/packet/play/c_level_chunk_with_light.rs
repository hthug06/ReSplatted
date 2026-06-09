use crate::io::read::MinecraftReadExt;
use crate::packet::PacketRead;
use crate::types::chunk_section::ChunkSection;
use crate::types::height_map::Heightmaps;
use std::io::{Cursor, Read};

#[derive(Debug)]
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Data_and_Update_Light
pub struct LevelChunkWithLightPacket {
    pub x: i32,
    pub z: i32,
    pub heightmaps: Heightmaps,
    chunk_data: Vec<ChunkSection>,
}

impl PacketRead for LevelChunkWithLightPacket {
    const ID: i32 = 0x2D;

    fn read(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        // More info here:
        // https://minecraft.wiki/w/Java_Edition_protocol/Chunk_format#Data_structure

        let x = cursor.read_i32()?;
        let z = cursor.read_i32()?;

        let heightmaps = Heightmaps::read(cursor)?;
        println!("Heightmaps: {:?}", heightmaps);

        // DATA aka array of chunk section
        // The array is not prefixed
        // The number of elements in the array is calculated based on the world's height.
        // Send from bottom to top (because the higher you are, the less block there is)

        // Read the size, put it into a buffer and then, read all the section
        let data_size = cursor.read_var_int()?;

        let mut chunk_data_bytes = vec![0; data_size as usize];
        cursor.read_exact(&mut chunk_data_bytes)?;

        // Read section from this new buffer and a new cursor
        let mut section_cursor = Cursor::new(chunk_data_bytes.as_slice());
        let mut chunk_data = Vec::new();

        let mut i = 0;
        while section_cursor.position() < data_size as u64 {
            let section = ChunkSection::read(&mut section_cursor)?;
            let y_min = -64 + (i * 16);
            let y_max = y_min + 15;
            println!(
                "Section {i} (Y {y_min}..{y_max}): block_count={} and liquid_count={}",
                section.block_count, section.liquid_count
            );
            chunk_data.push(section);
            i += 1;
        }

        Ok(Self {
            x,
            z,
            heightmaps,
            chunk_data,
        })

        // Block entities
        /*let block_entities_size = cursor.read_var_int()?;
        let mut block_entities_data = vec![0; block_entities_size as usize];
        cursor.read_to_end(&mut block_entities_data)?;*/

        /*// Light data
        // Sky Light Mask
        let sky_light_mask_size = cursor.read_var_int()?;
        for _ in sky_light_mask_size {
            cursor.read_i64()?;
        }

        // Block Light Mask
        let block_light_mask_size = cursor.read_var_int()?;
        for _ in block_light_mask_size {
            cursor.read_i64()?;
        }

        // Empty Sky Light Mask
        for _ in cursor.read_var_int()? {
            cursor.read_i64()?;
        }

        // Empty Block Light Mask
        for _ in cursor.read_var_int()? {
            cursor.read_i64()?;
        }

        // Sky light array
        let sky_light_array_count = cursor.read_var_int()?;
        for _ in 0..sky_light_array_count {
            let array_length = cursor.read_var_int()?; // Toujours 2048
            cursor.set_position(cursor.position() + array_length as u64); // On saute 2048 octets
        }

        // Block Light Arrays
        let block_light_array_count = cursor.read_var_int()?;
        for _ in 0..block_light_array_count {
            let array_length = cursor.read_var_int()?; // Toujours 2048
            cursor.set_position(cursor.position() + array_length as u64); // On saute 2048 octets
        }*/
    }
}
