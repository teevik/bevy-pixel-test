use bevy::prelude::*;
use crate::game::components::{PixelSimulation, ChunkChanges};

pub fn render_pixel_simulation(
    mut query: Query<(&PixelSimulation, &mut ChunkChanges)>,
    mut textures: ResMut<Assets<Texture>>
) {
    for (pixel_simulation, mut chunk_changes) in query.iter_mut() {
        for chunk_change in &*chunk_changes {
            let chunk = pixel_simulation.chunks.get(&*chunk_change.chunk_position).unwrap();
            let texture = textures.get_mut(&chunk.texture_handle).unwrap();

            for cell_change in &chunk_change.cell_changes {
                let texture_index_start = cell_change.cell_position.to_cell_index() * 4;
                
                texture.data[texture_index_start] = cell_change.new_color[0];
                texture.data[texture_index_start + 1] = cell_change.new_color[1];
                texture.data[texture_index_start + 2] = cell_change.new_color[2];
                texture.data[texture_index_start + 3] = cell_change.new_color[3];
            }
        }

        chunk_changes.clear();
    }
}
