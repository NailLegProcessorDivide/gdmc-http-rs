use std::collections::HashMap;

use crate::blocks::Blocks;
use crate::coords::Vec2D;



struct BlockEntities {
    
}

pub struct SubChunk {
    pub blocks: [[[Blocks; 16]; 16]; 16],
}

pub struct Chunk {
    pub sub_chunks: [SubChunk; 24],
    pub inhabited_time: u64,
    pub fluid_ticks: Vec<()>,//todo
    pub height_maps: Vec<()>,
}

pub struct Region {
    pub chunks: [[Option<Box<Chunk>>; 32]; 32]
}

pub struct World {
    pub regions: HashMap<Vec2D, Region>
}

impl Default for Blocks {
    fn default() -> Blocks {
        Blocks::Air
    }
}

impl SubChunk {
    pub fn new() -> Self {
        Self {
            blocks: [[[Blocks::default(); 16]; 16]; 16],
        }
    }
}

impl Chunk {
    pub fn new_empty() -> Self {
        Self {
            sub_chunks: std::array::from_fn(|_| SubChunk::new()),
            inhabited_time: 0,
            fluid_ticks: Vec::new(),
            height_maps: Vec::new(),
        }
    }
}