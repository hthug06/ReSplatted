use crate::io::read::MinecraftReadExt;
use crate::packet::PacketRead;
use crate::types::block_entity::BlockEntity;
use crate::types::chunk_section::ChunkSection;
use crate::types::height_map::Heightmap;
use std::io::{Cursor, Read};

#[derive(Debug)]
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Chunk_Data_and_Update_Light
pub struct LevelChunkWithLightPacket {
    pub x: i32,
    pub z: i32,
    pub height_map: Heightmap,
    pub chunk_data: Vec<ChunkSection>,
    pub block_entities: Vec<BlockEntity>,
}

impl PacketRead for LevelChunkWithLightPacket {
    const ID: i32 = 0x2D;

    fn read(cursor: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        // More info here:
        // https://minecraft.wiki/w/Java_Edition_protocol/Chunk_format#Data_structure

        let x = cursor.read_i32()?;
        let z = cursor.read_i32()?;

        let heightmaps = Heightmap::read(cursor)?;

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

        while section_cursor.position() < data_size as u64 {
            chunk_data.push(ChunkSection::read(&mut section_cursor)?);
        }

        // Block entities
        // Aka block that can contain data
        let mut block_entities = Vec::new();
        for _ in 0..cursor.read_var_int()? {
            block_entities.push(BlockEntity::read(cursor)?);
        }

        Ok(Self {
            x,
            z,
            height_map: heightmaps,
            chunk_data,
            block_entities,
        })

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
