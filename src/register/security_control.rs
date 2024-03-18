//!
//! Rust Definition of the Security Control Register
//! 

use super::{Register, RegisterValue};

use alloc::string::String;

use derive_builder::Builder;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(validate = "Self::validate"))]
pub struct SecurityControlRegister0 {
    // Protection enable of the RXFIFO, see description in the RXFIFO
    // overflow section on page 33. Should be cleared if MAC level
    // security is not used or is implemented outside CC2420.
    #[builder(default = "true")]
    rx_fifo_protection: bool,
    // Defines what to use for the first byte in CBC-MAC (does not
    // apply to CBC-MAC part of CCM):
    // 0: Use the first data byte as the first byte into CBC-MAC
    // 1: Use the length of the data to be authenticated (calculated as
    // the packet length field - SEC_TXL - 2) for TX or using 
    // SEC_RXL for RX) as the first byte into CBC-MAC (before the first
    // data byte).
    // This bit should be set high for CBC-MAC 802.15.4 inline security.
    #[builder(default = "true")]
    sec_cbc_head: bool,
    // Stand Alone Key select
    // 0: Key 0 is used
    // 1: Key 1 is used
    #[builder(default = "true")]
    sec_sa_key_sel: bool,
    // TX Key select
    // 0: Key 0 is used
    // 1: Key 1 is used
    #[builder(default = "true")]
    sec_tx_key_sel: bool,
    // RX Key select
    // 0: Key 0 is used
    // 1: Key 1 is used
    #[builder(default = "false")]
    sec_rx_key_sel: bool,
    // Number of bytes in authentication field for CBC-MAC, encoded
    // as (M-2)/2
    // 0: Reserved
    // 1: 4
    // ...
    // 7: 16
    #[builder(default = "1")]
    sec_m: u8,
    // Security Mode
    // 0: In-line security is disabled
    // 1: CBC-MAC
    // 2: CTR
    // 3: CCM
    #[builder(default = "0")]
    sec_mode: u8,
}

impl RegisterValue for SecurityControlRegister0 {
    fn register_value(&self) -> u16 {
        let mut value = 0;

        if self.rx_fifo_protection {
            value |= 1 << 9;
        }

        if self.sec_cbc_head {
            value |= 1 << 8;
        }

        if self.sec_sa_key_sel {
            value |= 1 << 7;
        }

        if self.sec_tx_key_sel {
            value |= 1 << 6;
        }

        if self.sec_rx_key_sel {
            value |= 1 << 5;
        }

        value |= (self.sec_m as u16) << 2;

        value |= self.sec_mode as u16;

        value
    }
}

impl From<u16> for SecurityControlRegister0 {
    fn from(value: u16) -> Self {
        Self {
            rx_fifo_protection: (((1 << 9) & value) != 0),
            sec_cbc_head: (((1 << 8) & value) != 0),
            sec_sa_key_sel: (((1 << 7) & value) != 0),
            sec_tx_key_sel: (((1 << 6) & value) != 0),
            sec_rx_key_sel: (((1 << 5) & value) != 0),
            sec_m: (((0b111 << 2) & value) >> 2) as u8,
            sec_mode: (0b11 & value) as u8,
        }
    }
}

