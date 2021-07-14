use bevy::prelude::*;
use crate::game::components::{PixelSimulation};
use crate::game::constants::{CHUNK_SIZE, PIXEL_SIMULATION_TIMESTEP};
use crate::game::data::pixel_simulation::{Cell, CellType, ChunkPosition, ChunkCellPosition, Chunk, WorldCellPosition, ChunksDimensions, ChunkIndex};
use std::num::Wrapping;
use crate::game::data::chunk_changes::CellChange;
use rand::{thread_rng, Rng};

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
            for y in (0..3).rev() {
                let current_chunk_position = ChunkPosition(UVec2::new(x, y));
                let current_chunk_index = ChunkIndex::from_chunk_position(current_chunk_position);

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

                                let mut try_move_offset = |cell_offset: IVec2, pixel_simulation: &mut PixelSimulation, textures: &mut Assets<Texture>| -> bool {
                                    let offseted_cell_position = chunk_cell_position.as_i32() + cell_offset;
                                    let chunk_index_offset = ((offseted_cell_position.as_f32()) / (CHUNK_SIZE as f32)).floor().as_i32();
                                    
                                    let target_chunk_position = current_chunk_position.as_i32() + chunk_index_offset;

                                    if target_chunk_position.x >= 0 && target_chunk_position.x < 3 && target_chunk_position.y >= 0 && target_chunk_position.y < 3 {
                                        let target_chunk_position = ChunkPosition(target_chunk_position.as_u32());
                                        let target_chunk_index = ChunkIndex::from_chunk_position(target_chunk_position);
                                        
                                        let target_chunk_cell_position = ChunkCellPosition((offseted_cell_position - (chunk_index_offset * (CHUNK_SIZE as i32))).as_u32());

                                        if pixel_simulation.chunks.get_chunk(target_chunk_index).get_cell(target_chunk_cell_position).is_none() {
                                            pixel_simulation.chunks.get_chunk(current_chunk_index).set_cell(chunk_cell_position, None, textures);
                                            pixel_simulation.chunks.get_chunk(target_chunk_index).set_cell(target_chunk_cell_position, Some(cell_container), textures);
                                            
                                            return true;
                                        }
                                    }
                                    
                                    false
                                };

                                let mut try_switch_if = |cell_offset: IVec2, target_cell_types: &[CellType], pixel_simulation: &mut PixelSimulation, textures: &mut Assets<Texture>| -> bool {
                                    let offseted_cell_position = chunk_cell_position.as_i32() + cell_offset;
                                    let chunk_index_offset = ((offseted_cell_position.as_f32()) / (CHUNK_SIZE as f32)).floor().as_i32();

                                    let target_chunk_position = current_chunk_position.as_i32() + chunk_index_offset;

                                    if target_chunk_position.x >= 0 && target_chunk_position.x < 3 && target_chunk_position.y >= 0 && target_chunk_position.y < 3 {
                                        let target_chunk_position = ChunkPosition(target_chunk_position.as_u32());
                                        let target_chunk_index = ChunkIndex::from_chunk_position(target_chunk_position);

                                        let target_chunk_cell_position = ChunkCellPosition((offseted_cell_position - (chunk_index_offset * (CHUNK_SIZE as i32))).as_u32());

                                        let target_cell = pixel_simulation.chunks.get_chunk(target_chunk_index).get_cell(target_chunk_cell_position);
                                        return match target_cell {
                                            None => {
                                                pixel_simulation.chunks.get_chunk(current_chunk_index).set_cell(chunk_cell_position, None, textures);
                                                pixel_simulation.chunks.get_chunk(target_chunk_index).set_cell(target_chunk_cell_position, Some(cell_container), textures);

                                                true
                                            }
                                            Some(target_cell) => {
                                                if target_cell_types.iter().any(|target_cell_type| *target_cell_type == target_cell.cell_type) {
                                                    let target_cell = pixel_simulation.chunks.get_chunk(target_chunk_index).get_cell(target_chunk_cell_position);
                                                    pixel_simulation.chunks.get_chunk(current_chunk_index).set_cell(chunk_cell_position, target_cell, textures);
                                                    pixel_simulation.chunks.get_chunk(target_chunk_index).set_cell(target_chunk_cell_position, Some(cell_container), textures);

                                                    true
                                                } else {
                                                    false
                                                }
                                            }

                                        }
                                    }

                                    false
                                };

                                let slide_direction = if is_even_iteration { -1 } else { 1 };

                                match cell_container.cell_type {
                                    CellType::Sand => {
                                        if try_switch_if(IVec2::new(0, 1), &[CellType::Water], &mut pixel_simulation, &mut textures) {}
                                        else if try_switch_if(IVec2::new(slide_direction, 1), &[CellType::Water], &mut pixel_simulation, &mut textures) {}
                                        else if try_switch_if(IVec2::new(-slide_direction, 1), &[CellType::Water], &mut pixel_simulation, &mut textures) {}
                                        else { pixel_simulation.chunks.get_chunk(current_chunk_index).set_cell(chunk_cell_position, Some(cell_container), &mut textures); }
                                    }
                                    CellType::Water => {
                                        if try_move_offset(IVec2::new(0, 1), &mut pixel_simulation, &mut textures) {}
                                        else if try_move_offset(IVec2::new(slide_direction, 1), &mut pixel_simulation, &mut textures) {}
                                        else if try_move_offset(IVec2::new(-slide_direction, 1), &mut pixel_simulation, &mut textures) {}
                                        else if try_move_offset(IVec2::new(slide_direction, 0), &mut pixel_simulation, &mut textures) {}
                                        else if try_move_offset(IVec2::new(-slide_direction, 0), &mut pixel_simulation, &mut textures) {}
                                        else if try_move_offset(IVec2::new(slide_direction * 2, 0), &mut pixel_simulation, &mut textures) {}
                                        else if try_move_offset(IVec2::new(-slide_direction * 2, 0), &mut pixel_simulation, &mut textures) {}
                                        else { pixel_simulation.chunks.get_chunk(current_chunk_index).set_cell(chunk_cell_position, Some(cell_container), &mut textures); }
                                    }
                                }
                            }
                        }
                    }
                }
                
            }
        }


        for x in 0..3 {
            for y in 0..3 {
                let chunk_position = ChunkPosition(UVec2::new(x, y));
                let chunk_index = ChunkIndex::from_chunk_position(chunk_position);

                pixel_simulation.chunks.get_chunk(chunk_index).clear_particles_texture(&mut textures);
            }
        }

        let mut particles = pixel_simulation.particles.clone();

        particles.retain_mut(|particle| {
            particle.velocity.y += 200. * PIXEL_SIMULATION_TIMESTEP;
            particle.position += particle.velocity * PIXEL_SIMULATION_TIMESTEP;
            particle.position = particle.position.clamp(Vec2::new(0., 0.), Vec2::new((3 * CHUNK_SIZE - 1) as f32, (3 * CHUNK_SIZE - 1) as f32));

            let world_cell_position = particle.position.as_u32();
            let chunk_position = ChunkPosition(((world_cell_position) / (CHUNK_SIZE as u32)).clamp(UVec2::new(0, 0), UVec2::new(2, 2)));
            let chunk_cell_position = ChunkCellPosition(world_cell_position - (*chunk_position * (CHUNK_SIZE as u32)));
            let chunk_index = ChunkIndex::from_chunk_position(chunk_position);

            if (chunk_position.y == 2 && (*chunk_cell_position).y == (CHUNK_SIZE-1) as u32) || pixel_simulation.chunks.get_chunk(chunk_index).get_cell(chunk_cell_position).is_some() {
                let offseted_cell_position = chunk_cell_position.as_i32() + IVec2::new(0, -1);
                let chunk_index_offset = ((offseted_cell_position.as_f32()) / (CHUNK_SIZE as f32)).floor().as_i32();
                let target_chunk_position = chunk_position.as_i32() + chunk_index_offset;


                if target_chunk_position.x >= 0 && target_chunk_position.x < 3 && target_chunk_position.y >= 0 && target_chunk_position.y < 3 {
                    let target_chunk_position = ChunkPosition(target_chunk_position.as_u32());
                    let target_chunk_index = ChunkIndex::from_chunk_position(target_chunk_position);
                    let target_chunk_cell_position = ChunkCellPosition((offseted_cell_position - (chunk_index_offset * (CHUNK_SIZE as i32))).as_u32());

                    if pixel_simulation.chunks.get_chunk(target_chunk_index).get_cell(target_chunk_cell_position).is_none() {
                        pixel_simulation.chunks.get_chunk(target_chunk_index).set_cell(target_chunk_cell_position, Some(Cell {
                            cell_type: particle.particle_type,
                            color: particle.color,
                            last_iteration_updated: 0
                        }), &mut textures);

                    }
                }

                return false;
            }

            pixel_simulation.chunks.get_chunk(chunk_index).particles_texture.set_color(chunk_cell_position, particle.color, &mut textures);

            true
        });

        pixel_simulation.particles = particles;
    }
}
