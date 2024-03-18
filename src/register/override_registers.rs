//!
//! Rust Definition of the Override Registers.
//! 

use super::RegisterValue;

use derive_builder::Builder;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(error(validation_error = false)))]
pub struct AndOverrideRegister {
    // The VGA_RESET_N signal is used to reset the peak detectors
    // in the VGA in the RX chain.
    #[builder(default = "true")]
    vga_reset_n: bool,
    // Global bias power down.
    #[builder(default = "true")]
    bias_pd: bool,
    // The BALLUN_CTRL signal controls whether the PA should
    // receive its required external biasing (1) or not (0) by
    // controlling the RX/tX output switch.
    #[builder(default = "true")]
    balun_ctrl: bool,
    // RXTX signal: controls whether the LO buffers (0) or the PA buffers
    // (1) should be used
    #[builder(default = "true")]
    rxtx: bool,
    // Powerdown of prescaler
    #[builder(default = "true")]
    pre_pd: bool,
    // Powerdown of PA (negative path)
    #[builder(default = "true")]
    pa_n_pd: bool,
    // Powerdown of PA (positive path). When PA_N_PD=1 and
    // PA_P_PD=1 the up-conversion mixers are in powerdown
    #[builder(default = "true")]
    pa_p_pd: bool,
    // Powerdown of TX DACs
    #[builder(default = "true")]
    dac_lpf_pd: bool,
    #[builder(default = "true")]
    xosc16m_pd: bool,
    // Powerdown control of complex bandpass receive filter
    // calibration oscillator.
    #[builder(default = "true")]
    rxbpf_cal_pd: bool,
    // Powerdown control of charge pump
    #[builder(default = "true")]
    chp_pd: bool,
    // Powerdown control of VCO, I/Q generator, LO buffers
    #[builder(default = "true")]
    fs_pd: bool,
    // Powerdown control of the ADCs.
    #[builder(default = "true")]
    adc_pd: bool,
    // Powerdown control of the VGA
    #[builder(default = "true")]
    vga_pd: bool,
    // Powerdown control of complex bandpass receive filter.
    #[builder(default = "true")]
    rxbpf_pd: bool,
    // Powerdown control of LNA, down-conversion mixers and front
    // end bias.
    #[builder(default = "true")]
    lnamix_pd: bool,
}

impl RegisterValue for AndOverrideRegister {
    fn register_value(&self) -> u16 {
        let mut value = 0;

        if self.vga_reset_n {
            value |= 1 << 15;
        }

        if self.bias_pd {
            value |= 1 << 14;
        }

        if self.balun_ctrl {
            value |= 1 << 13;
        }

        if self.rxtx {
            value |= 1 << 12;
        }

        if self.pre_pd {
            value |= 1 << 11;
        }

        if self.pa_n_pd {
            value |= 1 << 10;
        }

        if self.pa_p_pd {
            value |= 1 << 9;
        }

        if self.dac_lpf_pd {
            value |= 1 << 8;
        }

        if self.xosc16m_pd {
            value |= 1 << 7;
        }

        if self.rxbpf_cal_pd {
            value |= 1 << 6;
        }

        if self.chp_pd {
            value |= 1 << 5;
        }

        if self.fs_pd {
            value |= 1 << 4;
        }

        if self.adc_pd {
            value |= 1 << 3;
        }

        if self.vga_pd {
            value |= 1 << 2;
        }

        if self.rxbpf_pd {
            value |= 1 << 1;
        }

        if self.lnamix_pd {
            value |= 1;
        }

        value
    }
}

