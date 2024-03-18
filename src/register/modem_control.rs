//!
//! The Modem Control Registers
//! 

use super::RegisterValue;

use alloc::string::String;

use derive_builder::Builder;

/// Modem Control Register Definition
#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(validate = "Self::validate"))]
pub struct ModemControlRegister0 {
    // Mode for accepting reserved IEEE 802.15.4 frame types when
    // address recognition is enabled (MDMCTRL0.ADR_DECODe = 1)
    // 0: Reserved frame type (100, 101, 110, 111) are rejected
    // 1: Reserved frame types (100, 101, 110, 111) are always accepted
    // When address recognition is disabled, all frames are received
    #[builder(default = "false")]
    pub reserved_frame_mode: bool,
    // Should be set high when the device is a PAN Coordinator.  Used for
    // filtering packets with no destination address, as specified in section
    // 7.5.6.2 in 802.15.4, D18
    #[builder(default = "false")]
    pub pan_coordinator: bool,
    // Hardware Address decode enabled (0: Disabled, 1: Enabled)
    #[builder(default = "true")]
    pub adr_decode: bool,
    // CCA Hysteresis in dB, values 0, through 7dB
    #[builder(default = "2")]
    pub cca_hyst: u8,
    // 0: Reserved
    // 1: CCA=1 when RSSI_VAL < CCA_THR - CCA_HYST
    //    CCA=0 when RSSI_VAL >= CCA_THR
    // 2: CCA=1 when not receiving valid IEEE 802.15.4 data,
    //    CCA=0 otherwise
    // 3: CCA=1 when RSSI_VAL < CCA_THR - CCA_HYST and not receiving
    //          valid IEEE 802.15.4 data.
    //    CCA=0 when RSSI_VAL >= CCA_THR or receiving a packet
    #[builder(default = "3")]
    pub cca_mode: u8,
    // In packet mode a CRC-16 (ITU-T) is calculated and is transmitted
    // after the last data byte in TX. In RX CRC is calculated and checked
    // for validity.
    #[builder(default = "true")]
    pub auto_crc: bool,
    // If AUTOACK is set, all packets accepted by address recognition with the
    // acknowledge request flag set and a valid CFC are acknowledged 
    // 12 symbol periods after being received.
    #[builder(default = "false")]
    pub auto_ack: bool,
    // The number of preamble bytes (2 zero-symbols) to be sent in TX mode
    // prior to the SYNCWORD, encoded in steps of 2.  The reset value of 2 is
    // compliant with IEEE 802.15.4, since the 4th zero byte is included in the
    // SYNCWORD.
    // 0: 1 leading zero bytes
    // ...
    // 15: 16 leading zero bytes
    #[builder(default = "2")]
    pub preamble_length: u8,
}

impl RegisterValue for ModemControlRegister0 {
    fn register_value(&self) -> u16 {
        let mut value = 0;

        if self.reserved_frame_mode {
            value |= 1 << 13;
        }

        if self.pan_coordinator {
            value |= 1 << 12;
        }

        if self.adr_decode {
            value |= 1 << 11;
        }

        value |= (self.cca_hyst as u16) << 8;

        value |= (self.cca_mode as u16) << 6;

        if self.auto_crc {
            value |= 1 << 5;
        }

        if self.auto_ack {
            value |= 1 << 4;
        }

        value |= self.preamble_length as u16;

        value
    }
}

impl From<u16> for ModemControlRegister0 {
    fn from(value: u16) -> Self {
        Self {
            reserved_frame_mode: (((1 << 13) & value) != 0),
            pan_coordinator: (((1 << 12) & value) != 0),
            adr_decode: (((1 << 11) & value) != 0),
            cca_hyst: (((0b111 << 8) & value) >> 8) as u8,
            cca_mode: (((0b11 << 6) & value) >> 6) as u8,
            auto_crc: (((1 << 5) & value) != 0),
            auto_ack: (((1 << 4) & value) != 0),
            preamble_length: (value & 0b1111) as u8,
        }
    }
}

