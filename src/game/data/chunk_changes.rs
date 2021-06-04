use crate::game::data::pixel_simulation::{CellPosition, ChunkPosition};
use smallvec::{SmallVec, smallvec};
use core::slice;

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

#[derive(Default, Clone)]
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
