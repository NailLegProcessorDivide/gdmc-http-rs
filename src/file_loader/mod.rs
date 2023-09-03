use std::{path::{Path, PathBuf}, error::Error, fs, ops::BitAnd};

use crate::{WorldLoader, world::{coords::V2di32, chunks::{World, Region}}};

pub struct FileLoader {
    path: PathBuf,
    world: World
}

impl FileLoader {
    pub fn new_from_file(path: &Path) -> Result<FileLoader, Box<dyn Error>> {
        if !path.is_dir() {
            return Err(format!("no path {}", path.to_str().unwrap()).into())
        }
        Ok(FileLoader{
            path: path.to_owned(),
            world: World::new()
        })
    }
}

impl WorldLoader for FileLoader {
    fn get_world(&self) -> &World {
        &self.world
    }

    fn load_chunks(&mut self, from: V2di32, size: V2di32) -> Result<(), Box<dyn Error>> {
        let first_region = from >> 5;
        let last_region_chunk = from + size - 1;
        let last_region = (last_region_chunk >> 5) + 1;
        let first_chunk = from & 31;
        let last_chunk = (last_region_chunk & 31) + 1;
        println!("{:?} {:?}, {:?} {:?} - {:?}", first_region, first_chunk, last_region, last_chunk, last_region_chunk);
        for (rx, rz) in (last_region - first_region).iter() {
            let region_pos = first_region + V2di32 {x: rx, z: rz};
            println!("region {:?}", region_pos);
            let sx = if rx == 0 {
                first_chunk.x
            }
            else {
                0
            };
            let sz = if rz == 0 {
                first_chunk.z
            }
            else {
                0
            };
            let ex = if rx == last_region.x - first_region.x - 1 {
                last_chunk.x
            }
            else {
                16
            };
            let ez = if rz == last_region.z - first_region.z - 1 {
                last_chunk.z
            }
            else {
                16
            };

            let chunk_from = V2di32::new(sx, sz);
            let chunk_to = V2di32::new(ex, ez);

            let mut regions = self.world.regions.write();
            if !regions.contains_key(&region_pos) {
                regions.insert(region_pos, Region::new());
            }
            let region = regions.get_mut(&region_pos).unwrap();
            let mut region_file = self.path.clone();
            region_file.push("region");
            region_file.push(&format!("r.{}.{}.mca", region_pos.x, region_pos.z));
            println!("region file {:?}", region_file);
            let mut chunks = Vec::new();
            for (x, z) in (chunk_to - chunk_from).iter(){
                chunks.push(chunk_from + V2di32::new(x, z));
            }
            let data = fs::read(region_file)?;
            region.populate_from_file(&data, &chunks)?;
        }
        Ok(())
    }
}