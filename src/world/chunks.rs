use std::array;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use flate2::read::{ZlibDecoder, GzDecoder};
use parking_lot::RwLock;
use byteorder::{BigEndian, ReadBytesExt};

use crate::world::blocks::{Blocks, palette_to_block};
use crate::world::coords::{V2di32, Vec3D};
use crate::nbt::{NBT, bytes_to_nbt};

use super::WorldError;
use super::block_entity::{TileTick, TileEntity};
use super::blocks::block_to_nbt;


pub struct SubChunk {
    pub blocks: [Blocks; 4096],
    pub block_lights: [u64; 256],
    pub sky_lights: [u64; 256],
    pub post_processing: Vec<Vec3D>,
    inhabited_time: u64,
}

impl SubChunk {
}

pub struct Chunk {
    pub sub_chunks: [SubChunk; 24],
    fluid_ticks: Vec<TileTick>,
    block_ticks: Vec<TileTick>,
    motion_blocking_height_map: Option<Box<[i16; 256]>>,
    motion_blocking_no_leaves_height_map: Option<Box<[i16; 256]>>,
    ocean_floor_height_map: Option<Box<[i16; 256]>>,
    world_surface_height_map: Option<Box<[i16; 256]>>,
    tile_entities: Vec<TileEntity>
}

pub struct Region {
    pub chunks: [Option<Box<RwLock<Chunk>>>; 1024]
}

pub struct World {
    pub regions: RwLock<HashMap<V2di32, Region>>
}

fn parse_height_map(height_map: &mut Option<Box<[i16; 256]>>, map: &HashMap<String, NBT>, name: &str) -> Result<(), String> {
    let height_map = match height_map {
        Some(v) => {v.as_mut()},
        None => {
            *height_map = Some(Box::new([0; 256]));
            height_map.as_mut().unwrap().as_mut()
        }
    };
    let data = match map.get(name) {
        Some(NBT::LongArray(v)) => v,
        Some(t) => return Err(format!("'{}' is {} not long array", name, t.get_type_name())),
        None => return Err(format!("no map '{}' in {:?}", name, map.keys())),
    };
    for (i, v) in height_map.iter_mut().enumerate() {
        let w_idx = i / 7;
        let s_idx = i % 7;
        let raw_height = (data[w_idx] >> (s_idx * 9)) & 0x1ff;
        *v = raw_height as i16 - 64;
    }
    Ok(())
}

const fn chunk_linear_coord(x: i32, y: i32, z: i32) -> usize {
    debug_assert!(x >= 0);
    debug_assert!(y >= 0);
    debug_assert!(z >= 0);
    debug_assert!(x < 16);
    debug_assert!(y < 16);
    debug_assert!(z < 16);
    ((z * 16 + y) * 16 + x) as usize
}

impl Chunk {
    pub fn new_empty() -> Self {
        Self {
            sub_chunks: array::from_fn(|_| SubChunk::new()),
            fluid_ticks: Vec::new(),
            block_ticks: Vec::new(),
            motion_blocking_height_map: None,
            motion_blocking_no_leaves_height_map: None,
            ocean_floor_height_map: None,
            world_surface_height_map: None,
            tile_entities: Vec::new(),
        }
    }

