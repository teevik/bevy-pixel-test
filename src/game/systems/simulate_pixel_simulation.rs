use bevy::prelude::*;
use crate::game::components::{PixelSimulation, ChunkChanges};
use crate::game::constants::CHUNK_SIZE;
use crate::game::data::pixel_simulation::{CellContainer, Cell, ChunkPosition, CellPosition, Chunk};
use std::num::Wrapping;
use crate::game::data::chunk_changes::CellChange;

pub fn simulate_pixel_simulation(
    mut query: Query<(&PixelSimulation, &mut ChunkChanges)>,
    mut iteration: Local<Wrapping<u64>>,
    mut textures: ResMut<Assets<Texture>>
) {
    *iteration += Wrapping(1);
    // println!("{}", *iteration);
    
    let is_even_iteration = iteration.0 % 2 == 0;

    for (pixel_simulation, mut chunk_changes) in query.iter_mut() {
        // let keys: Vec<ChunkPosition> = pixel_simulation.chunks.keys().map(|chunk_position| *chunk_position).collect();

        let dimensions = pixel_simulation.chunks_dimensions;
        
        let awd = dimensions.left..=dimensions.right;
        let horizontal_range = if is_even_iteration {
            itertools::Either::Left(awd)
        } else {
            itertools::Either::Right(awd.rev())
        };
        
        for chunk_x in horizontal_range {
            for chunk_y in dimensions.bottom..=dimensions.top {
                let chunk_position = ChunkPosition(IVec2::new(chunk_x, chunk_y));
                if !pixel_simulation.chunks.contains_key(&chunk_position) { continue };
                
                let chunk = &mut pixel_simulation.chunks[&chunk_position].lock().unwrap();
                let chunk_cells = &mut chunk.cells;

                let horizontal_range = if is_even_iteration {
                    itertools::Either::Left(0..CHUNK_SIZE)
                } else {
                    itertools::Either::Right((0..CHUNK_SIZE).rev())
                };
                
                let set_cell = || {
                    
                };

                for x in horizontal_range {
                    for y in (0..CHUNK_SIZE).rev() {
                        let cell_position = CellPosition(UVec2::new(x as u32, y as u32));
                        
                        if let Some(cell_container) = chunk_cells.get_cell(cell_position) {
                            if cell_container.last_iteration_updated != iteration.0 {
                                let mut cell_container = cell_container;
                                cell_container.last_iteration_updated = iteration.0;
                                
                                let mut try_move_offset = |cell_offset: IVec2| -> bool {
                                    let offseted_cell_position = cell_position.as_i32() + cell_offset;

                                    let chunk_offset = (offseted_cell_position.as_f32() / (CHUNK_SIZE as f32)).floor().as_i32();
                                    let target_chunk_position = ChunkPosition(*chunk_position + IVec2::new(chunk_offset.x, -chunk_offset.y));
                                    let target_cell_position = CellPosition((offseted_cell_position - (chunk_offset * CHUNK_SIZE as i32)).as_u32());

                                    if pixel_simulation.chunks.contains_key(&target_chunk_position) {
                                        let target_chunk = &mut pixel_simulation.chunks[&target_chunk_position].lock().unwrap();
                                        let target_chunk_cells = &mut target_chunk.cells;

                                        if target_chunk_cells.get_cell(target_cell_position).is_none() {
                                            chunk_cells.set_cell(cell_position, None);
                                            target_chunk_cells.set_cell(target_cell_position, Some(cell_container));

                                            chunk_changes.add_cell_change(chunk_position, CellChange {
                                                cell_position: CellPosition(UVec2::new(x as u32, y as u32)),
                                                new_color: [0, 0, 0, 0]
                                            });

                                            chunk_changes.add_cell_change(target_chunk_position, CellChange {
                                                cell_position: target_cell_position,
                                                new_color: cell_container.color
                                            });
                                            true
                                        } else {
                                            false
                                        }
                                    } else {
                                        false
                                    }
                                };
                                
                                let slide_direction = if is_even_iteration { -1 } else { 1 };
                                
                                if try_move_offset(IVec2::new(0, 1)) {}
                                else if try_move_offset(IVec2::new(slide_direction, 1)) {}
                                else if try_move_offset(IVec2::new(-slide_direction, 1)) {}
                                else { chunk_cells.set_cell(cell_position, Some(cell_container)); }
                            }
                        }
                    }
                }
            }
        }
    }
}
