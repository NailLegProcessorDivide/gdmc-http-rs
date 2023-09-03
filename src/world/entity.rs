use crate::nbt::NBT;



pub struct Entity {
    _data: NBT
}

impl TryFrom<&NBT> for Entity {

    type Error = String;

    fn try_from(nbt: &NBT) -> Result<Self, Self::Error> {
        Ok(Entity {_data: nbt.clone()})
    }
}