use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
        texture::{ TextureDimension, TextureFormat, Extent3d },
    },
};
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin, InspectorPlugin, WorldInspectorParams, InspectableRegistry};
use bevy_inspector_egui::widgets::{ResourceInspector};
use bevy::utils::HashMap;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, Diagnostics};
use smallvec::SmallVec;

const CHUNK_SIZE: usize = 64;
type ChunkCellIndex = u16;

struct MainCamera;

#[derive(Inspectable, Default)]
struct Resources {
    clear_color: ResourceInspector<ClearColor>,
}

#[derive(Default, Clone)]
pub struct PixelSimulation {
    chunks: HashMap<IVec2, Chunk>
}

#[derive(Clone)]
pub struct Chunk {
    material: Handle<PixelChunkMaterial>,
    cells: [[Option<CellContainer>; CHUNK_SIZE]; CHUNK_SIZE]
}

impl Chunk {
    pub fn new(material: Handle<PixelChunkMaterial>) -> Self {
        let cells = [[None; CHUNK_SIZE]; CHUNK_SIZE];
        
        Self {
            material,
            cells
        }
    }
}

#[derive(Copy, Clone)]
pub struct CellContainer {
    pub cell: Cell,
    pub color: [u8; 4],
    pub last_frame_updated: u64
}

#[derive(Copy, Clone)]
pub enum Cell {
    Sand,
    Water
}

struct FpsText;

/// This example illustrates how to mutate a texture on the CPU.
fn main() {
    let mut app = App::build();
    
    app
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldInspectorParams {
            despawnable_entities: true,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectorPlugin::<Resources>::new())
        .add_asset::<PixelChunkMaterial>()
        .add_startup_system(setup.system())
        .add_system(timer_tick.system())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(text_update_system.system());

    let mut registry: Mut<InspectableRegistry> = app
        .world_mut()
        .get_resource_or_insert_with(InspectableRegistry::default);

    // registry.register::<PixelSimulation>();
    
    app.run();
}

/// The component that represents the mutable texture.
/// It is a resource at the same time.
/// Because it's an asset, it needs an unique UUID.
#[derive(RenderResources, Default, TypeUuid, Inspectable)]
#[uuid = "926b9368-a7ee-42de-9725-152a8b65b37b"]
pub struct PixelChunkMaterial {
    pub texture: Handle<Texture>,
}

const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;
layout(location = 0) out vec2 uv_Position;
layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    uv_Position = Vertex_Uv;
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) in vec2 uv_Position;
layout(location = 0) out vec4 o_Target;
layout(set = 2, binding = 0) uniform texture2D PixelChunkMaterial_texture;
layout(set = 2, binding = 1) uniform sampler PixelChunkMaterial_texture_sampler;
void main() {
    o_Target = texture(sampler2D(PixelChunkMaterial_texture, PixelChunkMaterial_texture_sampler), uv_Position);
}
"#;

