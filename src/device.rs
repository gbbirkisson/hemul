use std::ops::{Index, IndexMut};

pub type Word = u16;
pub type Byte = u8;

#[derive(Debug, Clone)]
pub enum Address {
    Short(Byte),
    Full(Byte, Byte)
}

impl From<Address> for Word {
    fn from(value: Address) -> Self {
        let (addr, page) = match value {
            Address::Short(_) => todo!("(addr, 0)?"),
            Address::Full(addr, page) => (addr, page),
        };
        let addr = u16::from(addr);
        let page = u16::from(page);
        page << 8 | addr
    }
}

impl From<(Byte, Byte)> for Address {
    fn from(value: (Byte, Byte)) -> Self {
        Address::Full(value.0, value.1)
    }
}

pub trait Addressable: Index<Word, Output = Byte> + IndexMut<Word, Output = Byte> {}

pub trait Tickable {
    fn tick(&mut self);
}

pub type InteruptType = u8;

pub trait Interuptable {
    fn interupt(&mut self, t: InteruptType);
}
