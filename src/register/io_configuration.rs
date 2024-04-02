//!
//! Rust Definition of the IO Configuration Register
//! 

use super::Register;

use alloc::string::String;

use derive_builder::Builder;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(validate = "Self::validate"))]
pub struct IOConfigurationRegister {
    // Accept all beacon frames when address recognition is enabled.
    // This bit should be set when the PAN identifier programmed into
    // CC2420 RAM is equal to 0xFFFF and cleared otherwise.  This bit
    // is don't care wehn MDMCTRL0.ADR_DECODE=0
    // 0: Only accept beacons with a source PAN identifier which
    // matches the PAN identifier programmed into CC2420 RAM.
    // 1: Accept all beacons regardless of the source PAN identifier
    #[builder(default = "false")]
    pub bcn_accept: bool,
    // Polarity of the output signal FIFO.
    // 0: Polarity is active high
    // 1: Polarity is active low
    #[builder(default = "false")]
    pub fifo_polarity: bool,
    // Polarity of the output signal FIFOP.
    // 0: Polarity is active high
    // 1: Polarity is active low
    #[builder(default = "false")]
    pub fifop_polarity: bool,
    // Polarity of the SFD pin.
    // 0: Polarity is active high
    // 1: Polarity is active low
    #[builder(default = "false")]
    pub sfd_polarity: bool,
    // Polarity of the CCA pin.
    // 0: Polarity is active high
    // 1: Polarity is active low
    #[builder(default = "false")]
    pub cca_polarity: bool,
    // FIFOP_THR sets the threshold in number of bytes in the RXFIFO for
    // FIFOP to go active.
    #[builder(default = "64")]
    pub fifop_threshold: u8,
}

impl Register for IOConfigurationRegister {
    fn register_value(&self) -> u16 {
        let mut value = 0;

        if self.bcn_accept {
            value |= 1 << 11;
        }

        if self.fifo_polarity {
            value |= 1 << 10;
        }

        if self.fifop_polarity {
            value |= 1 << 9;
        }

        if self.sfd_polarity {
            value |= 1 << 8;
        }

        if self.cca_polarity {
            value |= 1 << 7;
        }

        value |= self.fifop_threshold as u16;

        value
    }

    fn address(&self) -> u8 { 0x1C }

    fn fill_from_buffer(&mut self, buffer: [u8; 3]) {
        *self = u16::from_le_bytes(buffer[1..3].try_into().unwrap()).into();
    }
}

impl From<u16> for IOConfigurationRegister {
    fn from(value: u16) -> Self {
        Self {
            bcn_accept: (((1 << 11) & value) != 0),
            fifo_polarity: (((1 << 10) & value) != 0),
            fifop_polarity: (((1 << 9) & value) != 0),
            sfd_polarity: (((1 << 8) & value) != 0),
            cca_polarity: (((1 << 7) & value) != 0),
            fifop_threshold: (value & 0x7F) as u8,
        }
    }
}

impl IOConfigurationRegisterBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(fifop_threshold) = self.fifop_threshold {
            if fifop_threshold > 0x7F {
                return Err("Invalid FIFOP_THR. 0<=FIFO_THR<=127".into());
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(validate = "Self::validate"))]
pub struct IOConfigurationRegister1 {
    // The HSSD Module is used as follows:
    // 0: Off
    // 1: Output AGC status (gain setting  / peak detector status /
    // accumulator value)
    // 2: Output ADC I and Q values.
    // 3. Output I/Q after digital down mix and channel filtering.
    // 4: Reserved
    // 5: Reserved
    // 6: Input ADC I and Q Values
    // 7: Input DAC I and Q Values
    // The HSSD Module requires that the FS is up and running as it
    // uses CLK_PRE(~150 MHZ) to product its ~37.5 MHz data clock
    // and serialize its output words.
    #[builder(default = "0")]
    pub hssd_src: u8,
    // Multiplexer setting for the SFD pin.
    #[builder(default = "0")]
    pub sfd_mux: u8,
    // Multiplexer setting for the CCA pin.
    #[builder(default = "0")]
    pub cca_mux: u8,
}

impl Register for IOConfigurationRegister1 {
    fn register_value(&self) -> u16 {
        let mut value = 0;

        value |= (self.hssd_src as u16) << 10;

        value |= (self.sfd_mux as u16) << 5;

        value |= self.cca_mux as u16;

        value
    }

    fn address(&self) -> u8 { 0x1D }

    fn fill_from_buffer(&mut self, buffer: [u8; 3]) {
        *self = u16::from_le_bytes(buffer[1..3].try_into().unwrap()).into();
    }
}

