use bevy::prelude::*;
use crate::game::components::{PixelSimulation};
use crate::game::constants::CHUNK_SIZE;
use crate::game::data::pixel_simulation::{Cell, CellType, ChunkPosition, ChunkCellPosition, Chunk, WorldCellPosition, ChunksDimensions, ChunkIndex};
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
        let horizontal_range_normal = 0..3;
        let horizontal_range = if is_even_iteration {
            itertools::Either::Left(horizontal_range_normal)
        } else {
            itertools::Either::Right(horizontal_range_normal.rev())
        };
        
        for x in horizontal_range {
            for y in 0..3 {
                let current_chunk_index = ChunkIndex(x * y);
                let current_chunk_position = current_chunk_index.to_chunk_position();

                let horizontal_range = if is_even_iteration {
                    itertools::Either::Left(0..CHUNK_SIZE)
                } else {
                    itertools::Either::Right((0..CHUNK_SIZE).rev())
                };

                for x in horizontal_range {
                    for y in (0..CHUNK_SIZE).rev() {
                        let chunk_cell_position = ChunkCellPosition(UVec2::new(x as u32, y as u32));

                        if let Some(cell_container) = pixel_simulation.chunks.get_chunk(current_chunk_index).get_cell(chunk_cell_position) {
                            if cell_container.last_iteration_updated != iteration.0 {
                                let mut cell_container = cell_container;
                                cell_container.last_iteration_updated = iteration.0;

                                let mut try_move_offset = |cell_offset: IVec2| -> bool {
                                    let offseted_cell_position = chunk_cell_position.as_i32() + cell_offset;
                                    let chunk_index_offset = offseted_cell_position / CHUNK_SIZE as i32;
                                    
                                    let target_chunk_position = current_chunk_position.as_i32() + chunk_index_offset;
                                    
                                    if target_chunk_position.x >= 0 && target_chunk_position.x < 3 && target_chunk_position.y >= 0 && target_chunk_position.y < 3 {
                                        let target_chunk_position = ChunkPosition(target_chunk_position.as_u32());
                                        let target_chunk_index = ChunkIndex::from_chunk_position(target_chunk_position);
                                        
                                        let target_chunk_cell_position = ChunkCellPosition((offseted_cell_position - (chunk_index_offset * (CHUNK_SIZE as i32))).as_u32());
                                        
                                        if pixel_simulation.chunks.get_chunk(target_chunk_index).get_cell(target_chunk_cell_position).is_none() {
                                            pixel_simulation.chunks.get_chunk(current_chunk_index).set_cell(chunk_cell_position, None, &mut textures);
                                            pixel_simulation.chunks.get_chunk(target_chunk_index).set_cell(target_chunk_cell_position, Some(cell_container), &mut textures);
                                            
                                            return true;
                                        }
                                    }
                                    
                                    false
                                };

                                let slide_direction = if is_even_iteration { -1 } else { 1 };

                                if try_move_offset(IVec2::new(0, 1)) {}
                                else if try_move_offset(IVec2::new(slide_direction, 1)) {}
                                else if try_move_offset(IVec2::new(-slide_direction, 1)) {}
                                else { pixel_simulation.chunks.get_chunk(current_chunk_index).set_cell(chunk_cell_position, Some(cell_container), &mut textures); }
                            }
                        }
                    }
                }
                
            }
        }        
    }
}
