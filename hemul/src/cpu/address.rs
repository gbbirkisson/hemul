use crate::{Byte, Word};

/// The 6502 processor expects addresses to be stored in 'little endian' order, with the least
/// significant byte first and the most significant byte second.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Address {
    /// Zero page address, so page is always 0
    Zero(Byte),

    /// Full address so address is first and page is second
    Full(Byte, Byte),
}

impl From<Address> for Word {
    fn from(value: Address) -> Self {
        let (addr, page) = match value {
            Address::Zero(addr) => (addr, 0),
            Address::Full(addr, page) => (addr, page),
        };
        let addr = Self::from(addr);
        let page = Self::from(page);
        (page << 8) | addr
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_page() {
        assert_eq!(0x66 as Word, Address::Zero(0x66).into());
    }

    #[test]
    fn test_full() {
        assert_eq!(0x8866 as Word, Address::Full(0x66, 0x88).into());
    }

    #[test]
    fn test_from_word() {
        assert_eq!(Address::from(0x8866), Address::Full(0x66, 0x88));
    }

    #[test]
    fn test_from_bytes() {
        assert_eq!(Address::from((0x66, 0x88)), Address::Full(0x66, 0x88));
    }
}
