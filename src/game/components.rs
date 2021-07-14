use crate::game::data::pixel_simulation::{Chunks, Particles};

pub struct MainCamera;

pub struct PixelSimulation {
    pub chunks: Chunks,
    pub particles: Particles
}

impl PixelSimulation {
    pub fn new(chunks: Chunks) -> Self {
        Self {
            chunks,
            particles: Particles::new()
        }
    }
}