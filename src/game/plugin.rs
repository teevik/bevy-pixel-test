use bevy::prelude::*;
use crate::game::systems::setup::setup;
use crate::game::systems::simulate_pixel_simulation::simulate_pixel_simulation;
use crate::game::systems::update_pixel_simulation::update_pixel_simulation;
use crate::game::systems::render_pixel_simulation::render_pixel_simulation;
use crate::game::data::system_labels::SystemLabels;
use crate::game::constants::PIXEL_SIMULATION_TIMESTEP;
use bevy::core::FixedTimestep;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
        
        app.add_system(
            simulate_pixel_simulation.system()
                .label(SystemLabels::SimulatePixelSimulation)
                .before(SystemLabels::RenderPixelSimulation)
                .with_run_criteria(FixedTimestep::step(PIXEL_SIMULATION_TIMESTEP))
        );
            
        app.add_system(
            update_pixel_simulation.system()
                .label(SystemLabels::UpdatePixelSimulation)
                .before(SystemLabels::SimulatePixelSimulation)
        );
        
        app.add_system(
            render_pixel_simulation.system()
                .label(SystemLabels::RenderPixelSimulation)
        );
    }
}
