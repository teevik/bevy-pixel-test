use bevy::{
    prelude::*,
};
use crate::game::components::{MainCamera, PixelSimulation};
use crate::game::data::pixel_simulation::{Chunk, Chunks, ChunkTexture, ChunkPosition, ChunkIndex};
use crate::game::constants::WORLD_CHUNK_SIZE;

pub fn setup(
    mut commands: Commands,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn()
        .insert(Name::new("Camera"))
        .insert(MainCamera)
        .insert_bundle(OrthographicCameraBundle {
            transform: Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)),
            ..OrthographicCameraBundle::new_2d()
        });

    commands.spawn_bundle(UiCameraBundle::default());
    
    let mut chunks = Chunks::new(|| {
        let main_texture = ChunkTexture::new(&mut textures, &mut materials);
        let particles_texture = ChunkTexture::new(&mut textures, &mut materials);

        Chunk::new(main_texture, particles_texture)
    });
    
    commands.spawn()
        .insert(Name::new("Pixel Simulation"))
        .insert(PixelSimulation::new(chunks.clone()))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .with_children(|child_builder| {
            for x in 0..3 {
                for y in 0..3 {
                    let chunk_position = ChunkPosition(UVec2::new(x, y));
                    let chunk_index = ChunkIndex::from_chunk_position(chunk_position);
                    let chunk = chunks.get_chunk(chunk_index);

                    child_builder.spawn()
                        .insert(Name::new(format!("Chunk {} {}", chunk_position.x, chunk_position.y)))
                        .insert(GlobalTransform::default())
                        .insert(Transform::from_translation(Vec3::new((chunk_position.x as f32 - 1.) * WORLD_CHUNK_SIZE, -(chunk_position.y as f32 - 1.) * WORLD_CHUNK_SIZE, 0.)))
                        .with_children(|child_builder| {
                            child_builder.spawn()
                                .insert_bundle(SpriteBundle {
                                    material: (*chunk.get_main_texture().get_material_handle()).clone(),
                                    sprite: Sprite::new(Vec2::ONE * WORLD_CHUNK_SIZE),
                                    transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                                    ..Default::default()
                                });

                            child_builder.spawn()
                                .insert_bundle(SpriteBundle {
                                    material: (*chunk.get_particles_texture().get_material_handle()).clone(),
                                    sprite: Sprite::new(Vec2::ONE * WORLD_CHUNK_SIZE),
                                    transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
                                    ..Default::default()
                                });
                        });
                }
            }
        });
}
