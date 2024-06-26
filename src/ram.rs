//!
//! Definition of the RAM locations on the CC2420 Chip
//! 

#![allow(unused)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ram {
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

impl Ram {
    /// The start address of a given RAM address sector (for reading)
    pub fn read_address(self) -> (u8, u8) {
        let value = self as u16;
        (
            ((0x7F & value) | (1 << 7)) as u8,
            (((0x3 << 7) & value) >> 1 | (1 << 5)) as u8,
        )
    }

    /// The start address of a given RAM address sector (for writing)
    pub fn write_address(self) -> (u8, u8) {
        let value = self as u16;
        (
            ((0x7F & value) | (1 << 7)) as u8,
            (((0x3 << 7) & value) >> 1) as u8,
        )
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
        let read_address = Ram::ShortAddress.read_address();
        assert_eq!(
            read_address,
            (0b1110_1010, 0b1010_0000)
        )
    }

    #[test]
    fn test_get_write_address_bank_2() {
        let write_address = Ram::ShortAddress.write_address();
        assert_eq!(
            write_address,
            (0b1110_1010, 0b1000_0000)
        )
    }

    #[test]
    fn test_get_read_address_bank_1() {
        let read_address = Ram::RxFifo.read_address();
        assert_eq!(
            read_address,
            (0b1000_0000, 0b0110_0000)
        )
    }

    #[test]
    fn test_get_write_address_bank_1() {
        let write_address = Ram::RxFifo.write_address();
        assert_eq!(
            write_address,
            (0b1000_0000, 0b0100_0000)
        )
    }

    #[test]
    fn test_get_read_address_bank_0() {
        let read_address = Ram::TxFifo.read_address();
        assert_eq!(
            read_address,
            (0b1000_0000, 0b0010_0000)
        )
    }

    #[test]
    fn test_get_write_address_bank_0() {
        let write_address = Ram::TxFifo.write_address();
        assert_eq!(
            write_address,
            (0b1000_0000, 0b0000_0000)
        )
    }
}