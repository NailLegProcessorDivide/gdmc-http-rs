use std::error::Error;

use reqwest;
use serde::{Deserialize, Serialize};

use crate::{nbt, WorldLoader, World, world::coords::V2di32};

pub struct NetLoader {
    server: String,
    world: World,
}

#[derive(Serialize, Deserialize)]
struct BlockState {
    axis: Option<String>,
    face: Option<String>,
    facing: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct BlockDataSec {
    axis: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct BlockData {
    id: String,
    x: i32,
    y: i32,
    z: i32,
    state: Option<BlockState>,
    data: Option<String>,
}

impl NetLoader {
    pub fn new(server: String) -> Self {
        Self {
            server,
            world: World::new(),
        }
    }

    fn get_chunk_nbt(&mut self, x: i32, z: i32, dx: i32, dz: i32) -> Result<(), Box<dyn Error>> {
        let response = match reqwest::blocking::get(format!(
            "{}/chunks?x={}&z={}&dx={}&dz={}",
            self.server, x, z, dx, dz
        )) {
            Ok(v) => v,
            Err(_) => return Err("request error".into()),
        };
        let response_data = match response.bytes() {
            Ok(v) => v,
            Err(_) => return Err("response error".into()),
        };
        let (_nbt_name, nbt_data) = nbt::bytes_to_nbt(&response_data)?;
        let data = nbt_data.get_compound()?;
        for (i, v) in data.get("Chunks").unwrap().get_list()?.iter().enumerate() {
            for (n, v) in v.get_compound()?.iter() {
                println!("data.Chunks[{i}].{n} = {}", v.get_type_name());
            }
        }
        let _x = match data.get("ChunkX") {
            Some(v) => v.get_int()?,
            None => return Err("no ChunkX in response".into()),
        };
        let _dx = match data.get("ChunkDX") {
            Some(v) => v.get_int()?,
            None => return Err("no ChunkDX in response".into()),
        };
        let _z = match data.get("ChunkZ") {
            Some(v) => v.get_int()?,
            None => return Err("no ChunkZ in response".into()),
        };
        let _dz = match data.get("ChunkDZ") {
            Some(v) => v.get_int()?,
            None => return Err("no ChunkDZ in response".into()),
        };
        let chunks = match data.get("Chunks") {
            Some(v) => v.get_list()?,
            None => return Err("no Chunks in response".into()),
        };
        for chunk_nbt in chunks.iter() {
            self.world.insert_chunk_from_nbt(chunk_nbt)?;
        }
        Ok(())
    }
}

impl WorldLoader for NetLoader {
    fn get_world(&self) -> &World {
        &self.world
    }

    fn load_chunks(&mut self, from: V2di32, to: V2di32) -> Result<(), Box<dyn Error>> {
        self.get_chunk_nbt(from.x, from.z, to.x, to.z)
    }
}
