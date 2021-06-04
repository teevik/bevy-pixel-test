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
use smallvec::{SmallVec, smallvec};
use shrinkwraprs::Shrinkwrap;
use bevy::core::FixedTimestep;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

const PIXEL_SIMULATION_TIMESTEP: f64 = 10.0 / 60.0;

const CHUNK_SIZE: usize = 64;
const WORLD_CHUNK_SIZE: f32 = 300.0;
type ChunkCellIndex = u16;

#[derive(Clone, Copy)]
pub struct CellChange {
    pub cell_position: CellPosition,
    pub new_color: [u8; 4]
}

#[derive(Clone)]
pub struct ChunkChange {
    pub chunk_position: ChunkPosition,
    pub cell_changes: SmallVec<[CellChange; 64]>
}

#[derive(Default, Clone)]
pub struct ChunkChanges {
    chunk_changes: Vec<ChunkChange>
}

impl IntoIterator for ChunkChanges {
    type Item = ChunkChange;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.chunk_changes.into_iter()
    }
}

impl ChunkChanges {
    pub fn new() -> Self {
        Self {
            chunk_changes: Vec::new()
        }
    }
    
    pub fn add_cell_changes(&mut self, chunk_position: ChunkPosition, cell_changes: &[CellChange]) {
        let existing_chunk_change_index = self.chunk_changes.iter().position(|existing_chunk_change| *existing_chunk_change.chunk_position == *chunk_position);
        
        if let Some(existing_chunk_change_index) = existing_chunk_change_index {
            self.chunk_changes[existing_chunk_change_index].cell_changes.extend_from_slice(cell_changes);
        } else {
            self.chunk_changes.push(ChunkChange {
                chunk_position,
                cell_changes: cell_changes.into()
            });
        }
    }

    pub fn add_cell_change(&mut self, chunk_position: ChunkPosition, cell_change: CellChange) {
        let existing_chunk_change_index = self.chunk_changes.iter().position(|existing_chunk_change| *existing_chunk_change.chunk_position == *chunk_position);

        if let Some(existing_chunk_change_index) = existing_chunk_change_index {
            self.chunk_changes[existing_chunk_change_index].cell_changes.push(cell_change);
        } else {
            self.chunk_changes.push(ChunkChange {
                chunk_position,
                cell_changes: smallvec![cell_change]
            });
        }
    }
    
    pub fn clear(&mut self) {
        self.chunk_changes.clear();
    }
}

struct MainCamera;

#[derive(Shrinkwrap, Clone, Copy)]
pub struct CellPosition(UVec2);

#[derive(Shrinkwrap, Clone, Copy)]
pub struct ChunkPosition(IVec2);

impl CellPosition {
    fn to_cell_index(&self) -> usize {
        self.x as usize + (self.y as usize * CHUNK_SIZE)
    }
}

#[derive(Inspectable, Default)]
struct Resources {
    clear_color: ResourceInspector<ClearColor>,
}

#[derive(Default, Clone)]
pub struct PixelSimulation {
    chunks: HashMap<IVec2, Chunk>,
    chunk_changes: ChunkChanges
}

