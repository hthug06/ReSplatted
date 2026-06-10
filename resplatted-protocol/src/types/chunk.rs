use crate::packet::play::c_level_chunk_with_light::LevelChunkWithLightPacket;
use crate::types::block_entity::BlockEntity;
use crate::types::block_pos::BlockPos;
use crate::types::chunk_section::ChunkSection;
use crate::types::height_map::Heightmap;
use std::io::Error;

pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub sections: Vec<ChunkSection>,
    pub block_entities: Vec<BlockEntity>,
    pub height_map: Heightmap,
}

impl Chunk {
    /// Create a chunk from a level chunk with light packet
    pub fn from_level_chunk_packet(packet: LevelChunkWithLightPacket) -> Self {
        Self {
            x: packet.x,
            z: packet.z,
            sections: packet.chunk_data,
            block_entities: packet.block_entities,
            height_map: packet.height_map,
        }
    }

    /// Count the total number of block in a chunk
    /// We do this by summing the block count of each section
    pub fn count_total_block(&self) -> u32 {
        self.sections
            .iter()
            .fold(0, |acc: u32, section: &ChunkSection| {
                acc + section.block_count as u32
            })
    }

    /// Count the total of liquid in the chunk
    pub fn count_total_liquid(&self) -> u32 {
        self.sections
            .iter()
            .fold(0, |acc: u32, section: &ChunkSection| {
                acc + section.liquid_count as u32
            })
    }

    /// Return the chunk section of the given y
    fn get_section_index(&self, y: i32) -> Result<i32, Error> {
        // Because the world start at -64, section 0 is -64..-49 (only overworld)
        // TODO: adapt this for the nether and the end
        let index = (y + 64) / 16;
        if index < 0 || index > self.sections.len() as i32 {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Y coordinate out of bounds",
            ));
        }
        Ok(index)
    }

    /// Return the block id at the given blockpos coordinate in chunk
    pub fn get_block_at(&self, pos: BlockPos) -> Result<i64, Error> {
        let section_index = self.get_section_index(pos.y)?;
        let section = self.sections.get(section_index as usize).ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::InvalidInput,
                "Section index out of bounds",
            )
        })?;
        let block_id = section.get_block_at(pos)?;
        Ok(block_id)
    }
}
