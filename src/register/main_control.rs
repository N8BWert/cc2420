//!
//! The Main Control Register
//! 

use super::Register;
use derive_builder::Builder;

/// Main Control Register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(error(validation_error = false)))]
pub struct MainControlRegister {
    // Active low reset of the entire circuit should be applied
    // before doing anything else
    #[builder(default = "true")]
    pub reset_n: bool,
    // Active low reset of the encryption module (Testing Only)
    #[builder(default = "true")]
    pub enc_reset_n: bool,
    // Active low reset of the demodulator module (Testing Only)
    #[builder(default = "true")]
    pub demod_reset_n: bool,
    // Active low rest of the modulator module (Testing Only)
    #[builder(default = "true")]
    pub mod_reset_n: bool,
    // Active low reset of the frequency synthesizer module (Testing Only)
    #[builder(default = "true")]
    pub fs_reset_n: bool,
    // Bypass the crystal osciallator and use a buffered version of the signal on
    // Q1 directly.  This can be used to apply an external rail-rail clock
    // signal to the Q1 pin.
    #[builder(default = "false")]
    pub xosc16m_bypass: bool,
}

impl Register for MainControlRegister {
    fn register_value(&self) -> u16 {
        let mut value = 0b11111000_00000000;
        
        if !self.reset_n {
            value &= !(1 << 15);
        }

        if !self.enc_reset_n {
            value &= !(1 << 14);
        }

        if !self.demod_reset_n {
            value &= !(1 << 13);
        }

        if !self.mod_reset_n {
            value &= !(1 << 12);
        }

        if !self.fs_reset_n {
            value &= !(1 << 11);
        }

        if self.xosc16m_bypass {
            value |= 1;
        }
        
        value
    }

    fn address(&self) -> u8 { 0x10 }

    fn fill_from_buffer(&mut self, buffer: [u8; 3]) {
        *self = u16::from_le_bytes(buffer[1..3].try_into().unwrap()).into();
    }
}

impl From<u16> for MainControlRegister {
    fn from(value: u16) -> Self {
        Self {
            reset_n: (((1 << 15) & value) != 0),
            enc_reset_n: (((1 << 14) & value) != 0),
            demod_reset_n: (((1 << 13) & value) != 0),
            mod_reset_n: (((1 << 12) & value) != 0),
            fs_reset_n: (((1 << 11) & value) != 0),
            xosc16m_bypass: (1 & value) == 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_value_reset_n() {
        let main_control_register = MainControlRegisterBuilder::default()
            .reset_n(false)
            .build()
            .unwrap();

        assert_eq!(
            main_control_register.register_value(),
            0b01111000_00000000,
        )
    }

    #[test]
    fn test_register_value_enc_reset_n() {
        let main_control_register = MainControlRegisterBuilder::default()
            .enc_reset_n(false)
            .build()
            .unwrap();

        assert_eq!(
            main_control_register.register_value(),
            0b10111000_00000000,
        )
    }

    #[test]
    fn test_register_value_demod_reset_n() {
        let main_control_register = MainControlRegisterBuilder::default()
            .demod_reset_n(false)
            .build()
            .unwrap();

        assert_eq!(
            main_control_register.register_value(),
            0b11011000_00000000,
        )
    }

    #[test]
    fn test_register_value_mod_reset_n() {
        let main_control_register = MainControlRegisterBuilder::default()
            .mod_reset_n(false)
            .build()
            .unwrap();

        assert_eq!(
            main_control_register.register_value(),
            0b11101000_00000000,
        )
    }

    #[test]
    fn test_register_value_fs_reset_n() {
        let main_control_register = MainControlRegisterBuilder::default()
            .fs_reset_n(false)
            .build()
            .unwrap();

        assert_eq!(
            main_control_register.register_value(),
            0b11110000_00000000,
        )
    }

    #[test]
    fn test_register_value_xosxc16m_bypass() {
        let main_control_register = MainControlRegisterBuilder::default()
            .xosc16m_bypass(true)
            .build()
            .unwrap();

        assert_eq!(
            main_control_register.register_value(),
            0b11111000_00000001,
        )
    }

    #[test]
    fn test_main_control_from_u16() {
        let value: u16 = 0b00100000_00000001;

        let expected_register = MainControlRegisterBuilder::default()
            .reset_n(false)
            .enc_reset_n(false)
            .demod_reset_n(true)
            .mod_reset_n(false)
            .fs_reset_n(false)
            .xosc16m_bypass(true)
            .build()
            .unwrap();

        assert_eq!(
            expected_register,
            value.into(),
        )
    }
}