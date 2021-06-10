use crate::game::data::pixel_simulation::{CellPosition, ChunkPosition};
use smallvec::{SmallVec};

#[derive(Clone, Copy)]
pub struct CellChange {
    pub cell_position: CellPosition,
    pub new_color: [u8; 4]
}

#[derive(Clone)]
pub struct ChunkChange {
    pub chunk_position: ChunkPosition,
    pub cell_changes: SmallVec<[CellChange; 64]>
}