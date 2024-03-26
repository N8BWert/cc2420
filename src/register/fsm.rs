//!
//! Rust Register Definition for the Finite State Machine Time
//! Constants Register.
//! 

use super::Register;

use alloc::string::String;

use derive_builder::Builder;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(validate = "Self::validate"))]
pub struct FiniteStateMachineConstants {
    // The time in 5 us steps between the time the RX chain is enabled
    // and the demodulator and AGC is enabled. The RX chain is
    // started when bandpass filter has been calibrated (after 6.5
    // symbol periods.)
    #[builder(default = "3")]
    pub tc_rxchain_to_rx: u8,
    // The time in advance the RXTX switch is set high, before
    // enabling TX. In us.
    #[builder(default = "6")]
    pub tc_switch_to_tx: u8,
    // The time in advance the PA is powered up before enabling TX.
    // in us.
    #[builder(default = "10")]
    pub tc_paon_to_tx: u8,
    // The time after the last chip in the packet is sent, and the TXRX
    // switch is disabled. In us.
    #[builder(default = "2")]
    pub tc_txend_to_switch: u8,
    // The time after the last chip in the packet is sent, and the PA is
    // set in power-down. Also the time at which the modulator is
    // disabled. In us.
    #[builder(default = "4")]
    pub tc_txend_to_paoff: u8,
}

impl Register for FiniteStateMachineConstants {
    fn register_value(&self) -> u16 {
        let mut value = 0;

        value |= (self.tc_rxchain_to_rx as u16) << 13;

        value |= (self.tc_switch_to_tx as u16) << 10;

        value |= (self.tc_paon_to_tx as u16) << 6;

        value |= (self.tc_txend_to_switch as u16) << 3;

        value |= self.tc_txend_to_paoff as u16;

        value
    }

    fn address(&self) -> u8 { 0x20 }
    
    fn from_buffer(&mut self, buffer: [u8; 3]) {
        *self = u16::from_le_bytes(buffer[1..3].try_into().unwrap()).into();
    }
}

impl From<u16> for FiniteStateMachineConstants {
    fn from(value: u16) -> Self {
        Self {
            tc_rxchain_to_rx: (((0b111 << 13) & value) >> 13) as u8,
            tc_switch_to_tx: (((0b111 << 10) & value) >> 10) as u8,
            tc_paon_to_tx: (((0b1111 << 6) & value) >> 6) as u8,
            tc_txend_to_switch: (((0b111 << 3) & value) >> 3) as u8,
            tc_txend_to_paoff: (0b111 & value) as u8,
        }
    }
}

impl FiniteStateMachineConstantsBuilder {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tc_rxchain_to_rx_value() {
        let fsm_constants = FiniteStateMachineConstantsBuilder::default()
            .tc_rxchain_to_rx(5)
            .build()
            .unwrap();

        assert_eq!(
            fsm_constants.register_value(),
            0b101_110_1010_010_100,
        )
    }

    #[test]
    fn test_tc_switch_to_tx() {
        let fsm_constants = FiniteStateMachineConstantsBuilder::default()
            .tc_switch_to_tx(5)
            .build()
            .unwrap();

        assert_eq!(
            fsm_constants.register_value(),
            0b011_101_1010_010_100,
        )
    }

    #[test]
    fn test_tc_paon_to_tx() {
        let fsm_constants = FiniteStateMachineConstantsBuilder::default()
            .tc_paon_to_tx(5)
            .build()
            .unwrap();

        assert_eq!(
            fsm_constants.register_value(),
            0b011_110_0101_010_100,
        )
    }

    #[test]
    fn test_tc_txend_to_switch() {
        let fsm_constants = FiniteStateMachineConstantsBuilder::default()
            .tc_txend_to_switch(5)
            .build()
            .unwrap();

        assert_eq!(
            fsm_constants.register_value(),
            0b011_110_1010_101_100,
        )
    }

    #[test]
    fn test_tc_txend_to_paoff() {
        let fsm_constants = FiniteStateMachineConstantsBuilder::default()
            .tc_txend_to_paoff(5)
            .build()
            .unwrap();

        assert_eq!(
            fsm_constants.register_value(),
            0b011_110_1010_010_101,
        )
    }

    #[test]
    fn test_fsm_constants_from_u16() {
        let value = 0b101_101_0101_101_101;

        let expected_fsm_constants = FiniteStateMachineConstantsBuilder::default()
            .tc_rxchain_to_rx(5)
            .tc_switch_to_tx(5)
            .tc_paon_to_tx(5)
            .tc_txend_to_switch(5)
            .tc_txend_to_paoff(5)
            .build()
            .unwrap();

        assert_eq!(
            expected_fsm_constants,
            value.into()
        )
    }
}