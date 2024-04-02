//!
//! Register Definition for the Transmit Control Register
//! 

use super::Register;

use alloc::string::String;

use derive_builder::Builder;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(validate = "Self::validate"))]
pub struct TransmitControlRegister {
    // TX Mixer buffer bias current.
    // 0: 690 uA
    // 1: 980 uA
    // 2: 1.16 mA (nominal)
    // 3: 1.44 mA
    #[builder(default = "2")]
    pub tx_mix_buffer_current: u8,
    // Sets the wait time after STXON before transmission is started.
    // 0: 8 symbol periods (128 us)
    // 1: 12 symbol periods (192 us)
    #[builder(default = "true")]
    pub tx_turnaround: bool,
    // Selects veractor array settings in the transmit mixers
    #[builder(default = "0")]
    pub tx_mix_cap_array: u8,
    // Transmit mixers current:
    // 0: 1.72 mA
    // 1: 1.88 mA
    // 2: 2.05 mA
    // 3: 2.21 mA
    #[builder(default = "0")]
    pub tx_mix_current: u8,
    // Current Programming of the PA
    // 0: -3 current adjustment
    // 1: -2 current adjustment
    // 2: -1 current adjustment
    // 3: Nominal setting
    // 4: +1 current adjustment
    // 5: +2 current adjustment
    // 6: +3 current adjustment
    // 7: +4 current adjustment
    #[builder(default = "3")]
    pub pa_current: u8,
    // Output PA Level. (~0 dBm)
    #[builder(default = "31")]
    pub pa_level: u8,
}

impl Register for TransmitControlRegister {
    fn register_value(&self) -> u16 {
        let mut value = 1 << 5;

        value |= (self.tx_mix_buffer_current as u16) << 14;

        if self.tx_turnaround {
            value |= 1 << 13;
        }

        value |= (self.tx_mix_cap_array as u16) << 11;

        value |= (self.tx_mix_current as u16) << 9;

        value |= (self.pa_current as u16) << 6;

        value |= self.pa_level as u16;

        value
    }

    fn address(&self) -> u8 { 0x15 }

    fn fill_from_buffer(&mut self, buffer: [u8; 3]) {
        *self = u16::from_le_bytes(buffer[1..3].try_into().unwrap()).into();
    }
}

impl From<u16> for TransmitControlRegister {
    fn from(value: u16) -> Self {
        Self {
            tx_mix_buffer_current: (((0b11 << 14) & value) >> 14) as u8,
            tx_turnaround: (((1 << 13) & value) != 0),
            tx_mix_cap_array: (((0b11 << 11) & value) >> 11) as u8,
            tx_mix_current: (((0b11 << 9) & value) >> 9) as u8,
            pa_current: (((0b111 << 6) & value) >> 6) as u8,
            pa_level: (value & 0b11111) as u8,
        }
    }
}

impl TransmitControlRegisterBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(tx_mix_buf_current) = self.tx_mix_buffer_current {
            if tx_mix_buf_current > 3 {
                return Err("Invalid TXMIXBUF_CUR. Expected 0<=TXMIXBUF_CUR<=3".into());
            }
        }

        if let Some(tx_mix_cap_array) = self.tx_mix_cap_array {
            if tx_mix_cap_array > 3 {
                return Err("Invalid TXMIX_CAP_ARRAY. Expected 0<=TXMIX_CAP_ARRAY<=3".into());
            }
        }

        if let Some(tx_mix_current) = self.tx_mix_current {
            if tx_mix_current > 3 {
                return Err("Invalid TXMIX_CURRENT. Expected 0<=TXMIX_CURRENT<=3".into());
            }
        }

        if let Some(pa_current) = self.pa_current {
            if pa_current > 7 {
                return Err("Invalid PA_CURRENT. Expected 0<=PA_CURRENT<=7".into());
            }
        }

        if let Some(pa_level) = self.pa_level {
            if pa_level > 31 {
                return Err("Invalid PA_LEVEL. Expected 0<=PA_LEVEL<=31".into());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tx_mix_buffer_current_value() {
        let transmit_control = TransmitControlRegisterBuilder::default()
            .tx_mix_buffer_current(3)
            .build()
            .unwrap();

        assert_eq!(
            transmit_control.register_value(),
            0b11_1_00_00_011_1_11111,
        )
    }

    #[test]
    fn test_tx_turnaround_value() {
        let transmit_control = TransmitControlRegisterBuilder::default()
            .tx_turnaround(false)
            .build()
            .unwrap();

        assert_eq!(
            transmit_control.register_value(),
            0b10_0_00_00_011_1_11111,
        )
    }

    #[test]
    fn test_tx_mix_cap_array_value() {
        let transmit_control = TransmitControlRegisterBuilder::default()
            .tx_mix_cap_array(2)
            .build()
            .unwrap();

        assert_eq!(
            transmit_control.register_value(),
            0b10_1_10_00_011_1_11111,
        )
    }

    #[test]
    fn test_tx_mix_current_value() {
        let transmit_control = TransmitControlRegisterBuilder::default()
            .tx_mix_current(1)
            .build()
            .unwrap();

        assert_eq!(
            transmit_control.register_value(),
            0b10_1_00_01_011_1_11111,
        )
    }

    #[test]
    fn test_pa_current_value() {
        let transmit_control = TransmitControlRegisterBuilder::default()
            .pa_current(4)
            .build()
            .unwrap();

        assert_eq!(
            transmit_control.register_value(),
            0b10_1_00_00_100_1_11111,
        )
    }

    #[test]
    fn test_pa_level_value() {
        let transmit_control = TransmitControlRegisterBuilder::default()
            .pa_level(7)
            .build()
            .unwrap();

        assert_eq!(
            transmit_control.register_value(),
            0b10_1_00_00_011_1_00111,
        )
    }

    #[test]
    fn test_transmit_control_from_u16() {
        let value: u16 = 0b01_0_11_01_100_1_00111;

        let expected_transmit_control = TransmitControlRegisterBuilder::default()
            .tx_mix_buffer_current(1)
            .tx_turnaround(false)
            .tx_mix_cap_array(3)
            .tx_mix_current(1)
            .pa_current(4)
            .pa_level(7)
            .build()
            .unwrap();

        assert_eq!(
            expected_transmit_control,
            value.into()
        )
    }
}