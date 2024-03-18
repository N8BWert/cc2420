//!
//! RSSI and CCA Status and Control Register
//! 

use super::RegisterValue;

use alloc::string::String;

use derive_builder::Builder;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(error(validation_error = false)))]
pub struct RSSIRegister {
    // Clear Channel Assessment threshold value, signed number on
    // 2's complement for comparison with the RSSI.
    // The unit is 1dB, offset is the same as for RSSI_VAL.  The CCA
    // signal goes active when the received signal is below this value.
    // The CCA signal is available on the CCA pin.
    // The reset value is approximately -77dBm.
    #[builder(default = "-32")]
    pub cca_threshold: i8,
    // RSSI estimate on a logarithmic scale, signed number on 2's complement
    // Unit is 1dB, offset is described in the RSSI / Energy Detection
    // section on page 48 of the datasheet.
    // The RSSI_VAL value is averaged over 8 symbol periods. The RSSI_VALID
    // status bit may be checked to verify that the receiver has been enabled for
    // at least 8 symbol periods.
    // The reset value of -128 also indicates that the RSSI_VAL value
    // is invalid.
    // READ-ONLY
    #[builder(default = "-128")]
    pub rssi_value: i8,
}

impl RegisterValue for RSSIRegister {
    fn register_value(&self) -> u16 {
        let mut value = 0;

        value |= (self.cca_threshold.to_be_bytes()[0] as u16) << 8;

        value
    }
}

impl From<u16> for RSSIRegister {
    fn from(value: u16) -> Self {
        Self {
            cca_threshold: i8::from_be_bytes([((value & 0xFF00) >> 8) as u8]),
            rssi_value: i8::from_be_bytes([(value & 0x00FF) as u8]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cca_value() {
        let rssi_register = RSSIRegisterBuilder::default()
            .cca_threshold(-22)
            .build()
            .unwrap();

        assert_eq!(
            rssi_register.register_value(),
            0b11101010_00000000,
        )
    }

    #[test]
    fn test_rssi_register_from_u16() {
        let value: u16 = 0x1294;

        let expected_register = RSSIRegisterBuilder::default()
            .cca_threshold(18)
            .rssi_value(-108)
            .build()
            .unwrap();

        assert_eq!(
            expected_register,
            value.into(),
        )
    }
}