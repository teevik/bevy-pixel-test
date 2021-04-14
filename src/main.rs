use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        renderer::RenderResources
    }
};
use bevy_inspector_egui::{widgets::ResourceInspector, Inspectable, InspectorPlugin, WorldInspectorParams, WorldInspectorPlugin};
use bevy::render::texture::{Extent3d, TextureDimension, TextureFormat};
use std::ops::Mul;
use bevy::utils::HashMap;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
struct MyMaterial {
    pub color: Color,
}

const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) out vec4 o_Target;
layout(set = 2, binding = 0) uniform MyMaterial_color {
    vec4 color;
};
void main() {
    o_Target = color;
}
"#;

struct PixelSimulation {
    pub texture_map: HashMap<UVec2, Handle<Texture>>
}

#[derive(Inspectable, Default)]
struct Resources {
    clear_color: ResourceInspector<ClearColor>,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldInspectorParams {
            despawnable_entities: true,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectorPlugin::<Resources>::new())
        .add_asset::<MyMaterial>()
        .add_startup_system(setup.system())
        .add_system(aaaa.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut textures: ResMut<Assets<Texture>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MyMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    commands.spawn()
        .insert(Name::new("Camera"))
        .insert_bundle(OrthographicCameraBundle::new_2d());

    let mut chunks = HashMap::<UVec2, Entity>::default();
    let mut texture_map = HashMap::<UVec2, Handle<Texture>>::default();

    for column in -1 .. 2 {
        for row in -1 .. 2 {
            let texture = Texture::new_fill(Extent3d::new(128, 128, 1), TextureDimension::D2, &[255, 255, 250, 255], TextureFormat::default());
            let texture_handle = textures.add(texture);

            let entity = commands.spawn()
                .insert(Name::new(format!("Chunk {} {}", column, row)))
                .insert_bundle(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new((column as f32) * 128. * 4., (row as f32) * 128. * 4., 0.),
                        scale: Vec3::ONE.mul(4.),
                        ..Default::default()
                    },
                    material: materials.add(ColorMaterial::texture(texture_handle.clone())),
                    ..Default::default()
                })
                .id();

            chunks.insert(UVec2::new(column as u32, row as u32), entity);
            texture_map.insert(UVec2::new(column as u32, row as u32), texture_handle);
        }
    }

    let children: Vec<Entity> = chunks.values().copied().collect();
    commands.spawn()
        .insert(Name::new("Pixel Simulation"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(PixelSimulation { texture_map })
        .push_children(children.as_slice());
}

fn aaaa(
    query: Query<&PixelSimulation>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>
) {
    for pixel_simulation in query.iter() {
        // let material = materials.get(material_handle).unwrap();
        // let texture_handle = material.texture.clone().unwrap();
        // let texture = textures.get_mut(texture_handle).unwrap();
        for (position, texture_handle) in &pixel_simulation.texture_map {
            let texture = textures.get_mut(texture_handle).unwrap();

            for i in 0 .. 128 * 128 * 4 {
                // texture.data[i] = rand::random();
            }
        }
    }
}