use bevy::prelude::*;
use crate::game::components::{MainCamera, PixelSimulation};
use crate::game::constants::CHUNK_SIZE;
use crate::game::data::pixel_simulation::{CellType, Particle};
use rand::Rng;
use std::ops::Range;
use lazy_static::lazy_static;
use palette::{Lch, Gradient, FromColor, Srgba};

lazy_static! {
    static ref SAND_GRADIENT: Gradient<Lch> = Gradient::new(vec![
        Lch::new(78.0, 25.0, 92.0),
        Lch::new(83.0, 25.0, 92.0)
    ]);

    static ref WATER_GRADIENT: Gradient<Lch> = Gradient::new(vec![
        Lch::new(65.0, 37.0, 249.0),
        Lch::new(70.0, 37.0, 249.0)
    ]);
}

pub fn update_pixel_simulation(
    mut query: Query<&mut PixelSimulation>,
    main_camera_query: Query<&Transform, With<MainCamera>>,
    windows: Res<Windows>,
    mouse_button_inputs: Res<Input<MouseButton>>
) {
    let window = windows.get_primary().unwrap();
    let camera_transform = main_camera_query.single().unwrap();
    let should_spawn_sand = mouse_button_inputs.pressed(MouseButton::Left);
    let should_spawn_water = mouse_button_inputs.pressed(MouseButton::Right);

    if should_spawn_sand || should_spawn_water {
        for mut pixel_simulation in query.iter_mut() {
            if let Some(cursor_position) = window.cursor_position() {
                let size = Vec2::new(window.width() as f32, window.height() as f32);
        
                let p = cursor_position - size / 2.0;
                let cursor_position_world = Vec2::from(camera_transform.compute_matrix() * p.extend(0.0).extend(1.0));
        
                let world_cell_position = (cursor_position_world / 300. * CHUNK_SIZE as f32).round() + (Vec2::ONE * (CHUNK_SIZE as f32 * 1.5));
                let world_cell_position = Vec2::new(world_cell_position.x, 64. - world_cell_position.y);
                let world_cell_position = Vec2::new(world_cell_position.x, world_cell_position.y + 2. * CHUNK_SIZE as f32);

                const spread: Range<f32> = -10. .. 10.;
                let mut rng = rand::thread_rng();

                if should_spawn_sand {
                    for _ in 0..5 {
                        let color = SAND_GRADIENT.get(rng.gen_range(0.0 .. 1.0));
                        let color = Srgba::from_color(color).into_format();

                        pixel_simulation.particles.add_particle(Particle {
                            particle_type: CellType::Sand,
                            position: world_cell_position,
                            velocity: Vec2::new(rng.gen_range(spread), rng.gen_range(spread)),
                            color
                        });
                    }
                }

                if should_spawn_water {
                    for _ in 0..5 {
                        let color = WATER_GRADIENT.get(rng.gen_range(0.0 .. 1.0));
                        let color = Srgba::from_color(color).into_format();

                        pixel_simulation.particles.add_particle(Particle {
                            particle_type: CellType::Water,
                            position: world_cell_position,
                            velocity: Vec2::new(rng.gen_range(spread), rng.gen_range(spread)),
                            color
                        });
                    }
                }
            }
        }
    }
}
