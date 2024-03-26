//!
//! Drive for the CC2420 IEEE 802.15.4 Compatible Radio Module
//! 

#![no_std]

extern crate alloc;

use embedded_hal::spi::{SpiDevice, Mode, MODE_0};

mod register;
use register::*;

pub mod error;
pub use error::RadioError;

pub mod status;
pub use status::RadioStatus;

pub mod strobe;
pub use strobe::Strobe;

pub const RADIO_SPI_MODE: Mode = MODE_0;
pub const MAX_SCLK_FREQUENCY: u32 = 10_000_000;

pub struct Radio<SPI, SPIE> where
    SPI: SpiDevice<u8, Error=SPIE> {
    spi: SPI,
}

impl<SPI, SPIE> Radio<SPI, SPIE> where
    SPI: SpiDevice<u8, Error=SPIE> {
    pub fn new(spi: SPI) -> Self {
        Self {
            spi,
        }
    }

    /// Read the status of the radio
    pub fn status(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut buffer = [Strobe::ReadStatus.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Turn on the radio's crystal oscillator
    pub fn xosc_on(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut buffer = [Strobe::XOSCOn.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Calibrate the frequency for Tx.
    pub fn calibrate_tx(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut buffer = [Strobe::CalibrateFrequency.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Enable Rx Mode
    pub fn enable_rx(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut buffer = [Strobe::EnableRx.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Enable Tx Mode
    pub fn enable_tx(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut buffer = [Strobe::EnableTx.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// If CCA indicates a clear channel, enable calibration and switch to tx mode
    pub fn cca_enable_tx(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut buffer = [Strobe::EnableTxCCA.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Safely flush the rx fifo (reading a byte first)
    pub fn flush_rx_fifo(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        // TODO: Read 1 Byte from FIFO
        let mut buffer = [Strobe::FlushRx.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Flush the tx fifo
    pub fn flush_tx_fifo(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut buffer = [Strobe::FlushTx.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Send an acknowledge frame, with pending field cleared.
    pub fn acknowledge_cleared(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut buffer = [Strobe::Ack.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Send an acknowledge frame, with pending field set.
    pub fn acknowledge_set(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut buffer = [Strobe::AckPend.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Enable decryption in-line of the RX FIFO
    pub fn enable_decryption(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut buffer = [Strobe::RxDecryption.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Enable encryption in-line of the TX FIFO
    pub fn enable_encryption(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut buffer = [Strobe::TxEncryption.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// AES Stand alone encryption.
    pub fn aes_encryption(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut buffer = [Strobe::AesEncryption.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Write some register value into a given register
    pub fn write_register(&mut self, register: &dyn register::Register) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut buffer = register.write_value();
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Read the register value into itself and return the status
    /// 
    /// TODO: I'm not happy with this definition, so I may change it in the future.
    /// Mostly, I thought it would be neat if register was a trait instead of an enum,
    /// but that seems to have made definitions a bit more odd than I would have liked.
    pub fn read_register(&mut self, register: &mut dyn register::Register) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut buffer = [0u8; 3];
        buffer[0] = register.read_address();
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        let status = buffer[0].into();
        register.from_buffer(buffer);
        Ok(status)
    }
}