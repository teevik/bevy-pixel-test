mod game;
mod show_fps;

use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin, InspectorPlugin, WorldInspectorParams, InspectableRegistry};
use bevy_inspector_egui::widgets::{ResourceInspector};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin};
use crate::show_fps::plugin::ShowFpsPlugin;
use crate::game::plugin::GamePlugin;

#[derive(Inspectable, Default)]
struct Resources {
    clear_color: ResourceInspector<ClearColor>,
}


fn main() {
    let mut app = App::build();
    
    app.add_plugins(DefaultPlugins);
    
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_plugin(InspectorPlugin::<Resources>::new());
    app.add_plugin(FrameTimeDiagnosticsPlugin::default());

    app.add_plugin(GamePlugin);
    app.add_plugin(ShowFpsPlugin);

    app.insert_resource(WorldInspectorParams {
        despawnable_entities: true,
        ..Default::default()
    });

    let mut registry: Mut<InspectableRegistry> = app
        .world_mut()
        .get_resource_or_insert_with(InspectableRegistry::default);

    // registry.register::<PixelSimulation>();
    
    app.run();
}