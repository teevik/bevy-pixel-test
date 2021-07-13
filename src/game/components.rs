use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::game::data::chunk_changes::{ChunkChange, CellChange};
use crate::game::data::pixel_simulation::{Chunk, ChunkPosition, WorldCellPosition, CellType, Cell, ChunkCellPosition, ChunksDimensions, Chunks};
use bevy::math::Rect;
use core::slice;
use smallvec::{smallvec};
use std::sync::{Arc, Mutex};
use crate::game::constants::CHUNK_SIZE;

pub struct MainCamera;

pub struct PixelSimulation {
    pub chunks: Chunks
}