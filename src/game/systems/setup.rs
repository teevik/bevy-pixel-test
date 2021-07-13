use bevy::{
    prelude::*,
    render::{
        texture::{ TextureDimension, TextureFormat, Extent3d },
    },
};
use crate::game::components::{MainCamera, PixelSimulation};
use crate::game::data::pixel_simulation::{Chunk, Chunks};
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
    
    let chunks = Chunks::new(|| {
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

        Chunk::new(texture_handle, material_handle)
    });
    
    commands.spawn()
        .insert(Name::new("Pixel Simulation"))
        .insert(PixelSimulation { chunks: chunks.clone() })
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .with_children(|child_builder| {
            for (chunk_index, chunk) in chunks {
                let chunk_position = chunk_index.to_chunk_position();
                
                child_builder.spawn()
                    .insert(Name::new(format!("Chunk {} {}", chunk_position.x, chunk_position.y)))
                    .insert_bundle(SpriteBundle {
                        material: (*chunk.get_material_handle()).clone(),
                        sprite: Sprite::new(Vec2::ONE * WORLD_CHUNK_SIZE),
                        transform: Transform::from_translation(Vec3::new(chunk_position.x as f32 * WORLD_CHUNK_SIZE, -(chunk_position.y as f32) * WORLD_CHUNK_SIZE, 0.)),
                        ..Default::default()
                    });
            }
        });
}
