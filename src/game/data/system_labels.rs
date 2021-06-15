use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum SystemLabels {
    UpdatePixelSimulation,
    SimulatePixelSimulation
}