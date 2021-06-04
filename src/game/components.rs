use bevy::utils::HashMap;
use crate::game::data::chunk_changes::ChunkChanges;
use crate::game::data::pixel_simulation::{Chunk, ChunkPosition};

pub struct MainCamera;

#[derive(Default, Clone)]
pub struct PixelSimulation {
    pub chunks: HashMap<ChunkPosition, Chunk>,
    pub chunk_changes: ChunkChanges
}

impl PixelSimulation {
    pub fn new(chunks: HashMap<ChunkPosition, Chunk>) -> Self {
        Self {
            chunks,
            chunk_changes: ChunkChanges::new()
        }
    }
}