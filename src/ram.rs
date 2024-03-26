//!
//! Definition of the RAM locations on the CC2420 Chip
//! 

#![allow(unused)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM {
    ShortAddress = 0x16A,
    PanID = 0x168,
    IEEEAddress = 0x160,
    TxNonce = 0x140,
    Key1 = 0x130,
    EncryptionBuffer = 0x120,
    RxNonce = 0x110,
    Key0 = 0x100,
    RxFifo = 0x080,
    TxFifo = 0x000,
}

impl RAM {
    /// The start address of a given RAM address sector (for reading)
    pub fn read_address(self) -> (u8, u8) {
        let value = self as u16;
        (
            (0x7F & value) as u8,
            (((0x3 << 7) & value) >> 2) as u8,
        )
    }

    /// The start address of a given RAM address sector (for writing)
    pub fn write_address(self) -> (u8, u8) {
        let mut address = self.read_address();
        address.1 |= 1 << 5;
        address
    }

    /// The Length (in bytes) of a given RAM address sector
    pub fn length(&self) -> usize {
        match self {
            Self::ShortAddress => 2,
            Self::PanID => 2,
            Self::IEEEAddress => 8,
            Self::TxNonce => 16,
            Self::Key1 => 16,
            Self::EncryptionBuffer => 16,
            Self::RxNonce => 16,
            Self::Key0 => 16,
            Self::RxFifo => 128,
            Self::TxFifo => 128,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_read_address_bank_2() {
        let read_address = RAM::ShortAddress.read_address();
        assert_eq!(
            read_address,
            (0b110_1010, 0b1000_0000)
        )
    }

    #[test]
    fn test_get_write_address_bank_2() {
        let write_address = RAM::ShortAddress.write_address();
        assert_eq!(
            write_address,
            (0b1110_1010, 0b1000_0000)
        )
    }

    #[test]
    fn test_get_read_address_bank_1() {
        let read_address = RAM::RxFifo.read_address();
        assert_eq!(
            read_address,
            (0b0000_0000, 0b0100_0000)
        )
    }

    #[test]
    fn test_get_write_address_bank_1() {
        let write_address = RAM::RxFifo.write_address();
        assert_eq!(
            write_address,
            (0b1000_0000, 0b0100_0000)
        )
    }

    #[test]
    fn test_get_read_address_bank_0() {
        let read_address = RAM::TxFifo.read_address();
        assert_eq!(
            read_address,
            (0b0000_0000, 0b0000_0000)
        )
    }

    #[test]
    fn test_get_write_address_bank_0() {
        let write_address = RAM::TxFifo.write_address();
        assert_eq!(
            write_address,
            (0b1000_0000, 0b0000_0000)
        )
    }
}