impl PixelSimulation {
    fn new(chunks: HashMap<IVec2, Chunk>) -> Self {
        Self { 
            chunks,
            chunk_changes: ChunkChanges::new()
        }
    }
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum Labels {
    UpdatePixelSimulation,
    SimulatePixelSimulation,
    RenderPixelSimulation
}

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
        .add_system(
            simulate_pixel_simulation.system()
                .label(Labels::SimulatePixelSimulation)
                .before(Labels::RenderPixelSimulation)
                .with_run_criteria(FixedTimestep::step(PIXEL_SIMULATION_TIMESTEP))
        )
        .add_system(
            update_pixel_simulation.system()
                .label(Labels::UpdatePixelSimulation)
                .before(Labels::SimulatePixelSimulation)
        )
        .add_system(render_pixel_simulation.system().label(Labels::RenderPixelSimulation))
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

layout(set = 1, binding = 1) uniform Sprite {
    vec2 size;
    uint flip;
};

void main() {
    vec2 uv = Vertex_Uv;
    
    // Flip the sprite if necessary by flipping the UVs

    uint x_flip_bit = 1; // The X flip bit
    uint y_flip_bit = 2; // The Y flip bit

    float epsilon = 0.00000011920929;
    if ((flip & x_flip_bit) == x_flip_bit) {
        uv = vec2(1.0 - uv.x - epsilon, uv.y);
    }
    if ((flip & y_flip_bit) == y_flip_bit) {
        uv = vec2(uv.x, 1.0 - uv.y - epsilon);
    }
    
    uv_Position = uv;

    vec3 position = Vertex_Position * vec3(size, 1.0);
    gl_Position = ViewProj * Model * vec4(position, 1.0);
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
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
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
        .insert(PixelSimulation::new(chunks.clone()))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .with_children(|child_builder| {
            for (position, chunk) in chunks.iter() {
                child_builder.spawn()
                    .insert(Name::new(format!("Chunk {}", position)))
                    .insert_bundle(SpriteBundle {
                        transform: Transform::from_translation(Vec3::new(position.x as f32 * WORLD_CHUNK_SIZE, position.y as f32 * WORLD_CHUNK_SIZE, 0.)),
                        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                            pipeline_handle.clone(),
                        )]),
                        sprite: Sprite::new(Vec2::new(WORLD_CHUNK_SIZE, WORLD_CHUNK_SIZE)),
                        ..Default::default()
                    })
                    .insert(chunk.material.clone());
            }
        });
}

fn simulate_pixel_simulation(
    mut query: Query<&mut PixelSimulation>,
    mut textures: ResMut<Assets<Texture>>,
    mut pixel_chunk_materials: ResMut<Assets<PixelChunkMaterial>>
) {
    for mut pixel_simulation in query.iter_mut() {
        
    }
}

fn render_pixel_simulation(
    mut query: Query<&mut PixelSimulation>,
    mut textures: ResMut<Assets<Texture>>,
    mut pixel_chunk_materials: ResMut<Assets<PixelChunkMaterial>>
) {
    for mut pixel_simulation in query.iter_mut() {
        for chunk_change in &pixel_simulation.chunk_changes.chunk_changes {
            let chunk = pixel_simulation.chunks.get(&*chunk_change.chunk_position).unwrap();
            let material = pixel_chunk_materials.get_mut(&chunk.material).unwrap();
            let texture = textures.get_mut(&material.texture).unwrap();

            for cell_change in &chunk_change.cell_changes {
                let texture_index_start = cell_change.cell_position.to_cell_index() * 4;

                texture.data[texture_index_start + 0] = cell_change.new_color[0];
                texture.data[texture_index_start + 1] = cell_change.new_color[1];
                texture.data[texture_index_start + 2] = cell_change.new_color[2];
                texture.data[texture_index_start + 3] = cell_change.new_color[3];
            }
        }
        
        pixel_simulation.chunk_changes.clear();        
    }
}

fn update_pixel_simulation(
    mut query: Query<&mut PixelSimulation>,
    main_camera_query: Query<&Transform, With<MainCamera>>,
    mut textures: ResMut<Assets<Texture>>,
    mut pixel_chunk_materials: ResMut<Assets<PixelChunkMaterial>>,
    windows: Res<Windows>,
    mouse_button_inputs: Res<Input<MouseButton>>,
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
                let cell_position = CellPosition(cell_position);
                
                // println!("{}", cell_position);
                
                if let Some(chunk) = pixel_simulation.chunks.get_mut(&*chunk_position) {
                    chunk.cells[cell_position.x as usize][cell_position.y as usize] = Some(CellContainer { cell: Cell::Sand, color: [255, 255, 0, 255], last_frame_updated: 0 });

                    pixel_simulation.chunk_changes.add_cell_change(chunk_position, CellChange { new_color: [255, 255, 0, 255], cell_position });
                }
                
                // dbg!(cell_position);
            }
        }

        // for (chunk_position, chunk) in pixel_simulation.chunks.iter() {
        //     let mut cell_changes = SmallVec::new();
        //     
        // 
        //     // for i in 0..64*64 {
        //     //     cell_changes.push(CellChange {
        //     //         cell_index: i,
        //     //         new_color: [rand::random(), rand::random(), rand::random(), 255]
        //     //     })
        //     // }
        //     
        //     
        //     let chunk_change = ChunkChange {
        //         chunk_position: *chunk_position,
        //         cell_changes
        //     };
        //     chunk_changes.push(chunk_change);
        // }
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
