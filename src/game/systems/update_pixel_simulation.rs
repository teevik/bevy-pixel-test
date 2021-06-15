use bevy::prelude::*;
use crate::game::components::{MainCamera, PixelSimulation, ChunkChanges};
use crate::game::constants::CHUNK_SIZE;
use crate::game::data::pixel_simulation::{ChunkPosition, ChunkCellPosition, Cell, CellType, WorldCellPosition};

pub fn update_pixel_simulation(
    mut query: Query<&mut PixelSimulation>,
    main_camera_query: Query<&Transform, With<MainCamera>>,
    windows: Res<Windows>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut textures: ResMut<Assets<Texture>>
) {
    let window = windows.get_primary().unwrap();
    let camera_transform = main_camera_query.single().unwrap();

    for mut pixel_simulation in query.iter_mut() {
        let should_spawn_sand = mouse_button_inputs.pressed(MouseButton::Left);
        if should_spawn_sand {
            if let Some(cursor_position) = window.cursor_position() {
                let size = Vec2::new(window.width() as f32, window.height() as f32);

                let p = cursor_position - size / 2.0;
                let cursor_position_world = Vec2::from(camera_transform.compute_matrix() * p.extend(0.0).extend(1.0));

                let world_cell_position = (cursor_position_world / 300. * CHUNK_SIZE as f32).round() + (Vec2::ONE * (CHUNK_SIZE as f32 / 2.));
                let world_cell_position = Vec2::new(world_cell_position.x, 64. - world_cell_position.y);
                let chunk_position = (world_cell_position / CHUNK_SIZE as f32).floor().as_i32();
                let world_cell_position = world_cell_position.as_i32();
                let cell_position = (world_cell_position - chunk_position * CHUNK_SIZE as i32).as_u32();
                let chunk_position = IVec2::new(chunk_position.x, -chunk_position.y);

                let chunk_position = ChunkPosition(chunk_position);
                let cell_position = ChunkCellPosition(cell_position);
                
                if let Some(chunk) = pixel_simulation.chunks.get_mut(&*chunk_position) {
                    let chunk = &mut (*chunk.lock().unwrap());
                    chunk.set_cell(cell_position, Some(Cell { cell_type: CellType::Sand, color: [255, 255, 0, 255], last_iteration_updated: 0 }), &mut textures);
                }
            }
        }
    }
}
