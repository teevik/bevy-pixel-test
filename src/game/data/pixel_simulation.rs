use bevy::prelude::*;
use crate::game::constants::CHUNK_SIZE;
use shrinkwraprs::Shrinkwrap;
use crate::game::data::unsafe_cell_wrapper::UnsafeCellWrapper;

#[derive(Shrinkwrap, Clone, Copy)]
pub struct CellPosition(pub UVec2);

impl CellPosition {
    pub fn to_cell_index(&self) -> usize {
        self.x as usize + (self.y as usize * CHUNK_SIZE)
    }
}

#[derive(Shrinkwrap, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPosition(pub IVec2);

#[derive(Clone)]
pub struct Chunk {
    pub texture_handle: Handle<Texture>,
    pub material_handle: Handle<ColorMaterial>,
    pub cells: UnsafeCellWrapper<[[Option<CellContainer>; CHUNK_SIZE]; CHUNK_SIZE]>
}

impl Chunk {
    pub fn new(texture_handle: Handle<Texture>, material_handle: Handle<ColorMaterial>) -> Self {
        let cells = UnsafeCellWrapper::new([[None; CHUNK_SIZE]; CHUNK_SIZE]);

        Self {
            texture_handle,
            material_handle,
            cells
        }
    }

    pub fn get_cell(&self, cell_position: CellPosition) -> Option<CellContainer> {
        let cells = &mut unsafe { *self.cells.0.get() };

        cells[cell_position.x as usize][cell_position.y as usize]
    }

    pub fn set_cell(&self, cell_position: CellPosition, cell_container: Option<CellContainer>) {
        let cells = &mut unsafe { *self.cells.0.get() };

        cells[cell_position.x as usize][cell_position.y as usize] = cell_container;
    } 
}

#[derive(Copy, Clone)]
pub struct CellContainer {
    pub cell: Cell,
    pub color: [u8; 4],
    pub last_iteration_updated: u64
}

#[derive(Copy, Clone)]
pub enum Cell {
    Sand,
    Water
}
