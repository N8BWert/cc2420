//!
//! Receive Control Register Definitions
//! 

use super::Register;

use alloc::string::String;

use derive_builder::Builder;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(validate = "Self::validate"))]
pub struct ReceiveControlRegister0 {
    // RX mixer buffer bias current.
    // 0: 690 uA
    // 1: 980 uA (nominal)
    // 2: 1.16 mA
    // 3: 1.44 mA
    #[builder(default = "1")]
    pub rx_mix_buf_current: u8,
    // Controls current in the LNA gain compensation branch in AGC
    // high gain model.
    // 0: Compensation disabled
    // 1: 100 uA compensation current
    // 2: 300 uA compensation current (Nominal)
    // 3: 1000 uA compensation current
    #[builder(default = "0")]
    pub high_lna_gain: u8,
    // Controls current in the LNA gain compensation branch in AGC
    // Med gain mode.
    #[builder(default = "2")]
    pub med_lna_gain: u8,
    // Controls current in the LNA gain compensation branch in AGC
    // Low gain mode.
    #[builder(default = "3")]
    pub low_lna_gain: u8,
    // Controls main current in the LNA in AGC High gain mode.
    // 0: 240 uA LNA current (x2)
    // 1: 480 uA LNA current (x2)
    // 2: 640 uA LNA current (x2)
    // 3: 1280 uA LNA current (x2)
    #[builder(default = "2")]
    pub high_lna_current: u8,
    // Controls main current in the LNA in AGC Med gain mode
    #[builder(default = "1")]
    pub med_lna_current: u8,
    // Controls main current in the LNA in AGC Low gain mode
    #[builder(default = "1")]
    pub low_lna_current: u8,
}

impl Register for ReceiveControlRegister0 {
    fn register_value(&self) -> u16 {
        let mut value = 0;

        value |= (self.rx_mix_buf_current as u16) << 12;

        value |= (self.high_lna_gain as u16) << 10;

        value |= (self.med_lna_gain as u16) << 8;

        value |= (self.low_lna_gain as u16) << 6;

        value |= (self.high_lna_current as u16) << 4;
        
        value |= (self.med_lna_current as u16) << 2;

        value |= self.low_lna_current as u16;

        value
    }

    fn address(&self) -> u8 { 0x16 }

    fn from_buffer(&mut self, buffer: [u8; 3]) {
        *self = u16::from_le_bytes(buffer[1..3].try_into().unwrap()).into();
    }
}

impl From<u16> for ReceiveControlRegister0 {
    fn from(value: u16) -> Self {
        Self {
            rx_mix_buf_current: (((0b11 << 12) & value) >> 12) as u8,
            high_lna_gain: (((0b11 << 10) & value) >> 10) as u8,
            med_lna_gain: (((0b11 << 8) & value) >> 8) as u8,
            low_lna_gain: (((0b11 << 6) & value) >> 6) as u8,
            high_lna_current: (((0b11 << 4) & value) >> 4) as u8,
            med_lna_current: (((0b11 << 2) & value) >> 2) as u8,
            low_lna_current: (0b11 & value) as u8,
        }
    }
}

