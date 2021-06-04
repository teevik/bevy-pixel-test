use bevy::prelude::*;
use crate::show_fps::systems::update_fps_text::update_fps_text;
use crate::show_fps::systems::setup::setup;

pub struct ShowFpsPlugin;

impl Plugin for ShowFpsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
        app.add_system(update_fps_text.system());
    }
}