    fn populate_from_nbt(&mut self, chunk_data: &HashMap<String, NBT>) -> Result<(), Box<dyn Error>> {
        println!("keys {:?}", chunk_data.keys());
        println!("light {:?}", chunk_data.get("isLightOn"));
        match chunk_data.get("DataVersion") {
            Some(NBT::Int(3120)) => {}
            Some(NBT::Int(i)) => return Err(format!("unknown version {}", i).into()),
            _ => return Err(Box::<WorldError>::new("invalid DataVersion".into())),
        }
        if chunk_data.get("Status") != Some(&NBT::String("full".into())){
            return Err(Box::<WorldError>::new("Status not full".into()));
        }
        let sections = match chunk_data.get("sections") {
            Some(v) => v.get_list()?,
            None => return Err(Box::<WorldError>::new("no sections in response".into())),
        };
        for section in sections.iter() {
            let map = section.get_compound()?;
            let cy = match map.get("Y") {
                Some(v) => v.get_byte()? + 4,
                None => return Err(Box::<WorldError>::new("no Y in response".into())),
            };
            if cy >= 24 || cy < 0 {
                continue;
            }
            self.sub_chunks[cy as usize].parse_sections(map)?;
        }
        match chunk_data.get("Status") {
            Some(NBT::String(s)) if s == "full" => {}
            Some(s) => return Err(Box::<WorldError>::new(format!("status {s:?} not full").into())),
            None => return Err(Box::<WorldError>::new("no chunk status".into())),
        };
        match chunk_data.get("PostProcessing") {
            Some(NBT::List(v)) => {
                for (i, list) in v.iter().enumerate() {
                    self.sub_chunks[i].post_processing.clear();
                    match list {
                        NBT::List(updates) => {
                            for update in updates.iter() {
                                let zyx = update.get_short()? as i32;
                                self.sub_chunks[i].post_processing.push(Vec3D {
                                    x: zyx & 0x1f,
                                    y: (zyx >> 5) & 0x1f,
                                    z: (zyx >> 10) & 0x1f,
                                });
                            }
                        }
                        t => return Err(Box::<WorldError>::new(format!("PostProcessing tag {} not List", t.get_type_name()).into()))
                    }
                }
            },
            None => return Err(Box::<WorldError>::new("PostProcessing tag missing".into())),
            Some(t) => return Err(Box::<WorldError>::new(format!("PostProcessing tag {} not List", t.get_type_name()).into()))
        }
        match chunk_data.get("Heightmaps") {
            Some(NBT::Compound(maps)) => {
                parse_height_map(&mut self.motion_blocking_no_leaves_height_map, maps, "MOTION_BLOCKING_NO_LEAVES")?;
                parse_height_map(&mut self.motion_blocking_height_map, maps, "MOTION_BLOCKING")?;
                parse_height_map(&mut self.ocean_floor_height_map, maps, "OCEAN_FLOOR")?;
                parse_height_map(&mut self.world_surface_height_map, maps, "WORLD_SURFACE")?;
            },
            _ => return Err(Box::<WorldError>::new("Heightmaps tag missing".into())),
        }
        match chunk_data.get("block_entities") {
            Some(NBT::List(v)) => {
                self.tile_entities.clear();
                for nbt in v.iter() {
                    self.tile_entities.push(nbt.try_into()?);
                }
            },
            None => return Err(Box::<WorldError>::new("block_entities tag missing".into())),
            Some(t) => return Err(Box::<WorldError>::new(format!("block_entities tag {} not List", t.get_type_name()).into()))
        }
        match chunk_data.get("fluid_ticks") {
            Some(NBT::List(v)) => {
                self.fluid_ticks.clear();
                for nbt in v.iter() {
                    self.fluid_ticks.push(nbt.try_into()?);
                }
            },
            None => return Err(Box::<WorldError>::new("Entitys tag missing".into())),
            Some(t) => return Err(Box::<WorldError>::new(format!("Entitys tag {} not List", t.get_type_name()).into()))
        }
        match chunk_data.get("block_ticks") {
            Some(NBT::List(v)) => {
                self.block_ticks.clear();
                for nbt in v.iter() {
                    self.block_ticks.push(nbt.try_into()?);
                }
            },
            None => return Err(Box::<WorldError>::new("Entitys tag missing".into())),
            Some(t) => return Err(Box::<WorldError>::new(format!("Entitys tag {} not List", t.get_type_name()).into()))
        }
        return Ok(())
    }

    pub fn get_block(&self, pos: Vec3D) -> Result<Blocks, String> {
        Ok(self.sub_chunks[((pos.y >> 4) + 4) as usize].blocks[chunk_linear_coord(pos.x, pos.y & 15, pos.z)])
    }
}

fn ilog(n: u64) -> u32 {
    min(12, max(4, 64 - (n-1).leading_zeros()))
}

fn fill_light_map(light_data: &mut [u64; 256], list: &Vec<i8>) {
    for (i, light) in light_data.iter_mut().enumerate() {
        *light = (0..8).map(|n| (list[i * 8 + n] as u64) << (n * 8)).reduce(|a, n| a | n).unwrap();
    }
}

impl SubChunk {
    pub fn new() -> Self {
        Self {
            blocks: [Blocks::Air; 4096],
            post_processing: Vec::new(),
            block_lights: [0; 256],
            sky_lights: [0; 256],
            inhabited_time: 0,
        }
    }