impl From<u16> for AndOverrideRegister {
    fn from(value: u16) -> Self {
        Self {
            vga_reset_n: (((1 << 15) & value) != 0),
            bias_pd: (((1 << 14) & value) != 0),
            balun_ctrl: (((1 << 13) & value) != 0),
            rxtx: (((1 << 12) & value) != 0),
            pre_pd: (((1 << 11) & value) != 0),
            pa_n_pd: (((1 << 10) & value) != 0),
            pa_p_pd: (((1 << 9) & value) != 0),
            dac_lpf_pd: (((1 << 8) & value) != 0),
            xosc16m_pd: (((1 << 7) & value) != 0),
            rxbpf_cal_pd: (((1 << 6) & value) != 0),
            chp_pd: (((1 << 5) & value) != 0),
            fs_pd: (((1 << 4) & value) != 0),
            adc_pd: (((1 << 3) & value) != 0),
            vga_pd: (((1 << 2) & value) != 0),
            rxbpf_pd: (((1 << 1) & value) != 0),
            lnamix_pd: (1 & value) != 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(error(validation_error = false)))]
pub struct OrOverrideRegister {
    // The VGA_RESET_N signal is used to reset the peak detectors
    // in the VGA in the RX chain.
    #[builder(default = "false")]
    vga_reset_n: bool,
    // Global bias power down.
    #[builder(default = "false")]
    bias_pd: bool,
    // The BALLUN_CTRL signal controls whether the PA should
    // receive its required external biasing (1) or not (0) by
    // controlling the RX/tX output switch.
    #[builder(default = "false")]
    balun_ctrl: bool,
    // RXTX signal: controls whether the LO buffers (0) or the PA buffers
    // (1) should be used
    #[builder(default = "false")]
    rxtx: bool,
    // Powerdown of prescaler
    #[builder(default = "false")]
    pre_pd: bool,
    // Powerdown of PA (negative path)
    #[builder(default = "false")]
    pa_n_pd: bool,
    // Powerdown of PA (positive path). When PA_N_PD=1 and
    // PA_P_PD=1 the up-conversion mixers are in powerdown
    #[builder(default = "false")]
    pa_p_pd: bool,
    // Powerdown of TX DACs
    #[builder(default = "false")]
    dac_lpf_pd: bool,
    #[builder(default = "false")]
    xosc16m_pd: bool,
    // Powerdown control of complex bandpass receive filter
    // calibration oscillator.
    #[builder(default = "false")]
    rxbpf_cal_pd: bool,
    // Powerdown control of charge pump
    #[builder(default = "false")]
    chp_pd: bool,
    // Powerdown control of VCO, I/Q generator, LO buffers
    #[builder(default = "false")]
    fs_pd: bool,
    // Powerdown control of the ADCs.
    #[builder(default = "false")]
    adc_pd: bool,
    // Powerdown control of the VGA
    #[builder(default = "false")]
    vga_pd: bool,
    // Powerdown control of complex bandpass receive filter.
    #[builder(default = "false")]
    rxbpf_pd: bool,
    // Powerdown control of LNA, down-conversion mixers and front
    // end bias.
    #[builder(default = "false")]
    lnamix_pd: bool,
}

impl RegisterValue for OrOverrideRegister {
    fn register_value(&self) -> u16 {
        let mut value = 0;

        if self.vga_reset_n {
            value |= 1 << 15;
        }

        if self.bias_pd {
            value |= 1 << 14;
        }

        if self.balun_ctrl {
            value |= 1 << 13;
        }

        if self.rxtx {
            value |= 1 << 12;
        }

        if self.pre_pd {
            value |= 1 << 11;
        }

        if self.pa_n_pd {
            value |= 1 << 10;
        }

        if self.pa_p_pd {
            value |= 1 << 9;
        }

        if self.dac_lpf_pd {
            value |= 1 << 8;
        }

        if self.xosc16m_pd {
            value |= 1 << 7;
        }

        if self.rxbpf_cal_pd {
            value |= 1 << 6;
        }

        if self.chp_pd {
            value |= 1 << 5;
        }

        if self.fs_pd {
            value |= 1 << 4;
        }

        if self.adc_pd {
            value |= 1 << 3;
        }

        if self.vga_pd {
            value |= 1 << 2;
        }

        if self.rxbpf_pd {
            value |= 1 << 1;
        }

        if self.lnamix_pd {
            value |= 1;
        }

        value
    }
}

impl From<u16> for OrOverrideRegister {
    fn from(value: u16) -> Self {
        Self {
            vga_reset_n: (((1 << 15) & value) != 0),
            bias_pd: (((1 << 14) & value) != 0),
            balun_ctrl: (((1 << 13) & value) != 0),
            rxtx: (((1 << 12) & value) != 0),
            pre_pd: (((1 << 11) & value) != 0),
            pa_n_pd: (((1 << 10) & value) != 0),
            pa_p_pd: (((1 << 9) & value) != 0),
            dac_lpf_pd: (((1 << 8) & value) != 0),
            xosc16m_pd: (((1 << 7) & value) != 0),
            rxbpf_cal_pd: (((1 << 6) & value) != 0),
            chp_pd: (((1 << 5) & value) != 0),
            fs_pd: (((1 << 4) & value) != 0),
            adc_pd: (((1 << 3) & value) != 0),
            vga_pd: (((1 << 2) & value) != 0),
            rxbpf_pd: (((1 << 1) & value) != 0),
            lnamix_pd: (1 & value) != 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO:
}