use reqwest;
use serde::{Deserialize, Serialize};

use crate::{nbt::NBT, blocks, chunks::Chunk};

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

fn palette_to_block(pal_nbt: &NBT) -> Result<blocks::Blocks, String> {
    let pal_map = pal_nbt.get_compound()?;
    // println!("pal_nbt {:?}", pal_nbt);
    let name = match pal_map.get("Name") {
        Some(v) => v.get_string()?,
        None => return Err("block has no name!".to_string()),
    };
    for (n, v) in pal_map.iter() {
        if n != "Name" && n != "Properties" {
            println!("{}: {} - {:?}", name, n, v);
        }
    }
    let nbt = match pal_map.get("Properties") {
        Some(NBT::Compound(t)) => Some(t),
        None => None,
        v => panic!("expected compound properties got {:?}", v)
    };
    Ok(blocks::string_to_block(name, nbt))
}

fn nbt_to_chunk(nbt_data: NBT) -> Result<Vec<Chunk>, String> {
    let data = nbt_data.get_compound()?;
    for (i, v) in data.get("Chunks").unwrap().get_list()?.iter().enumerate() {
        for (n, v) in v.get_compound()?.iter() {
            println!("data.Chunks[{i}].{n} = {}", v.get_type_name());
        }
    }
    let x = match data.get("ChunkX") {
        Some(v) => v.get_int()?,
        None => return Err("no ChunkX in response".to_string()),
    };
    let dx = match data.get("ChunkDX") {
        Some(v) => v.get_int()?,
        None => return Err("no ChunkDX in response".to_string()),
    };
    let z = match data.get("ChunkZ") {
        Some(v) => v.get_int()?,
        None => return Err("no ChunkZ in response".to_string()),
    };
    let dz = match data.get("ChunkDZ") {
        Some(v) => v.get_int()?,
        None => return Err("no ChunkDZ in response".to_string()),
    };
    let chunks = match data.get("Chunks") {
        Some(v) => v.get_list()?,
        None => return Err("no Chunks in response".to_string()),
    };
    let mut res_chunks = Vec::new();
    for chunk_nbt in chunks.iter() {
        let chunk_data = chunk_nbt.get_compound()?;
        let cx = match chunk_data.get("xPos") {
            Some(v) => v.get_int()?,
            None => return Err("no xPos in response".to_string()),
        };
        let cz = match chunk_data.get("zPos") {
            Some(v) => v.get_int()?,
            None => return Err("no zPos in response".to_string()),
        };
        let mut chunk = Chunk::new_empty();
        let sections = match chunk_data.get("sections") {
            Some(v) => v.get_list()?,
            None => return Err("no sections in response".to_string()),
        };
        for section_nbt in sections {
            let section = section_nbt.get_compound()?;
            let cy = match section.get("Y") {
                Some(v) => v.get_byte()?,
                None => return Err("no Y in response".to_string()),
            };
            let block_states = match section.get("block_states") {
                Some(v) => v.get_compound()?,
                None => continue,
            };
            let palette = match block_states.get("palette") {
                Some(v) => v.get_list()?,
                None => return Err("no palette in response".to_string()),
            };
            let mut palette_blocks = Vec::with_capacity(palette.len());
            for p in palette.iter() {
                palette_blocks.push(palette_to_block(p)?);
            }
            let pal_len = palette_blocks.len();
            if pal_len == 1 {
                for x in 0..16 {
                    for y in 0..16 {
                        for z in 0..16 {
                                chunk.sub_chunks[(cy + 4) as usize].blocks[x][y][z] =
                                palette_blocks[0];
                        }
                    }
                }
            } else {
                let data = match block_states.get("data") {
                    Some(v) => v.get_long_array()?,
                    None => return Err("no data in response".to_string()),
                };
                let step_size = if pal_len <= 16 {
                    4
                } else if pal_len <= 32 {
                    5
                } else if pal_len <= 64 {
                    6
                } else if pal_len <= 128 {
                    7
                } else if pal_len <= 256 {
                    8
                } else if pal_len <= 512 {
                    9
                } else if pal_len <= 1024 {
                    10
                } else if pal_len <= 2048 {
                    11
                } else {
                    12
                };
                let mut idx = 0;
                let mut offset = 0;
                for y in 0..16 {
                    for z in 0..16 {
                        for x in 0..16 {
                            if idx >= data.len() {
                                return Err("data array too short!".to_string());
                            }
                            let p_idx = (data[idx] >> offset) & ((1 << step_size) - 1);
                            offset += step_size;
                            if offset + step_size > 64 {
                                offset = 0;
                                idx += 1;
                            }
                            chunk.sub_chunks[cy as usize].blocks[x][y][z] =
                                palette_blocks[p_idx as usize];
                        }
                    }
                }
            }
        }
        res_chunks.push(chunk);
    }
    println!("{} {} {} {}", x, dx, z, dz);
    Ok(res_chunks)
}

pub async fn get_chunk_nbt(x: i64, z: i64, dx: i64, dz: i64) -> Result<Vec<Chunk>, String> {
    let response = match reqwest::get(format!(
        "http://localhost:9000/chunks?x={}&z={}&dx={}&dz={}",
        x, z, dx, dz
    ))
    .await
    {
        Ok(v) => v,
        Err(_) => return Err("request error".to_string()),
    };
    let response_data = match response.bytes().await {
        Ok(v) => v,
        Err(_) => return Err("response error".to_string()),
    };
    let (_nbt_name, nbt_data) = nbt::bytes_to_nbt(&response_data)?;
    nbt_to_chunk(nbt_data)
}