    fn parse_sections(&mut self, section: &HashMap<String, NBT>) -> Result<(), Box<dyn Error>> {
        let block_states = match section.get("block_states") {
            Some(v) => v.get_compound()?,
            None => return Ok(()),
        };
        let palette = match block_states.get("palette") {
            Some(v) => v.get_list()?,
            None => return Err(Box::<WorldError>::new("no palette in response".into())),
        };
        let mut palette_blocks = Vec::with_capacity(palette.len());
        for p in palette.iter() {
            palette_blocks.push(palette_to_block(p)?);
        }
        let pal_len = palette_blocks.len() as u64;
        if pal_len == 1 {
            for (x, y, z) in Vec3D::new(16, 16, 16).iter() {
                self.blocks[chunk_linear_coord(x, y, z)] = palette_blocks[0];
            }
        } else {
            let data = match block_states.get("data") {
                Some(v) => v.get_long_array()?,
                None => return Err(Box::<WorldError>::new("no data in response".into())),
            };
            let step_size = ilog(pal_len);
            let mut idx = 0;
            let mut offset = 0;
            for (x, z, y) in Vec3D::new(16, 16, 16).iter() {
                if idx >= data.len() {
                    return Err(Box::<WorldError>::new("data array too short!".into()));
                }
                let p_idx = (data[idx] >> offset) & ((1 << step_size) - 1);
                offset += step_size;
                if offset + step_size > 64 {
                    offset = 0;
                    idx += 1;
                }
                self.blocks[chunk_linear_coord(x, y, z)] =
                    palette_blocks[p_idx as usize];
            }
            println!("len {idx}, {}", data.len());
            println!("pal {step_size}, {}", palette_blocks.len());
            assert!(idx+1 >= data.len())
        }
        // match section.get("BlockLight") {
        //     Some(NBT::ByteArray(v)) => {
        //         fill_light_map(&mut self.block_lights, v);
        //     },
        //     None => return Err(Box::<WorldError>::new("BlockLight tag missing".into())),
        //     Some(t) => return Err(Box::<WorldError>::new(format!("BlockLight tag {} not ByteArray", t.get_type_name()).into()))
        // }
        // match section.get("SkyLight") {
        //     Some(NBT::ByteArray(v)) => {
        //         fill_light_map(&mut self.block_lights, v);
        //     },
        //     None => return Err(Box::<WorldError>::new(format!("SkyLight tag missing {:?}", section.keys()).into())),
        //     Some(t) => return Err(Box::<WorldError>::new(format!("SkyLight tag {} not ByteArray", t.get_type_name()).into()))
        // }
        Ok(())
    }

    pub fn to_nbt(&self) -> NBT {
        let mut map = HashMap::new();
        let mut pal_to_idx = HashMap::<Blocks, u64>::new();
        let mut pal = Vec::new();
        let mut blocks = Vec::with_capacity(4096);
        let mut pal_index = 0;
        for (x, z, y) in Vec3D::new(16, 16, 16).iter() {
            let block = self.blocks[chunk_linear_coord(x, y, z)];
            if !pal_to_idx.contains_key(&block) {
                pal.push(block_to_nbt(&block));
                pal_to_idx.insert(block, pal_index);
                pal_index += 1;
            }
            blocks.push(pal_to_idx.get(&block.clone()).unwrap().clone());
        }
        let stride = ilog(pal_index) as usize;
        let strides_per = 64 / stride;
        let vlen = (4096 - 1) / strides_per + 1;
        let mut data = vec![0; vlen as usize];
        for i in 0..4096 {
            data[i / strides_per] |= (blocks[i] << ((i % strides_per) * stride)) as i64
        }
        map.insert("palette".to_string(), NBT::List(pal));
        map.insert("data".to_string(), NBT::LongArray(data));

        NBT::Compound(map)
    }
}

fn chunk_pos_to_region_idx(x: i32, z: i32) -> usize {
    debug_assert!(x >= 0);
    debug_assert!(z >= 0);
    debug_assert!(x < 32);
    debug_assert!(z < 32);
    (z * 32 + x) as usize
}

impl Region {
    pub fn new() -> Self {
        Self {
            chunks: array::from_fn(|_| None),
        }
    }

    pub fn insert_chunk_nbt(&mut self, chunk_nbt: &HashMap<String, NBT>, chunk_pos: V2di32) -> Result<(), Box<dyn Error>> {
        let chunk_opt = &mut (self.chunks[chunk_pos_to_region_idx(chunk_pos.x, chunk_pos.z)]);
        if chunk_opt.is_none() {
            *chunk_opt = Some(Box::new(RwLock::new(Chunk::new_empty())));
        }
        match chunk_opt {
            Some(t) => {
                t.as_ref().write().populate_from_nbt(chunk_nbt)?;
            },
            None => unreachable!(),
        }
        
        Ok(())
    }
    
