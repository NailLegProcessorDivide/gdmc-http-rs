use std::{error::Error, fmt::Display};

pub mod block_entity;
pub mod blocks;
pub mod chunks;
pub mod coords;
pub mod entity;
pub mod items;

#[derive(Debug)]
pub struct WorldError{
    msg: String
}

impl Display for WorldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl Error for WorldError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        &self.msg
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl From<&str> for WorldError {
    fn from(value: &str) -> Self {
        Self { msg: value.to_string() }
    }
}

impl From<String> for WorldError {
    fn from(value: String) -> Self {
        Self { msg: value }
    }
}

impl From<&String> for WorldError {
    fn from(value: &String) -> Self {
        Self { msg: value.clone() }
    }
}
