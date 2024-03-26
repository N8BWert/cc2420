//!
//! Rust Definition for the Manufacturer ID Registers.
//! 

use super::Register;

use alloc::string::String;

use derive_builder::Builder;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(error(validation_error = false)))]
pub struct LowerManufacturerID {
    // The device part number.  CC2420 has part number 0x002,
    #[builder(default = "2")]
    pub part_num: u8,
    // Gives the JEDEC manufacturer ID. The actual manufacturer ID
    // can be found in MANIFID[7:1], the number of continuation bytes
    // in MANFID[11:8] and MANFID[0]=1.
    // Chipcon's JEDC manufacturer ID is 0x7F, 0x7F, 0x7F, 0x9E
    #[builder(default = "0x33D")]
    pub manufacturer_id: u16,
}

impl Register for LowerManufacturerID {
    fn register_value(&self) -> u16 {
        0
    }

    fn address(&self) -> u8 { 0x1E }

    fn from_buffer(&mut self, buffer: [u8; 3]) {
        *self = u16::from_le_bytes(buffer[1..3].try_into().unwrap()).into();
    }
}

impl From<u16> for LowerManufacturerID {
    fn from(value: u16) -> Self {
        Self {
            part_num: (((0xF << 12) & value) >> 12) as u8,
            manufacturer_id: value & 0xFFF,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(error(validation_error = false)))]
pub struct UpperManufacturerID {
    // Version number. Current version is 3.
    // Note that previous CC2420 versions will have lower rest
    // values.
    #[builder(default = "2")]
    pub version: u8,
    // The device part number (upper bits).  CC2420 has part number 0x002
    #[builder(default = "0")]
    pub part_num: u16,
}

impl Register for UpperManufacturerID {
    fn register_value(&self) -> u16 {
        0
    }

    fn address(&self) -> u8 { 0x1F }

    fn from_buffer(&mut self, buffer: [u8; 3]) {
        *self = u16::from_le_bytes(buffer[1..3].try_into().unwrap()).into();
    }
}

impl From<u16> for UpperManufacturerID {
    fn from(value: u16) -> Self {
        Self {
            version: (((0xF << 12) & value) >> 12) as u8,
            part_num: value & 0xFFF,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_manufacturer_id_from_u16() {
        let value = 0b0101_0000_0000_0101;

        let expected_lower_manufacturer_id = LowerManufacturerIDBuilder::default()
            .part_num(5)
            .manufacturer_id(5)
            .build()
            .unwrap();

        assert_eq!(
            expected_lower_manufacturer_id,
            value.into(),
        )
    }

    #[test]
    fn test_higher_manufacturer_id_from_u16() {
        let value = 0b0101_0000_0000_0101;

        let expected_upper_manufacturer_id = UpperManufacturerIDBuilder::default()
            .version(5)
            .part_num(5)
            .build()
            .unwrap();

        assert_eq!(
            expected_upper_manufacturer_id,
            value.into(),
        )
    }
}