    pub fn populate_from_file(&mut self, data: &[u8], chunks: &[V2di32]) -> Result<(),  Box<dyn Error>> {
        for chunk in chunks.iter() {
            println!("chunk {:?}!", chunk);
            let chunk_idx = (chunk.x + chunk.z * 32) as usize;
            let mut chunk_location_slice = &data[(chunk_idx * 4)..];
            let mut data2 = data;
            let chunk_location: u32 = chunk_location_slice.read_u32::<BigEndian>()?;
            for i in 0..1024 {
                let location = data2.read_u32::<BigEndian>()?;
                println!("{:x} {:x}, {location:08x}", i&0x1f, i>>5);
            }
            println!("{}", chunk_idx);
            // println!("{:?}", &data[(chunk_idx * 4)..4096]);
            if chunk_location == 0 {
                return Err(Box::<WorldError>::new(format!("chunk {} {} not generated", chunk.x, chunk.z).into()))
            }
            let chunk_offset = chunk_location >> 8;
            let sector_count = chunk_location & 0xff;
            let chunk_header_slice_pre = &data[((chunk_offset as usize) * 4096)..];
            let mut chunk_header_slice = &chunk_header_slice_pre[..sector_count as usize * 4096];
            let chunk_len = chunk_header_slice.read_u32::<BigEndian>()?;
            let compression = chunk_header_slice[0];
            
            let nbt = match compression {
                1 => {
                    let mut buffer = Vec::new();
                    let mut decoder = GzDecoder::new(&chunk_header_slice[1..(chunk_len as usize)]);
                    decoder.read_to_end(&mut buffer)?;
                    match bytes_to_nbt(&buffer) {
                        Ok((_, data)) => data,
                        Err(s) => return Err(Box::<WorldError>::new(s.into())),
                    }
                }
                2 => {
                    let mut buffer = Vec::new();
                    let mut decoder = ZlibDecoder::new(&chunk_header_slice[1..(chunk_len as usize)]);
                    decoder.read_to_end(&mut buffer)?;
                    match bytes_to_nbt(&buffer) {
                        Ok((_, data)) => data,
                        Err(s) => return Err(Box::<WorldError>::new(s.into())),
                    }
                }
                3 => {
                    match bytes_to_nbt(&chunk_header_slice[1..(chunk_len as usize)]) {
                        Ok((_, data)) => data,
                        Err(s) => return Err(Box::<WorldError>::new(s.into())),
                    }
                }
                t => return Err(Box::<WorldError>::new(format!("unknown compression scheme {}", t).into()))
            };
            let chunk_data = &mut self.chunks[chunk_pos_to_region_idx(chunk.x, chunk.z)];
            if let None = chunk_data {
                *chunk_data = Some(Box::new(RwLock::new(Chunk::new_empty())));
            }
            match &mut self.chunks[chunk_pos_to_region_idx(chunk.x, chunk.z)] {
                Some(chunk_data) => {
                    let map = match nbt.get_compound() {
                        Ok(m) => m,
                        Err(e) => return Err(Box::<WorldError>::new(e.into())),
                    };
                    chunk_data.write().populate_from_nbt(map)?;
                },
                None => unreachable!(),
            }

        }
        Ok(())
    }

    pub fn get_block(&self, pos: Vec3D) -> Result<Blocks, String> {
        let cx = pos.x >> 4;
        let cz = pos.z >> 4;
        match &self.chunks[chunk_pos_to_region_idx(cx, cz)] {
            Some(chunk) => {
                chunk.as_ref().read().get_block(Vec3D::new(pos.x & 15, pos.y, pos.z & 15))
            },
            None => Err(format!("unloaded chunk (x, z): ({}, {})", cx, cz)),
        }
        
    }
}

impl World {
    pub fn new() -> Self {
        Self{
            regions: RwLock::new(HashMap::new())
        }
    }

    pub fn insert_chunk_from_nbt(&mut self, chunk_nbt: &NBT) -> Result<(), Box<dyn Error>> {
        let chunk_data = chunk_nbt.get_compound()?;
        let cx = match chunk_data.get("xPos") {
            Some(v) => v.get_int()?,
            None => return Err(Box::<WorldError>::new("no xPos in response".into())),
        };
        let cz = match chunk_data.get("zPos") {
            Some(v) => v.get_int()?,
            None => return Err(Box::<WorldError>::new("no zPos in response".into())),
        };
        let rx = cx >> 5;
        let rz = cz >> 5;
        let mut regions = self.regions.write();
        let region = regions.entry(V2di32::new(rx, rz)).or_insert_with(|| Region::new());

        region.insert_chunk_nbt(chunk_data, V2di32::new(cx & 31, cz & 31))?;
        Ok(())
    }

    pub fn get_block(&self, pos: Vec3D) -> Result<Blocks, String> {
        // region = 32 * 16 (chunks * blocks) = 9 bits
        let rx = pos.x >> 9;
        let rz = pos.z >> 9;
        match self.regions.read().get(&V2di32::new(rx, rz)) {
            Some(r) => {
                r.get_block(Vec3D::new(pos.x & 0x1ff, pos.y, pos.z & 0x1ff))
            },
            None => Err(format!("unloaded region (x, z): ({}, {})", rx, rz)),
        }
    }
}

#[test]
fn test_log() {
    assert_eq!(ilog(2), 4);
    assert_eq!(ilog(3), 4);
    assert_eq!(ilog(4), 4);
    assert_eq!(ilog(16), 4);
    assert_eq!(ilog(17), 5);
    assert_eq!(ilog(32), 5);
    assert_eq!(ilog(33), 6);
    assert_eq!(ilog(64), 6);
}

