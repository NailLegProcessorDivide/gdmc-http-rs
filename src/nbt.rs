use std::collections::HashMap;
use std::str;

use byteorder::{BigEndian, ReadBytesExt};

#[derive(PartialEq, Debug, Clone)]
pub enum NBT {
    Byte(i8),
    Bool(bool),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    List(Vec<NBT>),
    Compound(HashMap<String, NBT>),
    ByteArray(Vec<i8>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl NBT {
    pub fn get_type_name(self: &Self) -> &str {
        match self {
            NBT::Byte(_) => "byte",
            NBT::Bool(_) => "bool",
            NBT::Short(_) => "short",
            NBT::Int(_) => "int",
            NBT::Long(_) => "long",
            NBT::Float(_) => "float",
            NBT::Double(_) => "double",
            NBT::ByteArray(_) => "byteArray",
            NBT::String(_) => "string",
            NBT::List(_) => "list",
            NBT::Compound(_) => "compound",
            NBT::IntArray(_) => "intArray",
            NBT::LongArray(_) => "longArray",
        }
    }

    pub fn get_byte(self: &Self) -> Result<i8, String> {
        match self {
            NBT::Byte(b) => Ok(*b),
            _ => Err(format!("expected byte got {}!", self.get_type_name())),
        }
    }

    pub fn get_bool(self: &Self) -> Result<bool, String> {
        match self {
            NBT::Bool(b) => Ok(*b),
            _ => Err(format!("expected bool got {}!", self.get_type_name())),
        }
    }

    pub fn get_short(self: &Self) -> Result<i16, String> {
        match self {
            NBT::Short(b) => Ok(*b),
            _ => Err(format!("expected short got {}!", self.get_type_name())),
        }
    }

    pub fn get_int(self: &Self) -> Result<i32, String> {
        match self {
            NBT::Int(b) => Ok(*b),
            _ => Err(format!("expected int got {}!", self.get_type_name())),
        }
    }

    pub fn get_long(self: &Self) -> Result<i64, String> {
        match self {
            NBT::Long(b) => Ok(*b),
            _ => Err(format!("expected long got {}!", self.get_type_name())),
        }
    }

    pub fn get_float(self: &Self) -> Result<f32, String> {
        match self {
            NBT::Float(b) => Ok(*b),
            _ => Err(format!("expected float got {}!", self.get_type_name())),
        }
    }

    pub fn get_double(self: &Self) -> Result<f64, String> {
        match self {
            NBT::Double(b) => Ok(*b),
            _ => Err(format!("expected double got {}!", self.get_type_name())),
        }
    }

    pub fn get_byte_array(self: &Self) -> Result<&Vec<i8>, String> {
        match self {
            NBT::ByteArray(b) => Ok(b),
            _ => Err(format!("expected byte array got {}!", self.get_type_name())),
        }
    }

    pub fn get_string(self: &Self) -> Result<&str, String> {
        match self {
            NBT::String(b) => Ok(b),
            _ => Err(format!("expected string got {}!", self.get_type_name())),
        }
    }

    pub fn get_list(self: &Self) -> Result<&Vec<NBT>, String> {
        match self {
            NBT::List(b) => Ok(b),
            _ => Err(format!("expected list got {}!", self.get_type_name())),
        }
    }

    pub fn get_compound(self: &Self) -> Result<&HashMap<String, NBT>, String> {
        match self {
            NBT::Compound(b) => Ok(b),
            _ => Err(format!("expected compound got {}!", self.get_type_name())),
        }
    }

    pub fn get_int_array(self: &Self) -> Result<&Vec<i32>, String> {
        match self {
            NBT::IntArray(b) => Ok(b),
            _ => Err(format!("expected int array got {}!", self.get_type_name())),
        }
    }

    pub fn get_long_array(self: &Self) -> Result<&Vec<i64>, String> {
        match self {
            NBT::LongArray(b) => Ok(b),
            _ => Err(format!("expected long array got {}!", self.get_type_name())),
        }
    }
}

fn parse_naked_string(data: &[u8]) -> Result<(String, &[u8]), String> {
    match data {
        [l1, l2, dat @ ..] => {
            let len = (*l1 as u16) * 0x100 + (*l2 as u16);
            if dat.len() < (len as usize) {
                return Err("end of string after end of file".to_string());
            }
            match str::from_utf8(&dat[0..(len as usize)]) {
                Ok(s) => Ok((s.to_string(), &dat[(len as usize)..])),
                Err(_) => Err("non utf8 string".to_string()),
            }
        }
        _ => Err("error decoding string length, EOF".to_string()),
    }
}

fn parse_byte(data: &[u8]) -> Result<(NBT, &[u8]), String> {
    if data.len() == 0 {
        return Err("expected byte found eof".to_string());
    }
    Ok((NBT::Byte(data[0] as i8), &data[1..]))
}

fn parse_short(mut data: &[u8]) -> Result<(NBT, &[u8]), String> {
    match data.read_i16::<BigEndian>() {
        Ok(v) => Ok((NBT::Short(v), data)),
        Err(_) => Err("i16 parse error".to_string()),
    }
}

fn parse_int(mut data: &[u8]) -> Result<(NBT, &[u8]), String> {
    match data.read_i32::<BigEndian>() {
        Ok(v) => Ok((NBT::Int(v), data)),
        Err(_) => Err("i32 parse error".to_string()),
    }
}

fn parse_long(mut data: &[u8]) -> Result<(NBT, &[u8]), String> {
    match data.read_i64::<BigEndian>() {
        Ok(v) => Ok((NBT::Long(v), data)),
        Err(_) => Err("i64 parse error".to_string()),
    }
}

fn parse_float(mut data: &[u8]) -> Result<(NBT, &[u8]), String> {
    match data.read_f32::<BigEndian>() {
        Ok(v) => Ok((NBT::Float(v), data)),
        Err(_) => Err("i32 parse error".to_string()),
    }
}

fn parse_double(mut data: &[u8]) -> Result<(NBT, &[u8]), String> {
    match data.read_f64::<BigEndian>() {
        Ok(v) => Ok((NBT::Double(v), data)),
        Err(_) => Err("f64 parse error".to_string()),
    }
}

fn parse_byte_array(mut data: &[u8]) -> Result<(NBT, &[u8]), String> {
    let len = match data.read_i32::<BigEndian>() {
        Ok(v) => v,
        Err(_) => return Err("i32 parse error".to_string()),
    };
    let mut res = Vec::new();
    for _ in 0..(len as usize) {
        match data.read_i8() {
            Ok(v) => res.push(v),
            Err(_) => return Err("i64 array parse error".to_string()),
        };
    }
    Ok((NBT::ByteArray(res), data))
}

fn parse_string(data: &[u8]) -> Result<(NBT, &[u8]), String> {
    let (dat, data) = parse_naked_string(data)?;
    Ok((NBT::String(dat), data))
}

fn parse_list(mut data: &[u8]) -> Result<(NBT, &[u8]), String> {
    if data.len() == 0 {
        return Err("expected byte found eof".to_string());
    }
    let tag = data[0];
    data = &data[1..];
    let len = match data.read_i32::<BigEndian>() {
        Ok(v) => v,
        Err(_) => return Err("i32 parse error".to_string()),
    };
    let mut res = Vec::new();
    for _ in 0..(len as usize) {
        match tag {
            0x0 => return Err("unexpected null tag".to_string()),
            0x1 => {
                let (b, dat) = parse_byte(data)?;
                res.push(b);
                data = dat;
            }
            0x2 => {
                let (b, dat) = parse_short(data)?;
                res.push(b);
                data = dat;
            }
            0x3 => {
                let (b, dat) = parse_int(data)?;
                res.push(b);
                data = dat;
            }
            0x4 => {
                let (b, dat) = parse_long(data)?;
                res.push(b);
                data = dat;
            }
            0x5 => {
                let (b, dat) = parse_float(data)?;
                res.push(b);
                data = dat;
            }
            0x6 => {
                let (b, dat) = parse_double(data)?;
                res.push(b);
                data = dat;
            }
            0x7 => {
                let (b, dat) = parse_byte_array(data)?;
                res.push(b);
                data = dat;
            }
            0x8 => {
                let (b, dat) = parse_string(data)?;
                res.push(b);
                data = dat;
            }
            0x9 => {
                let (b, dat) = parse_list(data)?;
                res.push(b);
                data = dat;
            }
            0xa => {
                let (b, dat) = parse_compound(data)?;
                res.push(b);
                data = dat;
            }
            0xb => {
                let (b, dat) = parse_int_array(data)?;
                res.push(b);
                data = dat;
            }
            0xc => {
                let (b, dat) = parse_long_array(data)?;
                res.push(b);
                data = dat;
            }
            _ => return Err("unknown tag or eof reading list body".to_string()),
        }
    }
    Ok((NBT::List(res), data))
}

fn parse_compound(mut data: &[u8]) -> Result<(NBT, &[u8]), String> {
    let mut map = HashMap::new();
    loop {
        match data {
            [0x0, dat @ ..] => return Ok((NBT::Compound(map), dat)),
            [0x1, dat @ ..] => {
                let (name, dat) = parse_naked_string(dat)?;
                let (b, dat) = parse_byte(dat)?;
                map.insert(name, b);
                data = dat;
            }
            [0x2, dat @ ..] => {
                let (name, dat) = parse_naked_string(dat)?;
                let (b, dat) = parse_short(dat)?;
                map.insert(name, b);
                data = dat;
            }
            [0x3, dat @ ..] => {
                let (name, dat) = parse_naked_string(dat)?;
                let (b, dat) = parse_int(dat)?;
                map.insert(name, b);
                data = dat;
            }
            [0x4, dat @ ..] => {
                let (name, dat) = parse_naked_string(dat)?;
                let (b, dat) = parse_long(dat)?;
                map.insert(name, b);
                data = dat;
            }
            [0x5, dat @ ..] => {
                let (name, dat) = parse_naked_string(dat)?;
                let (b, dat) = parse_float(dat)?;
                map.insert(name, b);
                data = dat;
            }
            [0x6, dat @ ..] => {
                let (name, dat) = parse_naked_string(dat)?;
                let (b, dat) = parse_double(dat)?;
                map.insert(name, b);
                data = dat;
            }
            [0x7, dat @ ..] => {
                let (name, dat) = parse_naked_string(dat)?;
                let (b, dat) = parse_byte_array(dat)?;
                map.insert(name, b);
                data = dat;
            }
            [0x8, dat @ ..] => {
                let (name, dat) = parse_naked_string(dat)?;
                let (b, dat) = parse_string(dat)?;
                map.insert(name, b);
                data = dat;
            }
            [0x9, dat @ ..] => {
                let (name, dat) = parse_naked_string(dat)?;
                let (b, dat) = parse_list(dat)?;
                map.insert(name, b);
                data = dat;
            }
            [0xa, dat @ ..] => {
                let (name, dat) = parse_naked_string(dat)?;
                let (b, dat) = parse_compound(dat)?;
                map.insert(name, b);
                data = dat;
            }
            [0xb, dat @ ..] => {
                let (name, dat) = parse_naked_string(dat)?;
                let (b, dat) = parse_int_array(dat)?;
                map.insert(name, b);
                data = dat;
            }
            [0xc, dat @ ..] => {
                let (name, dat) = parse_naked_string(dat)?;
                let (b, dat) = parse_long_array(dat)?;
                map.insert(name, b);
                data = dat;
            }
            _ => return Err("unknown byte or eof reading compound body".to_string()),
        }
    }
}

fn parse_int_array(mut data: &[u8]) -> Result<(NBT, &[u8]), String> {
    let len = match data.read_i32::<BigEndian>() {
        Ok(v) => v,
        Err(_) => return Err("i32 parse error".to_string()),
    };
    let mut res = Vec::new();
    for _ in 0..(len as usize) {
        match data.read_i32::<BigEndian>() {
            Ok(v) => res.push(v),
            Err(_) => return Err("i32 array parse error".to_string()),
        };
    }
    Ok((NBT::IntArray(res), data))
}

fn parse_long_array(mut data: &[u8]) -> Result<(NBT, &[u8]), String> {
    let len = match data.read_i32::<BigEndian>() {
        Ok(v) => v,
        Err(_) => return Err("i32 parse error".to_string()),
    };
    let mut res = Vec::new();
    for _ in 0..(len as usize) {
        match data.read_i64::<BigEndian>() {
            Ok(v) => res.push(v),
            Err(_) => return Err("i64 array parse error".to_string()),
        };
    }
    Ok((NBT::LongArray(res), data))
}

pub fn bytes_to_nbt(data: &[u8]) -> Result<(String, NBT), String> {
    match data {
        [0x0, ..] => Err("unexpected null tag".to_string()),
        [0x1, dat @ ..] => {
            let (name, dat) = parse_naked_string(dat)?;
            let (b, _) = parse_byte(dat)?;
            Ok((name, b))
        }
        [0x2, dat @ ..] => {
            let (name, dat) = parse_naked_string(dat)?;
            let (b, _) = parse_short(dat)?;
            Ok((name, b))
        }
        [0x3, dat @ ..] => {
            let (name, dat) = parse_naked_string(dat)?;
            let (b, _) = parse_int(dat)?;
            Ok((name, b))
        }
        [0x4, dat @ ..] => {
            let (name, dat) = parse_naked_string(dat)?;
            let (b, _) = parse_long(dat)?;
            Ok((name, b))
        }
        [0x5, dat @ ..] => {
            let (name, dat) = parse_naked_string(dat)?;
            let (b, _) = parse_float(dat)?;
            Ok((name, b))
        }
        [0x6, dat @ ..] => {
            let (name, dat) = parse_naked_string(dat)?;
            let (b, _) = parse_double(dat)?;
            Ok((name, b))
        }
        [0x7, dat @ ..] => {
            let (name, dat) = parse_naked_string(dat)?;
            let (b, _) = parse_byte_array(dat)?;
            Ok((name, b))
        }
        [0x8, dat @ ..] => {
            let (name, dat) = parse_naked_string(dat)?;
            let (b, _) = parse_string(dat)?;
            Ok((name, b))
        }
        [0x9, dat @ ..] => {
            let (name, dat) = parse_naked_string(dat)?;
            let (b, _) = parse_list(dat)?;
            Ok((name, b))
        }
        [0xa, dat @ ..] => {
            let (name, dat) = parse_naked_string(dat)?;
            let (b, _) = parse_compound(dat)?;
            Ok((name, b))
        }
        [0xb, dat @ ..] => {
            let (name, dat) = parse_naked_string(dat)?;
            let (b, _) = parse_int_array(dat)?;
            Ok((name, b))
        }
        [0xc, dat @ ..] => {
            let (name, dat) = parse_naked_string(dat)?;
            let (b, _) = parse_long_array(dat)?;
            Ok((name, b))
        }
        _ => return Err("unknown byte or eof reading compound body".to_string()),
    }
}
