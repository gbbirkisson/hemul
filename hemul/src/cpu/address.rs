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
