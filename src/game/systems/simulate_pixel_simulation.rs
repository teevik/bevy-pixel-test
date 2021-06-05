use bevy::prelude::*;
use crate::game::components::PixelSimulation;
use crate::game::constants::CHUNK_SIZE;
use crate::game::data::pixel_simulation::{CellContainer, Cell, ChunkPosition, CellPosition, Chunk};
use std::num::Wrapping;
use crate::game::data::chunk_changes::CellChange;

pub fn simulate_pixel_simulation(
    mut query: Query<&mut PixelSimulation>,
    mut iteration: Local<Wrapping<u64>>
) {
    *iteration += Wrapping(1);
    // println!("{}", *iteration);
    
    let is_even_iteration = iteration.0 % 2 == 0;

    for mut pixel_simulation in query.iter_mut() {
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
                
                unsafe {
                    let chunk = &pixel_simulation.chunks[&chunk_position];

                    let horizontal_range = if is_even_iteration {
                        itertools::Either::Left(0..CHUNK_SIZE)
                    } else {
                        itertools::Either::Right((0..CHUNK_SIZE).rev())
                    };

                    for x in horizontal_range {
                        for y in (0..CHUNK_SIZE).rev() {
                            if let Some(cell_container) = cells[x][y] {
                                if cell_container.last_iteration_updated != iteration.0 {
                                    let mut cell_container = cell_container;
                                    cell_container.last_iteration_updated = iteration.0;

                                    let a= IVec2::new(x as i32, y as i32) + IVec2::new(0, 1);
                                    let chunk_offset = (a.as_f32() / (CHUNK_SIZE as f32)).floor().as_i32();
                                    let target_chunk_position = ChunkPosition(*chunk_position - chunk_offset);
                                    let target_cell_position = (a - (chunk_offset * CHUNK_SIZE as i32)).as_u32();
                                    
                                    if pixel_simulation.chunks.contains_key(&target_chunk_position) {
                                        let target_chunk = &pixel_simulation.chunks[&target_chunk_position];
                                        let target_chunk_cells = &mut *pixel_simulation.chunks[&target_chunk_position].cells.0.get();

                                        if target_chunk.get_cell(CellPosition(target_cell_position)).is_none() {
                                            cells[x][y] = None;

                                            target_chunk_cells[target_cell_position.x as usize][target_cell_position.y as usize] = Some(cell_container);

                                            pixel_simulation.chunk_changes.add_cell_change(chunk_position, CellChange {
                                                cell_position: CellPosition(UVec2::new(x as u32, y as u32)),
                                                new_color: [0, 0, 0, 0]
                                            });

                                            pixel_simulation.chunk_changes.add_cell_change(target_chunk_position, CellChange {
                                                cell_position: CellPosition(target_cell_position),
                                                new_color: cell_container.color
                                            });
                                        } else {
                                            cells[x][y] = Some(cell_container);
                                        }
                                    } else {
                                        cells[x][y] = Some(cell_container);
                                    }

                                    // cells[x][y] = None;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
