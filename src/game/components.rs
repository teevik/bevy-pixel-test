use bevy::utils::HashMap;
use crate::game::data::chunk_changes::ChunkChanges;
use crate::game::data::pixel_simulation::{Chunk, ChunkPosition};
use bevy::math::Rect;

pub struct MainCamera;

pub struct PixelSimulation {
    pub chunks: HashMap<ChunkPosition, Chunk>,
    pub chunks_dimensions: Rect<i32>,
    pub chunk_changes: ChunkChanges
}

impl PixelSimulation {
    pub fn new(chunks: HashMap<ChunkPosition, Chunk>) -> Self {
        let left = chunks.keys().min_by_key(|position| position.x).unwrap().x;
        let right = chunks.keys().max_by_key(|position| position.x).unwrap().x;
        let bottom = chunks.keys().min_by_key(|position| position.y).unwrap().y;
        let top = chunks.keys().max_by_key(|position| position.y).unwrap().y;

        let chunks_dimensions = Rect {
            left,
            right,
            top,
            bottom
        };
        
        Self {
            chunks,
            chunks_dimensions,
            chunk_changes: ChunkChanges::new()
        }
    }
}