impl SecurityControlRegister0Builder {
    fn validate(&self) -> Result<(), String> {
        if let Some(sec_m) = self.sec_m {
            if sec_m == 0 || sec_m > 7 {
                return Err("Invalid SEC_M. Expected 1<=SEC_M<=7".into());
            }
        }

        if let Some(sec_mode) = self.sec_mode {
            if sec_mode > 3 {
                return Err("Invalid SEC_MODE. Expected 0<=SEC_MODE<=3".into());
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(validate = "Self::validate"))]
pub struct SecurityControlRegister1 {
    // Multi-purpose length byte for TX in-line security operations:
    // CTR: Number of cleartext bytes between length byte and the
    //      first byte to be encrypted
    // CBC/MAC: Number of cleartext bytes between length byte and
    //      the first byte to be authenticated
    // CCM: I(a), defining the number of bytes to be authenticated but
    //      not encrypted
    // Stand-alone: SEC_TXL has not effect
    #[builder(default = "0")]
    pub sec_txl: u8,
    // Multi-prupose length byte for RX in-line security operations.
    // CTR: Number of cleartext bytes between length byte and the
    //      first byte to be decrypted
    // CMC/MAC: Number of cleartext bytes between length byte and
    //      the first byte to be authenticated
    // CCM: I(a), defining the number of bytes to be authenticated but
    //      not decrypted
    // Stand-alone: SEC_RXL has not effect
    #[builder(default = "0")]
    pub sec_rxl: u8,
}

impl RegisterValue for SecurityControlRegister1 {
    fn register_value(&self) -> u16 {
        let mut value = 0;

        value |= (self.sec_txl as u16) << 8;

        value |= self.sec_rxl as u16;

        value
    }
}

impl From<u16> for SecurityControlRegister1 {
    fn from(value: u16) -> Self {
        Self {
            sec_txl: (((0x7F << 8) & value) >> 8) as u8,
            sec_rxl: (0x7F & value) as u8,
        }
    }
}

impl SecurityControlRegister1Builder {
    fn validate(&self) -> Result<(), String> {
        if let Some(sec_txl) = self.sec_txl {
            if sec_txl > 127 {
                return Err("Invalid SEC_TXL. 0<=SEC_TXL<=127".into());
            }
        }

        if let Some(sec_rxl) = self.sec_rxl {
            if sec_rxl > 127 {
                return Err("Invalid SEC_RXL. 0<=SEC_RXL<=127".into());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rx_fifo_protection_value() {
        let security_control = SecurityControlRegister0Builder::default()
            .rx_fifo_protection(false)
            .build()
            .unwrap();

        assert_eq!(
            security_control.register_value(),
            0b0_1_1_1_0_001_00,
        )
    }

    #[test]
    fn test_sec_cbc_head_value() {
        let security_control = SecurityControlRegister0Builder::default()
            .sec_cbc_head(false)
            .build()
            .unwrap();

        assert_eq!(
            security_control.register_value(),
            0b1_0_1_1_0_001_00,
        )
    }

    #[test]
    fn test_sec_sa_key_sel_value() {
        let security_control = SecurityControlRegister0Builder::default()
            .sec_sa_key_sel(false)
            .build()
            .unwrap();

        assert_eq!(
            security_control.register_value(),
            0b1_1_0_1_0_001_00,
        )
    }

    #[test]
    fn test_tx_key_sel_value() {
        let security_control = SecurityControlRegister0Builder::default()
            .sec_tx_key_sel(false)
            .build()
            .unwrap();

        assert_eq!(
            security_control.register_value(),
            0b1_1_1_0_0_001_00,
        )
    }

    #[test]
    fn test_rx_key_sel_value() {
        let security_control = SecurityControlRegister0Builder::default()
            .sec_rx_key_sel(true)
            .build()
            .unwrap();

        assert_eq!(
            security_control.register_value(),
            0b1_1_1_1_1_001_00,
        )
    }

    #[test]
    fn test_sec_m_value() {
        let security_control = SecurityControlRegister0Builder::default()
            .sec_m(7)
            .build()
            .unwrap();

        assert_eq!(
            security_control.register_value(),
            0b1_1_1_1_0_111_00,
        )
    }

    #[test]
    fn test_sec_mode_value() {
        let security_control = SecurityControlRegister0Builder::default()
            .sec_mode(3)
            .build()
            .unwrap();

        assert_eq!(
            security_control.register_value(),
            0b1_1_1_1_0_001_11,
        )
    }

    #[test]
    fn test_security_control_from_u16() {
        let value: u16 = 0b0_0_0_0_1_110_11;

        let expected_security_control = SecurityControlRegister0Builder::default()
            .rx_fifo_protection(false)
            .sec_cbc_head(false)
            .sec_sa_key_sel(false)
            .sec_tx_key_sel(false)
            .sec_rx_key_sel(true)
            .sec_m(6)
            .sec_mode(3)
            .build()
            .unwrap();

        assert_eq!(
            expected_security_control,
            value.into(),
        )
    }

    #[test]
    fn test_sec_txl_value() {
        let security_control = SecurityControlRegister1Builder::default()
            .sec_txl(5)
            .build()
            .unwrap();

        assert_eq!(
            security_control.register_value(),
            0b0_0000101_0000_0000,
        )
    }

    #[test]
    fn test_sec_rxl_value() {
        let security_control = SecurityControlRegister1Builder::default()
            .sec_rxl(5)
            .build()
            .unwrap();

        assert_eq!(
            security_control.register_value(),
            0b0_0000000_0_0000101,
        )
    }

    #[test]
    fn test_security_control_1_from_u16() {
        let value: u16 = 0b0_1111111_0_1111111;

        let expected_security_control = SecurityControlRegister1Builder::default()
            .sec_txl(0x7F)
            .sec_rxl(0x7F)
            .build()
            .unwrap();

        assert_eq!(
            expected_security_control,
            value.into(),
        )
    }
}