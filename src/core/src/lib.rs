#![no_std]
extern crate alloc;

mod cursor;
mod rule;

pub mod prelude {
    pub use super::{cursor::Cursor, Deserialize, Parser, Serialize};
}

pub(crate) mod internal_prelude {
    pub use super::prelude::*;
    pub use alloc::vec::Vec;
}

pub trait Deserialize<DataType> {
    fn data_is_valid(input: &DataType) -> bool;
    fn deserialize(input: &mut DataType) -> Self;
}
pub trait Serialize<DataType> {
    fn serialize(self) -> DataType;
}

pub trait Parser<DataType> {
    fn parse(input: DataType) -> Self;
}