fn setup(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut textures: ResMut<Assets<Texture>>,
    mut render_graph: ResMut<RenderGraph>,
    mut my_textures: ResMut<Assets<PixelChunkMaterial>>,
    asset_server: Res<AssetServer>
) {
    // Create a new shader pipeline.
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, include_str!("pixel_chunk.vert"))),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, include_str!("pixel_chunk.frag")))),
    }));

    // Add an AssetRenderResourcesNode to our Render Graph.
    // This will bind MyTexture resources to shaders.
    render_graph.add_system_node(
        "pixel_chunk_material",
        AssetRenderResourcesNode::<PixelChunkMaterial>::new(true),
    );
    // Add a Render Graph edge connecting our new node called "pixel_chunk_material" to the main pass node.
    // This ensures "pixel_chunk_material" runs before the main pass.
    render_graph
        .add_node_edge("pixel_chunk_material", base::node::MAIN_PASS)
        .unwrap();

    commands.spawn()
        .insert(Name::new("Camera"))
        .insert(MainCamera)
        .insert_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(UiCameraBundle::default());

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    
    commands.spawn()
        .insert(Name::new("FPS Text"))
        .insert_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 30.,
                            color: Color::hex("181818").unwrap(),
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 30.,
                            color: Color::hex("181818").unwrap(),
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);

    let mut chunks = HashMap::<IVec2, Chunk>::default();

    for x in -1 .. 2 {
        for y in -1 .. 2 {
            let texture = Texture::new_fill(
                Extent3d {
                    width: 64,
                    height: 64,
                    depth: 1,
                },
                TextureDimension::D2,
                &[255, 255, 255, 255],
                TextureFormat::Rgba8UnormSrgb,
            );

            let texture_handle = textures.add(texture);

            let my_texture_handle = my_textures.add(PixelChunkMaterial { texture: texture_handle.clone() } );
            
            chunks.insert(IVec2::new(x, y), Chunk::new(my_texture_handle));
        }
    }
    
    commands.spawn()
        .insert(Name::new("Pixel Simulation"))
        .insert(PixelSimulation { chunks: chunks.clone() })
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .with_children(|child_builder| {
            for (position, chunk) in chunks.iter() {
                child_builder.spawn()
                    .insert(Name::new(format!("Chunk {}", position)))
                    .insert_bundle(SpriteBundle {
                        transform: Transform::from_translation(Vec3::new(position.x as f32, position.y as f32, 0.)),
                        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                            pipeline_handle.clone(),
                        )]),
                        sprite: Sprite::new(Vec2::new(512., 512.)),
                        ..Default::default()
                    })
                    .insert(chunk.material.clone());
            }
        });
}

fn timer_tick(
    query: Query<&PixelSimulation>,
    main_camera_query: Query<&Transform, With<MainCamera>>,
    mut textures: ResMut<Assets<Texture>>,
    mut pixel_chunk_materials: ResMut<Assets<PixelChunkMaterial>>,
    windows: Res<Windows>,
    mouse_button_inputs: Res<Input<MouseButton>>
) {
    let window = windows.get_primary().unwrap();
    let camera_transform = main_camera_query.single().unwrap();
    
    for pixel_simulation in query.iter() {
        pub struct CellChange {
            pub cell_index: usize,
            pub new_color: [u8; 4]
        }
        
        pub struct ChunkChange {
            pub chunk_position: IVec2,
            pub cell_changes: SmallVec<[CellChange; 64]>
        }

        let mut chunk_changes = Vec::<ChunkChange>::new();

        let should_spawn_sand = mouse_button_inputs.pressed(MouseButton::Left);
        if should_spawn_sand {
            if let Some(cursor_position) = window.cursor_position() {
                let size = Vec2::new(window.width() as f32, window.height() as f32);

                let p = cursor_position - size / 2.0;
                let cursor_position_world = Vec2::from(camera_transform.compute_matrix() * p.extend(0.0).extend(1.0));

                let a = (cursor_position_world / 360.).round().as_i32();
                dbg!(a);
            }
        }

        for (chunk_position, chunk) in pixel_simulation.chunks.iter() {
            let mut cell_changes = SmallVec::new();
            

            for i in 0..64*64 {
                cell_changes.push(CellChange {
                    cell_index: i,
                    new_color: [rand::random(), rand::random(), rand::random(), 255]
                })
            }
            
            
            let chunk_change = ChunkChange {
                chunk_position: *chunk_position,
                cell_changes
            };
            chunk_changes.push(chunk_change);
        }

        for chunk_change in chunk_changes {
            let chunk = pixel_simulation.chunks.get(&chunk_change.chunk_position).unwrap();
            let material = pixel_chunk_materials.get_mut(&chunk.material).unwrap();
            let texture = textures.get_mut(&material.texture).unwrap();
            
            for cell_change in chunk_change.cell_changes {
                let texture_index_start = cell_change.cell_index * 4;
                
                texture.data[texture_index_start + 0] = cell_change.new_color[0];
                texture.data[texture_index_start + 1] = cell_change.new_color[1];
                texture.data[texture_index_start + 2] = cell_change.new_color[2];
                texture.data[texture_index_start + 3] = cell_change.new_color[3];
            }
        }
    }
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.0}", average);
            }
        }
    }
}