impl From<u16> for IOConfigurationRegister1 {
    fn from(value: u16) -> Self {
        Self {
            hssd_src: (((0b111 << 10) & value) >> 10) as u8,
            sfd_mux: (((0b11111 << 5) & value) >> 5) as u8,
            cca_mux: (0b11111 & value) as u8,
        }
    }
}

impl IOConfigurationRegister1Builder {
    fn validate(&self) -> Result<(), String> {
        if let Some(hssd_src) = self.hssd_src {
            if hssd_src == 4 || hssd_src == 5 || hssd_src > 7 {
                return Err("Invalid HSSD_SRC. 0<=HSSD_SRC<=3 U 6<=HSSD_SRC<=7".into());
            }
        }

        if let Some(sfd_mux) = self.sfd_mux {
            if sfd_mux > 31 {
                return Err("Invalid SFDMUX. 0<=SFDMUX<=31".into());
            }
        }

        if let Some(cca_mux) = self.cca_mux {
            if cca_mux > 31 {
                return Err("Invalid CCAMUX. 0<=CCAMUX<=31".into());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcn_accept_value() {
        let io_configuration = IOConfigurationRegisterBuilder::default()
            .bcn_accept(true)
            .build()
            .unwrap();

        assert_eq!(
            io_configuration.register_value(),
            0b1_0_0_0_0_1000000,
        )
    }

    #[test]
    fn test_fifo_polarity_value() {
        let io_configuration = IOConfigurationRegisterBuilder::default()
            .fifo_polarity(true)
            .build()
            .unwrap();

        assert_eq!(
            io_configuration.register_value(),
            0b0_1_0_0_0_1000000,
        )
    }

    #[test]
    fn test_fifop_polarity_value() {
        let io_configuration = IOConfigurationRegisterBuilder::default()
            .fifop_polarity(true)
            .build()
            .unwrap();

        assert_eq!(
            io_configuration.register_value(),
            0b0_0_1_0_0_1000000,
        )
    }

    #[test]
    fn test_sfd_polarity_value() {
        let io_configuration = IOConfigurationRegisterBuilder::default()
            .sfd_polarity(true)
            .build()
            .unwrap();

        assert_eq!(
            io_configuration.register_value(),
            0b0_0_0_1_0_1000000,
        )
    }

    #[test]
    fn test_cca_polarity_value() {
        let io_configuration = IOConfigurationRegisterBuilder::default()
            .cca_polarity(true)
            .build()
            .unwrap();

        assert_eq!(
            io_configuration.register_value(),
            0b0_0_0_0_1_1000000,
        )
    }

    #[test]
    fn test_fifop_threshold_value() {
        let io_configuration = IOConfigurationRegisterBuilder::default()
            .fifop_threshold(127)
            .build()
            .unwrap();

        assert_eq!(
            io_configuration.register_value(),
            0b0_0_0_0_0_1111111,
        )
    }

    #[test]
    fn test_io_configuration_from_u16() {
        let value = 0b1_1_1_1_1_1111111;

        let expected_io_configuration = IOConfigurationRegisterBuilder::default()
            .bcn_accept(true)
            .fifo_polarity(true)
            .fifop_polarity(true)
            .sfd_polarity(true)
            .cca_polarity(true)
            .fifop_threshold(127)
            .build()
            .unwrap();

        assert_eq!(
            expected_io_configuration,
            value.into(),
        )
    }

    #[test]
    fn test_hssd_src_value() {
        let io_configuration = IOConfigurationRegister1Builder::default()
            .hssd_src(3)
            .build()
            .unwrap();

        assert_eq!(
            io_configuration.register_value(),
            0b011_00000_00000,
        )
    }

    #[test]
    fn test_sfd_mux_value() {
        let io_configuration = IOConfigurationRegister1Builder::default()
            .sfd_mux(5)
            .build()
            .unwrap();

        assert_eq!(
            io_configuration.register_value(),
            0b00101_00000,
        )
    }

    #[test]
    fn test_cca_mux_value() {
        let io_configuration = IOConfigurationRegister1Builder::default()
            .cca_mux(5)
            .build()
            .unwrap();

        assert_eq!(
            io_configuration.register_value(),
            0b00101,
        )
    }

    #[test]
    fn test_io_configuration_1_from_u16() {
        let value = 0b111_11111_11111;

        let expected_io_configuration = IOConfigurationRegister1Builder::default()
            .hssd_src(7)
            .sfd_mux(31)
            .cca_mux(31)
            .build()
            .unwrap();

        assert_eq!(
            expected_io_configuration,
            value.into(),
        )
    }
}