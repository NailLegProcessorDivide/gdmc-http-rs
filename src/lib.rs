use chunks::Chunk;
use nbt::NBT;

pub mod file_loader;
pub mod net_loader;
pub mod blocks;
pub mod chunks;
pub mod coords;
pub mod nbt;


// pub async fn get_block(x: i32, y: i32, z: i32) -> Result<blocks::Blocks, reqwest::Error> {
//     //let block_data: Vec<BlockData> = reqwest::get(format!("http://localhost:9000/blocks?x={}&y={}&z={}", x, y, z)).await?.json().await?;
//     let block_data: Vec<BlockData> = reqwest::get(format!(
//         "http://localhost:9000/blocks?x={}&y={}&z={}&includeState=true&includeData=true",
//         x, y, z
//     ))
//     .await?
//     .json()
//     .await?;
//     //let block_data: BlockData = serde_json::from_str(&res).unwrap();
//     let block = blocks::string_to_block(&block_data.first().unwrap().id);
//     println!("{}", block_data.first().unwrap().id);
//     Ok(block)
// }

// pub async fn get_blocks(
//     x: i32,
//     y: i32,
//     z: i32,
//     dx: i32,
//     dy: i32,
//     dz: i32,
// ) -> Result<Vec<Vec<Vec<blocks::Blocks>>>, reqwest::Error> {
//     //let block_data: Vec<BlockData> = reqwest::get(format!("http://localhost:9000/blocks?x={}&y={}&z={}&dx={}&dy={}&dz={}", x, y, z, dx, dy, dz)).await?.json().await?;
//     let block_data: Vec<BlockData> = reqwest::get(format!("http://localhost:9000/blocks?x={}&y={}&z={}&dx={}&dy={}&dz={}&includeState=true&includeData=true", x, y, z, dx, dy, dz)).await?.json().await?;
//     //let block_data: BlockData = serde_json::from_str(&res).unwrap();
//     let mut blocks = vec![vec![vec![blocks::Blocks::Air; dz as usize]; dy as usize]; dx as usize];
//     for block in block_data {
//         blocks[(block.x - x) as usize][(block.y - y) as usize][(block.z - z) as usize] =
//             blocks::string_to_block(&block.id);
//         if block.data.is_some() && block.data.as_ref().unwrap() != "{}" {
//             println!("{} {}", &block.id, block.data.unwrap());
//         }
//     }
//     Ok(blocks)
// }

// pub async fn get_chunk(x: i32, z: i32) -> Result<blocks::Chunk, reqwest::Error> {
//     let vecs = get_blocks(x * 16, -64, z * 16, 16, 320, 16).await?;
//     let mut c: blocks::Chunk = blocks::Chunk::new(x, z);
//     for (x, v1) in vecs.iter().enumerate() {
//         for (y, v2) in v1.iter().enumerate() {
//             for (z, b) in v2.iter().enumerate() {
//                 c.blocks[x][y][z] = *b;
//             }
//         }
//     }
//     println!("chunk {} {}", x, z);
//     Ok(c)
// }



#[cfg(test)]
mod tests {}
