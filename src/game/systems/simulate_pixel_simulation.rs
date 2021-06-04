use bevy::prelude::*;
use crate::game::components::PixelSimulation;
use crate::game::data::pixel_simulation::ChunkPosition;

pub fn simulate_pixel_simulation(
    mut query: Query<&mut PixelSimulation>
) {
    for mut pixel_simulation in query.iter_mut() {
        for (chunk_position, chunk) in &mut pixel_simulation.chunks {
            // unsafe {
            //     let a = pixel_simulation.chunks.get_mut(&ChunkPosition(IVec2::new(0, 0)));
            // }
        }
    }
}