impl ReceiveControlRegister0Builder {
    fn validate(&self) -> Result<(), String> {
        if let Some(rx_mix_buf_current) = self.rx_mix_buf_current {
            if rx_mix_buf_current > 3 {
                return Err("Invalid RXMIXBUF_COR. 0<=RXMIXBUF_CUR<=3".into());
            }
        }

        if let Some(high_lna_gain) = self.high_lna_gain {
            if high_lna_gain > 3 {
                return Err("Invalid HIGH_LNA_GAIN. 0<=HIGH_LNA_GAIN<=3".into());
            }
        }

        if let Some(med_lna_gain) = self.med_lna_gain {
            if med_lna_gain > 3 {
                return Err("Invalid MED_LNA_GAIN. 0<=MED_LNA_GAIN<=3".into());
            }
        }

        if let Some(low_lna_gain) = self.low_lna_gain {
            if low_lna_gain > 3 {
                return Err("Invalid LOW_LNA_GAIN. 0<=LOW_LNA_GAIN<=3".into());
            }
        }

        if let Some(high_lna_current) = self.high_lna_current {
            if high_lna_current > 3 {
                return Err("Invalid HIGH_LNA_CURRENT. 0<=HIGH_LNA_GAIN<=3".into());
            }
        }

        if let Some(med_lna_current) = self.med_lna_current {
            if med_lna_current > 3 {
                return Err("Invalid MED_LNA_CURRENT. 0<=MED_LNA_GAIN<=3".into());
            }
        }

        if let Some(low_lna_current) = self.low_lna_current {
            if low_lna_current > 3 {
                return Err("Invalid LOW_LNA_CURRENT. 0<=LOW_LNA_CURRENT<=3".into());
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(validate = "Self::validate"))]
pub struct ReceiveControlRegister1 {
    // Controls reference bias current to RX bandpass filters:
    // 0: 4 uA (Reset value) Use 1 instead
    // 1: 3uA Note: Recommended setting
    #[builder(default = "true")]
    pub rxbpf_locur: bool,
    // Controls reference bias current to RX bandpass filters:
    // 0: 4 uA (Default)
    // 1: 3.5 uA
    #[builder(default = "false")]
    pub rxbpf_midcur: bool,
    // LNA low gain mode setting in AGC low gain mode.
    #[builder(default = "true")]
    pub low_lowgain: bool,
    // LNA low gain mode setting in AGC medium gain mode.
    #[builder(default = "false")]
    pub med_lowgain: bool,
    // RX Mixers high gain mode setting in AGC high gain mode.
    #[builder(default = "true")]
    pub high_hgm: bool,
    // RX Mixers high gain mode setting in AGC medium gain mode.
    #[builder(default = "false")]
    pub med_hgm: bool,
    // Selects varactor array setting in the LNA
    // 0: OFF
    // 1: 0.1 pF (x2) (Nominal)
    // 2: 0.2 pF (x2)
    // 3: 0x3 pF (x2)
    #[builder(default = "1")]
    pub lna_cap_array: u8,
    // Control the receiver mixers output current.
    // 0: 12 uA
    // 1: 16 uA (Nominal)
    // 2: 20 uA
    // 3: 24 uA
    #[builder(default = "1")]
    pub rxmix_tail: u8,
    // Controls VCM level in the mixer feedback loop
    // 0: 8 uA mixer current
    // 1: 12 uA mixer current (Nominal)
    // 2: 16 uA mixer current
    // 3: 20 uA mixer current
    #[builder(default = "1")]
    pub rxmix_vcm: u8,
    // Controls Current in the mixer
    // 0: 360 uA mixer current (x2)
    // 1: 720 uA mixer current (x2)
    // 2: 900 uA mixer current (x2) (Nominal)
    // 3: 1260 uA mixer current (x2)
    #[builder(default = "2")]
    pub rxmix_current: u8,
}

impl Register for ReceiveControlRegister1 {
    fn register_value(&self) -> u16 {
        let mut value = 0;

        if self.rxbpf_locur {
            value |= 1 << 13;
        }

        if self.rxbpf_midcur {
            value |= 1 << 12;
        }

        if self.low_lowgain {
            value |= 1 << 11;
        }

        if self.med_lowgain {
            value |= 1 << 10;
        }

        if self.high_hgm {
            value |= 1 << 9;
        }

        if self.med_hgm {
            value |= 1 << 8;
        }

        value |= (self.lna_cap_array as u16) << 6;

        value |= (self.rxmix_tail as u16) << 4;

        value |= (self.rxmix_vcm as u16) << 2;

        value |= self.rxmix_current as u16;

        value
    }

    fn address(&self) -> u8 { 0x17 }

    fn from_buffer(&mut self, buffer: [u8; 3]) {
        *self = u16::from_le_bytes(buffer[1..3].try_into().unwrap()).into();
    }
}

impl From<u16> for ReceiveControlRegister1 {
    fn from(value: u16) -> Self {
        Self {
            rxbpf_locur: (((1 << 13) & value) != 0),
            rxbpf_midcur: (((1 << 12) & value) != 0),
            low_lowgain: (((1 << 11) & value) != 0),
            med_lowgain: (((1 << 10) & value) != 0),
            high_hgm: (((1 << 9) & value) != 0),
            med_hgm: (((1 << 10) & value) != 0),
            lna_cap_array: (((0b11 << 6) & value) >> 6) as u8,
            rxmix_tail: (((0b11 << 4) & value) >> 4) as u8,
            rxmix_vcm: (((0b11 << 2) & value) >> 2) as u8,
            rxmix_current: (0b11 & value) as u8,
        }
    }
}

impl ReceiveControlRegister1Builder {
    fn validate(&self) -> Result<(), String> {
        if let Some(lna_cap_array) = self.lna_cap_array {
            if lna_cap_array > 3 {
                return Err("Invalid LNA_CAP_ARRAY. 0<=LNA_CAP_ARRAY<=3".into());
            }
        }

        if let Some(rxmix_tail) = self.rxmix_tail {
            if rxmix_tail > 3 {
                return Err("Invalid RXMIX_TAIL. 0<=RXMIX_TAIL<=3".into());
            }
        }

        if let Some(rxmix_vcm) = self.rxmix_vcm {
            if rxmix_vcm > 3 {
                return Err("Invalid RXMIX_VCM. 0<=RXMIX_VCM<=3".into());
            }
        }

        if let Some(rxmix_current) = self.rxmix_current {
            if rxmix_current > 3 {
                return Err("Invalid RXMIX_CURRENT. 0<=RXMIX_CURRENT<=3".into());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rx_mix_buf_current_value() {
        let rx_control_0 = ReceiveControlRegister0Builder::default()
            .rx_mix_buf_current(3)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_0.register_value(),
            0b00_11_00_10_11_10_01_01,
        )
    }

    #[test]
    fn test_high_lna_gain_value() {
        let rx_control_0 = ReceiveControlRegister0Builder::default()
            .high_lna_gain(3)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_0.register_value(),
            0b00_01_11_10_11_10_01_01,
        )
    }

    #[test]
    fn test_med_lna_gain_value() {
        let rx_control_0 = ReceiveControlRegister0Builder::default()
            .med_lna_gain(0)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_0.register_value(),
            0b00_01_00_00_11_10_01_01,
        )
    }

    #[test]
    fn test_low_lna_gain_value() {
        let rx_control_0 = ReceiveControlRegister0Builder::default()
            .low_lna_gain(0)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_0.register_value(),
            0b00_01_00_10_00_10_01_01,
        )
    }

    #[test]
    fn test_high_lna_current_value() {
        let rx_control_0 = ReceiveControlRegister0Builder::default()
            .high_lna_current(0)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_0.register_value(),
            0b00_01_00_10_11_00_01_01,
        )
    }

