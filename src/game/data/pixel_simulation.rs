use bevy::prelude::*;
use crate::game::constants::CHUNK_SIZE;
use shrinkwraprs::Shrinkwrap;
use bevy::render::texture::{Extent3d, TextureDimension, TextureFormat};
use retain_mut::RetainMut;
use palette::{Srgba};

#[derive(Shrinkwrap, Clone, Copy)]
pub struct WorldCellPosition(pub IVec2);

#[derive(Shrinkwrap, Clone, Copy)]
pub struct ChunkCellPosition(pub UVec2);

#[derive(Shrinkwrap, Clone, Copy)]
pub struct ChunkIndex(pub usize);

impl ChunkIndex {
    pub fn from_chunk_position(chunk_position: ChunkPosition) -> Self {
        Self((chunk_position.0.x as usize) + (3 * (chunk_position.0.y as usize)))
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

#[derive(Clone)]
pub struct ChunkTexture {
    texture_handle: Handle<Texture>,
    material_handle: Handle<ColorMaterial>,
}

impl ChunkTexture {
    pub fn new(textures: &mut Assets<Texture>, materials: &mut Assets<ColorMaterial>) -> Self {
        let texture = Texture::new_fill(
            Extent3d {
                width: 64,
                height: 64,
                depth: 1,
            },
            TextureDimension::D2,
            &[0, 0, 0, 0],
            TextureFormat::Rgba8UnormSrgb,
        );

        let texture_handle = textures.add(texture);
        let material_handle = materials.add(texture_handle.clone().into());

        Self {
            texture_handle,
            material_handle
        }
    }

    pub fn get_material_handle(&self) -> &Handle<ColorMaterial> {
        &self.material_handle
    }

    pub fn set_color(&mut self, cell_position: ChunkCellPosition, color: Srgba::<u8>, textures: &mut Assets<Texture>) {
        let texture = textures.get_mut(&self.texture_handle).unwrap();

        let texture_index_start = cell_position.to_cell_index() * 4;

        texture.data[texture_index_start] = color.red;
        texture.data[texture_index_start + 1] = color.green;
        texture.data[texture_index_start + 2] = color.blue;
        texture.data[texture_index_start + 3] = color.alpha;
    }

    pub fn set_cells(&mut self, cells: &[(ChunkCellPosition, [u8; 4])], textures: &mut Assets<Texture>) {
        let texture = textures.get_mut(&self.texture_handle).unwrap();

        for (cell_position, color) in cells {
            let texture_index_start = cell_position.to_cell_index() * 4;

            texture.data[texture_index_start] = color[0];
            texture.data[texture_index_start + 1] = color[1];
            texture.data[texture_index_start + 2] = color[2];
            texture.data[texture_index_start + 3] = color[3];
        }
    }

    pub fn clear(&mut self, textures: &mut Assets<Texture>) {
        let texture = textures.get_mut(&self.texture_handle).unwrap();

        for color_part in &mut texture.data {
            *color_part = 0;
        }
    }
}

#[derive(Clone)]
pub struct Chunk {
    pub main_texture: ChunkTexture,
    pub particles_texture: ChunkTexture,
    cells: Cells
}

impl Chunk {
    pub fn new(main_texture: ChunkTexture, particles_texture: ChunkTexture) -> Self {
        Self {
            main_texture,
            particles_texture,
            cells: Cells::new()
        }
    }

    pub fn get_main_texture(&self) -> &ChunkTexture {
        &self.main_texture
    }

    pub fn get_particles_texture(&self) -> &ChunkTexture {
        &self.particles_texture
    }

    pub fn clear_particles_texture(&mut self, textures: &mut Assets<Texture>) {
        self.particles_texture.clear(textures);
    }

    pub fn get_cell(&self, cell_position: ChunkCellPosition) -> Option<Cell> {
        self.cells.get_cell(cell_position)
    }

    pub fn set_cell(&mut self, cell_position: ChunkCellPosition, cell: Option<Cell>, textures: &mut Assets<Texture>) {
        self.cells.set_cell(cell_position, cell);
        self.main_texture.set_color(cell_position, cell.map(|cell| cell.color).unwrap_or(Srgba::<u8>::new(0, 0, 0, 0)), textures);
    }
}

#[derive(Clone)]
pub struct Particles {
    particles: Vec<Particle>
}

impl Particles {
    pub fn new() -> Self {
        Self {
            particles: Vec::new()
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn retain_mut<F>(&mut self, f: F) where F: FnMut(&mut Particle) -> bool {
        self.particles.retain_mut(f);
    }
}

impl IntoIterator for Particles {
    type Item = Particle;
    type IntoIter = std::vec::IntoIter<Particle>;

    fn into_iter(self) -> Self::IntoIter {
        self.particles.into_iter()
    }
}

#[derive(Clone, Copy)]
pub struct Particle {
    pub particle_type: CellType,
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: Srgba<u8>
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
    pub color: Srgba<u8>,
    pub last_iteration_updated: u64
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CellType {
    Sand,
    Water
}