impl ModemControlRegister0Builder {
    fn validate(&self) -> Result<(), String> {
        if let Some(cca_hyst) = self.cca_hyst {
            if cca_hyst > 7 {
                return Err("Invalid CCA Hyst, Expected 0<=CCA_HYST<=7".into());
            }
        }

        if let Some(cca_mode) = self.cca_mode {
            if cca_mode == 0 || cca_mode > 3 {
                return Err("Invalid CCA Mode, Expected 1<=CCA_MODE<=3".into());
            }
        }

        if let Some(preamble_length) = self.preamble_length {
            if preamble_length > 15 {
                return Err("Invalid Preamble Length, Expected 0<=PREAMBLE_LENGTH<=15".into());
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(validate = "Self::validate"))]
pub struct ModemControlRegister1 {
    // Demodulator correlator threshold value, required before SFD search.
    #[builder(default = "20")]
    corr_threshold: u8,
    // Frequency Offset Average Filter Behavior
    // 0: Lock frequency offset filter after preamble match
    // 1: Continuously update frequency offset filter.
    #[builder(default = "false")]
    demod_average_mode: bool,
    // Set one of two RF modulation modes for RX / TX
    // 0: IEEE 802.15.4 compliant mode
    // 1: Reversed phase, non-IEEE compliant (could be used to set
    // up a system which will no receive 802.15.4 packets)
    #[builder(default = "false")]
    modulation_mode: bool,
    // Set test modes for TX
    // 0: Buffered mode, use TXFIFO (normal operation)
    // 1: Serial mode, use transmit data on serial interface, infinite
    // transmission. For lab testing only
    // 2: TXFIFO looping ignore underflow in TXFIFO and read cyclic infinite
    // transmission.  For lab testing only
    // 3: Send random data from CFC, infinite transmission.  For lab testing
    // only.
    #[builder(default = "0")]
    tx_mode: u8,
    // Set test mode of RX
    // 0: Buffered mode, use RXFIFO (normal operation)
    // 1: Receive serial mode, output received data on pins. Infinite
    // RX. For lab testing only.
    // 2 RXFIFO looping ignore overflow in RXFIFO and write cyclic, infinite
    // reception. For lab testing only
    // 3: Reserved
    #[builder(default = "0")]
    rx_mode: u8,
}

impl RegisterValue for ModemControlRegister1 {
    fn register_value(&self) -> u16 {
        let mut value = 0;

        value |= (self.corr_threshold as u16) << 6;

        if self.demod_average_mode {
            value |= 1 << 5;
        }

        if self.modulation_mode {
            value |= 1 << 4;
        }

        value |= (self.tx_mode as u16) << 2;

        value |= self.rx_mode as u16;

        value
    }
}

impl From<u16> for ModemControlRegister1 {
    fn from(value: u16) -> Self {
        Self {
            corr_threshold: (((0b11111 << 6) & value) >> 6) as u8,
            demod_average_mode: (((1 << 5) & value) != 0),
            modulation_mode: (((1 << 4) & value) != 0),
            tx_mode: (((0b11 << 2) & value) >> 2) as u8,
            rx_mode: (0b11 & value) as u8,
        }
    }
}

impl ModemControlRegister1Builder {
    fn validate(&self) -> Result<(), String> {
        if let Some(corr_threshold) = self.corr_threshold {
            if corr_threshold > 31 {
                return Err("Invalid Correlation Threshold, Expected 0<=CORR_THR<=31".into());
            }
        }

        if let Some(tx_mode) = self.tx_mode {
            if tx_mode > 3 {
                return Err("Invalid Tx Mode, Expected 0<=TX_MODE<=3".into());
            }
        }

        if let Some(rx_mode) = self.rx_mode {
            if rx_mode > 2 {
                return Err("Invalid Rx Mode, Expected 0<=RX_MODE<=2".into());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reserved_frame_mode_value() {
        let modem_control_0 = ModemControlRegister0Builder::default()
            .reserved_frame_mode(true)
            .build()
            .unwrap();

        assert_eq!(
            modem_control_0.register_value(),
            0b00_1_0_1_010_11_1_0_0010,
        )
    }

    #[test]
    fn test_pan_coordinator_value() {
        let modem_control_0 = ModemControlRegister0Builder::default()
            .pan_coordinator(true)
            .build()
            .unwrap();

        assert_eq!(
            modem_control_0.register_value(),
            0b00_0_1_1_010_11_1_0_0010,
        )
    }

    #[test]
    fn test_adr_decode_value() {
        let modem_control_0 = ModemControlRegister0Builder::default()
            .adr_decode(false)
            .build()
            .unwrap();

        assert_eq!(
            modem_control_0.register_value(),
            0b00_0_0_0_010_11_1_0_0010,
        )
    }

    #[test]
    fn test_cca_hyst_value() {
        let modem_control_0 = ModemControlRegister0Builder::default()
            .cca_hyst(3)
            .build()
            .unwrap();

        assert_eq!(
            modem_control_0.register_value(),
            0b00_0_0_1_011_11_1_0_0010,
        )
    }

    #[test]
    fn test_cca_mode_value() {
        let modem_control_0 = ModemControlRegister0Builder::default()
            .cca_mode(1)
            .build()
            .unwrap();

        assert_eq!(
            modem_control_0.register_value(),
            0b00_0_0_1_010_01_1_0_0010,
        )
    }

    #[test]
    fn test_auto_crc_value() {
        let modem_control_0 = ModemControlRegister0Builder::default()
            .auto_crc(false)
            .build()
            .unwrap();

        assert_eq!(
            modem_control_0.register_value(),
            0b00_0_0_1_010_11_0_0_0010,
        )
    }

    #[test]
    fn test_auto_ack_value() {
        let modem_control_0 = ModemControlRegister0Builder::default()
            .auto_ack(true)
            .build()
            .unwrap();

        assert_eq!(
            modem_control_0.register_value(),
            0b00_0_0_1_010_11_1_1_0010,
        )
    }

    #[test]
    fn test_preamble_length_value() {
        let modem_control_0 = ModemControlRegister0Builder::default()
            .preamble_length(5)
            .build()
            .unwrap();

        assert_eq!(
            modem_control_0.register_value(),
            0b00_0_0_1_010_11_1_0_0101,
        )
    }

    #[test]
    fn test_modem_control_register_0_from_u16() {
        let value: u16 = 0b00_1_1_0_111_01_0_1_1111;

        let expected_modem_control_0 = ModemControlRegister0Builder::default()
            .reserved_frame_mode(true)
            .pan_coordinator(true)
            .adr_decode(false)
            .cca_hyst(7)
            .cca_mode(1)
            .auto_crc(false)
            .auto_ack(true)
            .preamble_length(15)
            .build()
            .unwrap();

        assert_eq!(
            expected_modem_control_0,
            value.into(),
        )
    }

    #[test]
    fn test_correlation_threshold_value() {
        let modem_control_1 = ModemControlRegister1Builder::default()
            .corr_threshold(5)
            .build()
            .unwrap();

        assert_eq!(
            modem_control_1.register_value(),
            0b00000_00101_0_0_00_00,
        )
    }

    #[test]
    fn test_demod_avg_mode_value() {
        let modem_control_1 = ModemControlRegister1Builder::default()
            .demod_average_mode(true)
            .build()
            .unwrap();

        assert_eq!(
            modem_control_1.register_value(),
            0b00000_10100_1_0_00_00,
        )
    }

    #[test]
    fn test_modulation_mode_value() {
        let modem_control_1 = ModemControlRegister1Builder::default()
            .modulation_mode(true)
            .build()
            .unwrap();
        
        assert_eq!(
            modem_control_1.register_value(),
            0b00000_10100_0_1_00_00,
        )
    }

    #[test]
    fn test_tx_mode_value() {
        let modem_control_1 = ModemControlRegister1Builder::default()
            .tx_mode(2)
            .build()
            .unwrap();

        assert_eq!(
            modem_control_1.register_value(),
            0b00000_10100_0_0_10_00,
        )
    }

    #[test]
    fn test_rx_mode_value() {
        let modem_control_1 = ModemControlRegister1Builder::default()
            .rx_mode(2)
            .build()
            .unwrap();

        assert_eq!(
            modem_control_1.register_value(),
            0b00000_10100_0_0_00_10,
        )
    }

    #[test]
    fn test_modem_control_register_1_from_u16() {
        let value: u16 = 0b00000_00011_1_1_01_01;

        let expected_modem_control_1 = ModemControlRegister1Builder::default()
            .corr_threshold(3)
            .demod_average_mode(true)
            .modulation_mode(true)
            .tx_mode(1)
            .rx_mode(1)
            .build()
            .unwrap();

        assert_eq!(
            expected_modem_control_1,
            value.into(),
        )
    }
}