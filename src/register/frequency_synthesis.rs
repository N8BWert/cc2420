//!
//! Definition for the FSCTRL (Frequency Synthesizer Control and Status)
//! Register
//! 

use super::RegisterValue;

use alloc::string::String;

use derive_builder::Builder;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(validate = "Self::validate"))]
pub struct FrequencySynthesizerRegister {
    // Number of consecutive reference clock periods with successful
    // synchronisation windows required to indicate lock:
    // 0: 64
    // 1: 128 (recommended)
    // 2: 256
    // 3: 512
    #[builder(default = "1")]
    pub lock_threshold: u8,
    // Calibration has been performed since the last time the
    // frequency synthesizer was turned on.
    // READ-ONLY
    #[builder(default = "false")]
    pub cal_done: bool,
    // Calibration status, "1" when calibration in progress and "0"
    // otherwise.
    // READ-ONLY
    #[builder(default = "false")]
    pub cal_running: bool,
    // Synchronisation window pulse width:
    // 0: 2 prescaler clock periods (recommended)
    // 1: 4 prescaler clock periods
    #[builder(default = "false")]
    pub lock_length: bool,
    // Frequency synthesizer lock status:
    // 0: Frequency synthesizer is out of lock
    // 1: Frequency synthesizer is in lock
    // READ-ONLY
    #[builder(default = "false")]
    pub lock_status: bool,
    // Frequency control word, controlling the RF operating frequency.
    // Fc. In transmit mode, the local oscillator (LO) frequency equals
    // Fc. In receive mode, the LO frequency is 2 MHz below Fc.
    // Fc = 2048 + FREQ[9:0] MHz
    // See the Frequency and Channel Programming section on page
    // 50 for further information
    // default = 357 = 2405 MHz
    #[builder(default = "357")]
    pub frequency: u16,
}

impl RegisterValue for FrequencySynthesizerRegister {
    fn register_value(&self) -> u16 {
        let mut value = 0;

        value |= (self.lock_threshold as u16) << 14;

        if self.cal_done {
            value |= 1 << 13;
        }

        if self.cal_running {
            value |= 1 << 12;
        }

        if self.lock_length {
            value |= 1 << 11;
        }

        if self.lock_status {
            value |= 1 << 10;
        }

        value |= self.frequency as u16;

        value
    }
}

impl From<u16> for FrequencySynthesizerRegister {
    fn from(value: u16) -> Self {
        Self {
            lock_threshold: (((0b11 << 14) & value) >> 14) as u8,
            cal_done: (((1 << 13) & value) != 0),
            cal_running: (((1 << 12) & value) != 0),
            lock_length: (((1 << 11) & value) != 0),
            lock_status: (((1 << 10) & value) != 0),
            frequency: value & 0x1FF,
        }
    }
}

impl FrequencySynthesizerRegisterBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(lock_threshold) = self.lock_threshold {
            if lock_threshold > 3 {
                return Err("Invalid LOCK_THR. 0<=LOCK_THR<=3".into());
            }
        }

        if let Some(frequency) = self.frequency {
            if frequency >= (1 << 10) {
                return Err("Invalid FREQ. 0<=FREQ<=(1 << 10)".into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_threshold_value() {
        let frequency_synthesis = FrequencySynthesizerRegisterBuilder::default()
            .lock_threshold(2)
            .build()
            .unwrap();

        assert_eq!(
            frequency_synthesis.register_value(),
            0b10_0_0_0_0_0101100101,
        )
    }

    #[test]
    fn test_lock_length_value() {
        let frequency_synthesis = FrequencySynthesizerRegisterBuilder::default()
            .lock_length(true)
            .build()
            .unwrap();

        assert_eq!(
            frequency_synthesis.register_value(),
            0b01_0_0_1_0_0101100101,
        )
    }

    #[test]
    fn test_frequency_value() {
        let frequency_synthesis = FrequencySynthesizerRegisterBuilder::default()
            .frequency(32)
            .build()
            .unwrap();

        assert_eq!(
            frequency_synthesis.register_value(),
            0b01_0_0_0_0_0000100000,
        )
    }

    #[test]
    fn test_frequency_synthesis_from_u16() {
        let value = 0b10_1_1_1_1_0100001111;

        let expected_frequency_synthesis = FrequencySynthesizerRegisterBuilder::default()
            .lock_threshold(2)
            .cal_done(true)
            .cal_running(true)
            .lock_length(true)
            .lock_status(true)
            .frequency(271)
            .build()
            .unwrap();

        assert_eq!(
            expected_frequency_synthesis,
            value.into(),
        )
    }
}