//!
//! Rust Definition of the AGC Control Register
//! 

use super::RegisterValue;

use alloc::string::String;

use derive_builder::Builder;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(validate = "Self::validate"))]
pub struct AGCControlRegister {
    // Use the VGA_GAIN value during RX instead of the AGC value.
    #[builder(default = "false")]
    pub vga_gain_oe: bool,
    // When written, VGA manual gain override value; when read, the
    // currently used VGA gain setting.
    #[builder(default = "0x7F")]
    pub vga_gain: u8,
    // LNA / Mixer Gain mode override setting
    // 0: Gain mode is set by AGC algorithm
    // 1: Gain mode is always low-gain
    // 2: Gain mode is always med-gain
    // 3: Gain mode is always high-gain
    #[builder(default = "0")]
    pub lnamix_gainmode_o: u8,
    // Status bit, defining the currently selected gain mode selected by
    // the AGC or overriden by the LNAMIX_GAINMODE_0 setting.
    #[builder(default = "3")]
    pub lnamix_gainmode: u8,
}

impl RegisterValue for AGCControlRegister {
    fn register_value(&self) -> u16 {
        let mut value = 0;

        if self.vga_gain_oe {
            value |= 1 << 11;
        }

        value |= (self.vga_gain as u16) << 4;

        value |= (self.lnamix_gainmode_o as u16) << 2;

        value
    }
}

impl From<u16> for AGCControlRegister {
    fn from(value: u16) -> Self {
        Self {
            vga_gain_oe: (((1 << 11) & value) != 0),
            vga_gain: (((0x7F << 4) & value) >> 4) as u8,
            lnamix_gainmode_o: (((0b11 << 2) & value) >> 2) as u8,
            lnamix_gainmode: (0b11 & value) as u8,
        }
    }
}

impl AGCControlRegisterBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(vga_gain) = self.vga_gain {
            if vga_gain > 0x7F {
                return Err("Invalid VGA_GAIN. 0<=VGA_GAIN<=0x7F".into());
            }
        }

        if let Some(lnamix_gainmode_o) = self.lnamix_gainmode_o {
            if lnamix_gainmode_o > 3 {
                return Err("Invalid LNAMIX_GAINMODE_O. 0<=LNAMIX_GAINMODE_O<=3".into());
            }
        }

        if let Some(lnamix_gainmode) = self.lnamix_gainmode {
            if lnamix_gainmode > 3 {
                return Err("Invalid LNAMIX_GAINMODE. 0<=LNAMIX_GAINMODE<=3".into()).into();
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::register::agc;

    use super::*;

    #[test]
    fn test_vga_gain_oe_value() {
        let agc_control = AGCControlRegisterBuilder::default()
            .vga_gain_oe(true)
            .build()
            .unwrap();

        assert_eq!(
            agc_control.register_value(),
            0b1_1111111_00_00,
        )
    }

    #[test]
    fn test_vga_gain_value() {
        let agc_control = AGCControlRegisterBuilder::default()
            .vga_gain(0)
            .build()
            .unwrap();

        assert_eq!(
            agc_control.register_value(),
            0b0_0000000_00_00,
        )
    }

    #[test]
    fn test_lnamix_gainmode_o_value() {
        let agc_control = AGCControlRegisterBuilder::default()
            .lnamix_gainmode_o(1)
            .build()
            .unwrap();

        assert_eq!(
            agc_control.register_value(),
            0b0_1111111_01_00,
        )
    }

    #[test]
    fn test_agc_control_from_u16() {
        let value = 0b1_0000000_11_00;

        let expected_agc_control = AGCControlRegisterBuilder::default()
            .vga_gain_oe(true)
            .vga_gain(0)
            .lnamix_gainmode_o(3)
            .lnamix_gainmode(0)
            .build()
            .unwrap();

        assert_eq!(
            expected_agc_control,
            value.into(),
        )
    }
}