use bevy::prelude::*;
use crate::game::components::{PixelSimulation, ChunkChanges};
use crate::game::constants::CHUNK_SIZE;
use crate::game::data::pixel_simulation::{Cell, CellType, ChunkPosition, ChunkCellPosition, Chunk, WorldCellPosition, ChunksDimensions};
use std::num::Wrapping;
use crate::game::data::chunk_changes::CellChange;

pub fn simulate_pixel_simulation(
    mut query: Query<&mut PixelSimulation>,
    mut iteration: Local<Wrapping<u64>>,
    mut textures: ResMut<Assets<Texture>>
) {
    *iteration += Wrapping(1);
    // println!("{}", *iteration);
    
    let is_even_iteration = iteration.0 % 2 == 0;

    for mut pixel_simulation in query.iter_mut() {
        // let keys: Vec<ChunkPosition> = pixel_simulation.chunks.keys().map(|chunk_position| *chunk_position).collect();

        let chunks_dimensions = pixel_simulation.chunks_dimensions;
        
        struct ChunksMut<'a> {
            chunks_dimensions: ChunksDimensions,
            chunks: Vec<Option<&'a mut Chunk>>
        }
        
        impl<'a> ChunksMut<'a> {
            pub fn new(chunks_dimensions: ChunksDimensions) -> Self {
                let vec_size = (chunks_dimensions.width() * chunks_dimensions.height()) as usize;
                
                let mut vec1 = Vec::<Option<&mut Chunk>>::with_capacity(vec_size);
                vec1.resize_with(vec_size, || None);

                Self {
                    chunks_dimensions,
                    chunks: vec1
                }
            }
            
            pub fn set_chunk(&mut self, chunk_position: ChunkPosition, chunk: &'a mut Chunk) {
                let x_index = (chunk_position.x - self.chunks_dimensions.left) as usize;
                let y_index = (chunk_position.y - self.chunks_dimensions.bottom) as usize;
                let index = x_index + (y_index * CHUNK_SIZE);
                
                self.chunks[index] = Some(chunk);
            }

            pub fn get_chunk(&mut self, chunk_position: ChunkPosition) -> &mut Option<&'a mut Chunk> {
                let x_index = (chunk_position.x - self.chunks_dimensions.left) as usize;
                let y_index = (chunk_position.y - self.chunks_dimensions.bottom) as usize;
                let index = x_index + (y_index * CHUNK_SIZE);

                &mut self.chunks[index]
            }

            pub fn get_chunk_by_index(&mut self, chunk_index: UVec2) -> &mut Option<&'a mut Chunk> {
                let index = self.get_chunk_index(chunk_index);

                &mut self.chunks[index]
            }

            pub fn get_chunk_index(&mut self, index: UVec2) -> usize {
                let index = (index.x as usize) + ((index.y as usize) * CHUNK_SIZE);

                index
            }
        }

        let mut chunks_thing = ChunksMut::new(chunks_dimensions);
        
        for (chunk_position, chunk) in &pixel_simulation.chunks {
            let chunk = &mut (*chunk.lock().unwrap());
            
            chunks_thing.set_chunk(*chunk_position, chunk);
        }
        
        let horizontal_range_normal = 0..chunks_dimensions.width();
        let horizontal_range = if is_even_iteration {
            itertools::Either::Left(horizontal_range_normal)
        } else {
            itertools::Either::Right(horizontal_range_normal.rev())
        };
        
        for x in horizontal_range {
            for y in 0..chunks_dimensions.height() {
                let chunk_index = UVec2::new(x, y);
                
                if let Some(current_chunk) = chunks_thing.get_chunk_by_index(chunk_index) {
                    let horizontal_range = if is_even_iteration {
                        itertools::Either::Left(0..CHUNK_SIZE)
                    } else {
                        itertools::Either::Right((0..CHUNK_SIZE).rev())
                    };

                    for x in horizontal_range {
                        for y in (0..CHUNK_SIZE).rev() {
                            let chunk_cell_position = ChunkCellPosition(UVec2::new(x as u32, y as u32));

                            if let Some(cell_container) = current_chunk.get_cell(chunk_cell_position) {
                                if cell_container.last_iteration_updated != iteration.0 {
                                    let mut cell_container = cell_container;
                                    cell_container.last_iteration_updated = iteration.0;

                                    let mut try_move_offset = |cell_offset: IVec2| -> bool {
                                        let offseted_cell_position = chunk_cell_position.as_i32() + cell_offset;
                                        let chunk_index_offset = offseted_cell_position / CHUNK_SIZE as i32;
                                        
                                        let target_chunk_position = chunk_index.as_i32() + chunk_index_offset;
                                        
                                        if target_chunk_position.x >= 0 && target_chunk_position.x < chunks_dimensions.width() as i32 && target_chunk_position.y >= 0 && target_chunk_position.y < chunks_dimensions.height() as i32 {
                                            let target_chunk_position = target_chunk_position.as_u32();
                                            
                                            if let Some(target_chunk) = chunks_thing.get_chunk_by_index(target_chunk_position) {
                                                let target_chunk_cell_position = ChunkCellPosition((offseted_cell_position - (chunk_index_offset * (CHUNK_SIZE as i32))).as_u32());
                                                
                                                if target_chunk.get_cell(target_chunk_cell_position).is_none() {
                                                    (*current_chunk).set_cell(chunk_cell_position, None, &mut textures);
                                                    (*target_chunk).set_cell(target_chunk_cell_position, Some(cell_container), &mut textures);
                                                    
                                                    return true;
                                                }
                                            }
                                        }
                                        
                                        false
                                    };

                                    let slide_direction = if is_even_iteration { -1 } else { 1 };

                                    if try_move_offset(IVec2::new(0, 1)) {}
                                    else if try_move_offset(IVec2::new(slide_direction, 1)) {}
                                    else if try_move_offset(IVec2::new(-slide_direction, 1)) {}
                                    else { (*current_chunk).set_cell(chunk_cell_position, Some(cell_container), &mut textures); }
                                }
                            }
                        }
                    }
                }
            }
        }        
    }
}
