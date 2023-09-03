use crate::nbt::NBT;

pub struct TileEntity {
    data: NBT
}

impl TryFrom<&NBT> for TileEntity {

    type Error = String;

    fn try_from(value: &NBT) -> Result<Self, Self::Error> {
        Ok(Self { data: value.clone() })
    }
}

#[allow(dead_code)]
pub struct TileTick {
    id: String,
    phase: i32,
    tick: i32,
    x: i32,
    y: i32,
    z: i32
}

impl TryFrom<&NBT> for TileTick {

    type Error = String;

    fn try_from(value: &NBT) -> Result<Self, Self::Error> {
        let compound = value.get_compound()?;
        let id = match compound.get("i") {
            Some(t) => t.get_string()?.to_string(),
            None => return Err("no i tag in Tile_tick".to_string())
        };
        let phase = match compound.get("p") {
            Some(t) => t.get_int()?,
            None => return Err("no p tag in Tile_tick".to_string())
        };
        let tick = match compound.get("t") {
            Some(t) => t.get_int()?,
            None => return Err("no t tag in Tile_tick".to_string())
        };
        let x = match compound.get("x") {
            Some(t) => t.get_int()?,
            None => return Err("no x tag in Tile_tick".to_string())
        };
        let y = match compound.get("y") {
            Some(t) => t.get_int()?,
            None => return Err("no y tag in Tile_tick".to_string())
        };
        let z = match compound.get("z") {
            Some(t) => t.get_int()?,
            None => return Err("no z tag in Tile_tick".to_string())
        };
        Ok(TileTick {
            id,
            phase,
            tick,
            x,
            y,
            z })
    }
}