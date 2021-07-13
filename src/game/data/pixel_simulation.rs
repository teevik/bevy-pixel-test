use bevy::prelude::*;
use crate::game::constants::CHUNK_SIZE;
use shrinkwraprs::Shrinkwrap;
use std::iter::Enumerate;
use arr_macro::arr;

#[derive(Shrinkwrap, Clone, Copy)]
pub struct WorldCellPosition(pub IVec2);

#[derive(Shrinkwrap, Clone, Copy)]
pub struct ChunkCellPosition(pub UVec2);

#[derive(Shrinkwrap, Clone, Copy)]
pub struct ChunkIndex(pub usize);

impl ChunkIndex {
    pub fn from_chunk_position(chunk_position: ChunkPosition) -> Self {
        Self((chunk_position.0.x * chunk_position.0.y) as usize)
    }
    
    pub fn to_chunk_position(self) -> ChunkPosition {
        let y = (*self) / 3;
        let x = (*self) - (y * 3);
        
        ChunkPosition(UVec2::new(x as u32, y as u32))
    }
}

impl ChunkCellPosition {
    pub fn to_cell_index(&self) -> usize {
        self.x as usize + (self.y as usize * CHUNK_SIZE)
    }
}

#[derive(Shrinkwrap, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPosition(pub UVec2);

#[derive(Shrinkwrap, Clone, Copy)]
pub struct ChunksDimensions(pub Rect<i32>);

impl ChunksDimensions {
    pub fn width(&self) -> u32 {
        (self.right - self.left) as u32
    }

    pub fn height(&self) -> u32 {
        (self.top - self.bottom) as u32
    }
}


#[derive(Clone)]
pub struct Chunks {
    chunks: Vec<Chunk>,
}

impl Chunks {
    pub fn new<ChunkFactory: FnMut() -> Chunk>(mut chunk_factory: ChunkFactory) -> Self {
        let chunks = (0..9).map(|_| chunk_factory()).collect();
        
        Self {
            chunks
        }
    }
    
    pub fn get_chunk(&mut self, chunk_index: ChunkIndex) -> &mut Chunk {
        &mut self.chunks[*chunk_index]
    }
}

impl IntoIterator for Chunks {
    type Item = (ChunkIndex, Chunk);
    type IntoIter = std::iter::Map<Enumerate<std::vec::IntoIter<Chunk>>, fn((usize, Chunk)) -> (ChunkIndex, Chunk)>;

    fn into_iter(self) -> Self::IntoIter {
        self.chunks.into_iter().enumerate().map(|(chunk_index, chunk)| (ChunkIndex(chunk_index), chunk))
    }
}

#[derive(Clone)]
pub struct Chunk {
    texture_handle: Handle<Texture>,
    material_handle: Handle<ColorMaterial>,
    cells: Cells
}

impl Chunk {
    pub fn new(texture_handle: Handle<Texture>, material_handle: Handle<ColorMaterial>) -> Self {
        Self {
            texture_handle,
            material_handle,
            cells: Cells::new()
        }
    }
    
    pub fn get_material_handle(&self) -> &Handle<ColorMaterial> {
        &self.material_handle
    }

    pub fn get_cell(&self, cell_position: ChunkCellPosition) -> Option<Cell> {
        self.cells.get_cell(cell_position)
    }

    pub fn set_cell(&mut self, cell_position: ChunkCellPosition, cell: Option<Cell>, textures: &mut Assets<Texture>) {
        self.cells.set_cell(cell_position, cell);

        let texture = textures.get_mut(&self.texture_handle).unwrap();
        
        let texture_index_start = cell_position.to_cell_index() * 4;

        let new_color = cell.map(|cell| cell.color).unwrap_or([0, 0, 0, 0]);
        
        texture.data[texture_index_start] = new_color[0];
        texture.data[texture_index_start + 1] = new_color[1];
        texture.data[texture_index_start + 2] = new_color[2];
        texture.data[texture_index_start + 3] = new_color[3];
    }
}

#[derive(Clone)]
pub struct Cells {
    cells: [[Option<Cell>; CHUNK_SIZE]; CHUNK_SIZE]
}

impl Cells {
    pub fn new() -> Self {
        let cells = [[None; CHUNK_SIZE]; CHUNK_SIZE];

        Self {
            cells
        }
    }
    
    pub fn get_cell(&self, cell_position: ChunkCellPosition) -> Option<Cell> {
        self.cells[cell_position.x as usize][cell_position.y as usize]
    }

    pub fn set_cell(&mut self, cell_position: ChunkCellPosition, cell: Option<Cell>) {
        self.cells[cell_position.x as usize][cell_position.y as usize] = cell;
    }
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub cell_type: CellType,
    pub color: [u8; 4],
    pub last_iteration_updated: u64
}

#[derive(Copy, Clone)]
pub enum CellType {
    Sand,
    Water
}
