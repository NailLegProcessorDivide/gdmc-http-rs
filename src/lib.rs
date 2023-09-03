use std::error::Error;

use world::chunks::World;
use world::coords::V2di32;

pub mod file_loader;
pub mod net_loader;
pub mod world;
pub mod nbt;

pub trait WorldLoader {
    fn get_world(&self) -> &World;
    fn load_chunks(&mut self, from: V2di32, size: V2di32) -> Result<(), Box<dyn Error>>;
}

#[cfg(test)]
mod tests {}
