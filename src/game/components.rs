use bevy::utils::HashMap;
use crate::game::data::chunk_changes::{ChunkChange, CellChange};
use crate::game::data::pixel_simulation::{Chunk, ChunkPosition};
use bevy::math::Rect;
use core::slice;
use smallvec::{smallvec};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub struct MainCamera;

pub struct PixelSimulation {
    pub chunks: HashMap<ChunkPosition, Arc<Mutex<Chunk>>>,
    pub chunks_dimensions: Rect<i32>
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
            chunks: chunks.iter().map(|(chunk_position, chunk)| (*chunk_position, Arc::new(Mutex::new(chunk.clone())))).collect(),
            chunks_dimensions
        }
    }
}


#[derive(Clone)]
pub struct ChunkChanges {
    chunk_changes: Vec<ChunkChange>
}

impl IntoIterator for ChunkChanges {
    type Item = ChunkChange;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.chunk_changes.into_iter()
    }
}

impl<'a> IntoIterator for &'a ChunkChanges {
    type Item = &'a ChunkChange;
    type IntoIter = slice::Iter<'a, ChunkChange>;

    fn into_iter(self) -> slice::Iter<'a, ChunkChange> {
        self.chunk_changes.iter()
    }
}

impl ChunkChanges {
    pub fn new() -> Self {
        Self {
            chunk_changes: Vec::new()
        }
    }

    pub fn add_cell_changes(&mut self, chunk_position: ChunkPosition, cell_changes: &[CellChange]) {
        let existing_chunk_change_index = self.chunk_changes.iter().position(|existing_chunk_change| *existing_chunk_change.chunk_position == *chunk_position);

        if let Some(existing_chunk_change_index) = existing_chunk_change_index {
            self.chunk_changes[existing_chunk_change_index].cell_changes.extend_from_slice(cell_changes);
        } else {
            self.chunk_changes.push(ChunkChange {
                chunk_position,
                cell_changes: cell_changes.into()
            });
        }
    }

    pub fn add_cell_change(&mut self, chunk_position: ChunkPosition, cell_change: CellChange) {
        let existing_chunk_change_index = self.chunk_changes.iter().position(|existing_chunk_change| *existing_chunk_change.chunk_position == *chunk_position);

        if let Some(existing_chunk_change_index) = existing_chunk_change_index {
            self.chunk_changes[existing_chunk_change_index].cell_changes.push(cell_change);
        } else {
            self.chunk_changes.push(ChunkChange {
                chunk_position,
                cell_changes: smallvec![cell_change]
            });
        }
    }

    pub fn clear(&mut self) {
        self.chunk_changes.clear();
    }
}