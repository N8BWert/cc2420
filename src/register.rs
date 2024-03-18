//!
//! Register Definitions for the CC2420 Radio
//! 

#![allow(unused)]

mod main_control;
mod modem_control;
mod rssi;
mod sync;
mod transmit_control;
mod receive_control;
mod frequency_synthesis;
mod security_control;
mod battery_monitor;
mod io_configuration;
mod manufacturer_id;
mod fsm;
mod override_registers;
mod agc;

/// Encode the value of struct registers to their u16 representation
pub trait RegisterValue {
    /// Encode a register to a u16
    fn register_value(&self) -> u16;
}

/// Register Map of the CC2420 Module
pub enum Register {
    // No Operation Register
    SNOP,
    // Turn on the crystal oscillator (set XOSC16M_PD = 0 and BIAS_PD = 0)
    SXOSXON,
    // Enable and calibrate frequency synthesizer for TX
    STXCAL,
    // Enable RX
    SRXON,
    // Enable TX after calibration (if not already performed)
    // Start TX in-line encryption if SPI_SEC_MODE != 0
    STXON,
    // If CCA Indicates a clear channel:
    //  Enable calibration, then TX and start in-line encryption if SPI_SEC_MODE != 0
    STXONCCA,
    // Disable RX/TX frequency synthesizer
    SRFOFF,
    // Turn off the crystal oscillator and RF
    SXOSCOFF,
    // Flush the RX FIFO Buffer and reset the demodulator.
    // Always read at least one byte from the RX FIFO before issuing
    SFLUSHRX,
    // Flush the TX FIFO
    SFLUSHTX,
    // Send acknowledge frame, with pending field cleared
    SACK,
    // Send acknowledge frame, with pending field set
    SACKPEND,
    // Start RX FIFO in-line decryption / authentication (as set by SPI_SEC_MODE)
    SRXDEC,
    // Start TX FIFO in-line encryption / authentication (as set by SPI_SEC_MODE)
    //  without starting TX
    STXENC,
    // AES Stand alone encryption strobe. SPI_SEC_MODE is not required to be 0, but
    // the encryption module must be idle.  If not, the strobe is ignored
    SAES,
    // 0x0F is Unused
    // Main Control Register
    MAIN,
    // Modem Control Register 0
    MDMCTRL0,
    // Modem Control Register 1
    MDMCTRL1,
    // RSSI and CCA Status and Control Register
    RSSI,
    // Synchronisation word control register
    SYNCWORD,
    // Transmit Control Register
    TXCTRL,
    // Receive Control Register 0
    RXCTRL0,
    // Receive Control Register 1
    RXCTRL1,
    // Frequency Synthesizer Control and Status Register
    FSCTRL,
    // Security Control Register 0
    SECCTRL0,
    // Security Control Register 1
    SECCTRL1,
    // Battery Monitor Control and Status Register
    BATTMON,
    // Input / Output Control Register 0
    IOCFG0,
    // Input / Output Control Register 1
    IOCFG1,
    // Manufacturer ID, Low 16 bits
    MANFIDL,
    // Manufacturer ID, High 16 bits
    MANFIDH,
    // Finite State Machine Time Constants
    FSMTC,
    // Manual signal AND override register
    MANAND,
    // Manual signal OR override register
    MANOR,
    // AGC Control Register
    AGCCTRL,
    // AGC Test Register 0
    AGCTST0,
    // AGC Test Register 1
    AGCTST1,
    // AGC Test Register 2
    AGCTST2,
    // Frequency Synthesizer Test Register 0
    FSTST0,
    // Frequency Synthesizer Test Register 1
    FSTST1,
    // Frequency Synthesizer Test Register 2
    FSTST2,
    // Frequency Synthesizer Test Register 3
    FSTST3,
    // Receiver Bandpass Filter Test Register
    RXBPFTST,
    // Finite State Machine State Status Register
    FSMSTATE,
    // ADC Test Register
    ADCTST,
    // DAC Test Register
    DACTST,
    // Top Level Test Register
    TOPTST,
    // Transmit FIFO Byte Register
    TXFIFO,
    // Receiver FIFO Byte Register
    RXFIFO,
}

impl Register {
    pub fn is_strobe(register: Register) -> bool {
        match register.address() {
            0x00..=0x0E => true,
            _ => false,
        }
    }

    pub fn is_rw_register(register: Register) -> bool {
        match register.address() {
            0x10..=0x30 => true,
            _ => false,
        }
    }

    pub fn address(self) -> u8 {
        match self {
            Self::SNOP => 0x00,
            Self::SXOSXON => 0x01,
            Self::STXCAL => 0x02,
            Self::SRXON => 0x03,
            Self::STXON => 0x04,
            Self::STXONCCA => 0x05,
            Self::SRFOFF => 0x06,
            Self::SXOSCOFF => 0x07,
            Self::SFLUSHRX => 0x08,
            Self::SFLUSHTX => 0x09,
            Self::SACK => 0x0A,
            Self::SACKPEND => 0x0B,
            Self::SRXDEC => 0x0C,
            Self::STXENC => 0x0D,
            Self::SAES => 0x0E,
            Self::MAIN{..} => 0x10,
            Self::MDMCTRL0 => 0x11,
            Self::MDMCTRL1 => 0x12,
            Self::RSSI => 0x13,
            Self::SYNCWORD => 0x14,
            Self::TXCTRL => 0x15,
            Self::RXCTRL0 => 0x16,
            Self::RXCTRL1 => 0x17,
            Self::FSCTRL => 0x18,
            Self::SECCTRL0 => 0x19,
            Self::SECCTRL1 => 0x1A,
            Self::BATTMON => 0x1B,
            Self::IOCFG0 => 0x1C,
            Self::IOCFG1 => 0x1D,
            Self::MANFIDL => 0x1E,
            Self::MANFIDH => 0x1F,
            Self::FSMTC => 0x20,
            Self::MANAND => 0x21,
            Self::MANOR => 0x22,
            Self::AGCCTRL => 0x23,
            Self::AGCTST0 => 0x24,
            Self::AGCTST1 => 0x25,
            Self::AGCTST2 => 0x26,
            Self::FSTST0 => 0x27,
            Self::FSTST1 => 0x28,
            Self::FSTST2 => 0x29,
            Self::FSTST3 => 0x2A,
            Self::RXBPFTST => 0x2B,
            Self::FSMSTATE => 0x2C,
            Self::ADCTST => 0x2D,
            Self::DACTST => 0x2E,
            Self::TOPTST => 0x2F,
            Self::TXFIFO => 0x3E,
            Self::RXFIFO => 0x3F,
        }
    }
}