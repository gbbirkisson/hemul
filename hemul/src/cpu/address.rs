use crate::{Byte, Word};

#[derive(Debug, Clone)]
pub enum Address {
    Short(Byte),
    Full(Byte, Byte),
}

impl From<Address> for Word {
    fn from(value: Address) -> Self {
        let (addr, page) = match value {
            Address::Short(_) => todo!("(addr, 0)?"),
            Address::Full(addr, page) => (addr, page),
        };
        let addr = Self::from(addr);
        let page = Self::from(page);
        page << 8 | addr
    }
}

impl From<(Byte, Byte)> for Address {
    fn from(value: (Byte, Byte)) -> Self {
        Self::Full(value.0, value.1)
    }
}

impl From<Word> for Address {
    fn from(value: Word) -> Self {
        let addr = value & 0b0000_0000_1111_1111;
        let page = value >> 8;
        Self::Full(addr as u8, page as u8)
    }
}
