use bevy::{
    prelude::*,
    render::{
        texture::{ TextureDimension, TextureFormat, Extent3d },
    },
};
use crate::game::components::{MainCamera, PixelSimulation, ChunkChanges};
use bevy::utils::HashMap;
use crate::game::data::pixel_simulation::{Chunk, ChunkPosition};
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
    
    let mut chunks = HashMap::<ChunkPosition, Chunk>::default();

    for x in -1 .. 2 {
        for y in -1 .. 2 {
            let texture = Texture::new_fill(
                Extent3d {
                    width: 64,
                    height: 64,
                    depth: 1,
                },
                TextureDimension::D2,
                &[0, 0, 0, 0],
                TextureFormat::Rgba8UnormSrgb,
            );

            let texture_handle = textures.add(texture);
            let material_handle = materials.add(texture_handle.clone().into());

            chunks.insert(ChunkPosition(IVec2::new(x, y)), Chunk::new(texture_handle, material_handle));
        }
    }

    commands.spawn()
        .insert(Name::new("Pixel Simulation"))
        .insert(PixelSimulation::new(chunks.clone()))
        .insert(ChunkChanges::new())
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .with_children(|child_builder| {
            for (position, chunk) in chunks.iter() {
                child_builder.spawn()
                    .insert(Name::new(format!("Chunk {} {}", position.x, position.y)))
                    .insert_bundle(SpriteBundle {
                        material: chunk.material_handle.clone(),
                        sprite: Sprite::new(Vec2::ONE * WORLD_CHUNK_SIZE),
                        transform: Transform::from_translation(Vec3::new(position.x as f32 * WORLD_CHUNK_SIZE, position.y as f32 * WORLD_CHUNK_SIZE, 0.)),
                        ..Default::default()
                    });
            }
        });
}