    #[test]
    fn test_med_lna_current_value() {
        let rx_control_0 = ReceiveControlRegister0Builder::default()
            .med_lna_current(2)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_0.register_value(),
            0b00_01_00_10_11_10_10_01,
        )
    }

    #[test]
    fn test_low_ln_current_value() {
        let rx_control_0 = ReceiveControlRegister0Builder::default()
            .low_lna_current(2)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_0.register_value(),
            0b00_01_00_10_11_10_01_10,
        )
    }

    #[test]
    fn test_rx_control_0_from_u16() {
        let value = 0b00_10_11_00_01_01_10_10;

        let expected_rx_control = ReceiveControlRegister0Builder::default()
            .rx_mix_buf_current(2)
            .high_lna_gain(3)
            .med_lna_gain(0)
            .low_lna_gain(1)
            .high_lna_current(1)
            .med_lna_current(2)
            .low_lna_current(2)
            .build()
            .unwrap();

        assert_eq!(
            expected_rx_control,
            value.into()
        )
    }

    #[test]
    fn test_rxbpf_locur_value() {
        let rx_control_1 = ReceiveControlRegister1Builder::default()
            .rxbpf_locur(false)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_1.register_value(),
            0b00_0_0_1_0_1_0_01_01_01_10,
        )
    }

    #[test]
    fn test_rxbpf_midcur_value() {
        let rx_control_1 = ReceiveControlRegister1Builder::default()
            .rxbpf_midcur(true)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_1.register_value(),
            0b00_1_1_1_0_1_0_01_01_01_10,
        )
    }

    #[test]
    fn test_low_lowgain_value() {
        let rx_control_1 = ReceiveControlRegister1Builder::default()
            .low_lowgain(false)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_1.register_value(),
            0b00_1_0_0_0_1_0_01_01_01_10,
        )
    }

    #[test]
    fn test_med_lowgain_value() {
        let rx_control_1 = ReceiveControlRegister1Builder::default()
            .med_lowgain(true)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_1.register_value(),
            0b00_1_0_1_1_1_0_01_01_01_10,
        )
    }

    #[test]
    fn test_high_hgm_value() {
        let rx_control_1 = ReceiveControlRegister1Builder::default()
            .high_hgm(false)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_1.register_value(),
            0b00_1_0_1_0_0_0_01_01_01_10,
        )
    }

    #[test]
    fn test_med_hgm_value() {
        let rx_control_1 = ReceiveControlRegister1Builder::default()
            .med_hgm(true)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_1.register_value(),
            0b00_1_0_1_0_1_1_01_01_01_10,
        )
    }

    #[test]
    fn test_lna_cap_array_value() {
        let rx_control_1 = ReceiveControlRegister1Builder::default()
            .lna_cap_array(2)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_1.register_value(),
            0b00_1_0_1_0_1_0_10_01_01_10,
        )
    }

    #[test]
    fn test_rxmix_tail_value() {
        let rx_control_1 = ReceiveControlRegister1Builder::default()
            .rxmix_tail(2)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_1.register_value(),
            0b00_1_0_1_0_1_0_01_10_01_10,
        )
    }

    #[test]
    fn test_rxmix_vcm_value() {
        let rx_control_1 = ReceiveControlRegister1Builder::default()
            .rxmix_vcm(2)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_1.register_value(),
            0b00_1_0_1_0_1_0_01_01_10_10,
        )
    }

    #[test]
    fn test_rxmix_current_value() {
        let rx_control_1 = ReceiveControlRegister1Builder::default()
            .rxmix_current(1)
            .build()
            .unwrap();

        assert_eq!(
            rx_control_1.register_value(),
            0b00_1_0_1_0_1_0_01_01_01_01,
        )
    }

    #[test]
    fn test_rx_control_1_from_u16() {
        let value: u16 = 0b00_110101_10_10_10_01;

        let expected_rx_control = ReceiveControlRegister1Builder::default()
            .rxbpf_locur(true)
            .rxbpf_midcur(true)
            .low_lowgain(false)
            .med_lowgain(true)
            .high_hgm(false)
            .med_hgm(true)
            .lna_cap_array(2)
            .rxmix_tail(2)
            .rxmix_vcm(2)
            .rxmix_current(1)
            .build()
            .unwrap();

        assert_eq!(
            expected_rx_control,
            value.into()
        )
    }
}