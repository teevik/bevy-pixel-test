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
    cells: UnsafeCellWrapper<Cells>
}

impl Chunk {
    pub fn new(texture_handle: Handle<Texture>, material_handle: Handle<ColorMaterial>) -> Self {
        Self {
            texture_handle,
            material_handle,
            cells: UnsafeCellWrapper::new(Cells::new())
        }
    }
    
    pub fn get_cells(&self) -> &mut Cells {
        unsafe { &mut *self.cells.0.get() }
    }
}

#[derive(Clone)]
pub struct Cells {
    cells: [[Option<CellContainer>; CHUNK_SIZE]; CHUNK_SIZE]
}

impl Cells {
    pub fn new() -> Self {
        let cells = [[None; CHUNK_SIZE]; CHUNK_SIZE];

        Self {
            cells
        }
    }
    
    pub fn get_cell(&mut self, cell_position: CellPosition) -> Option<CellContainer> {
        self.cells[cell_position.x as usize][cell_position.y as usize]
    }

    pub fn set_cell(&mut self, cell_position: CellPosition, cell_container: Option<CellContainer>) {
        self.cells[cell_position.x as usize][cell_position.y as usize] = cell_container;
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
