//!
//! Drive for the CC2420 IEEE 802.15.4 Compatible Radio Module
//! 

#![no_std]

extern crate alloc;

use embedded_hal::spi::{SpiDevice, Mode, MODE_0};

mod ram;

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

    /// Reset the Radio
    pub fn reset(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        let register = MainControlRegisterBuilder::default().reset_n(false).build().unwrap();
        self.write_register(&register)
    }

    /// Set the sync word of the Radio
    pub fn set_sync_word(&mut self, value: u16) -> Result<RadioStatus, RadioError<SPIE>> {
        let register = SyncWordRegisterBuilder::default().sync_word(value).build().unwrap();
        self.write_register(&register)
    }

    /// Select the key to use for standalone AES encryption
    pub fn set_standalone_key(&mut self, key_0: bool) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut register = SecurityControlRegister0Builder::default().build().unwrap();
        let _ = self.read_register(&mut register)?;
        register.sec_sa_key_sel = key_0;
        self.write_register(&register)
    }
    
    /// Select the key to use for tx AES encryption
    pub fn set_tx_key(&mut self, key_0: bool) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut register = SecurityControlRegister0Builder::default().build().unwrap();
        let _ = self.read_register(&mut register)?;
        register.sec_tx_key_sel = key_0;
        self.write_register(&register)
    }

    /// Select the key to use for rx AES encryption
    pub fn set_rx_key(&mut self, key_0: bool) -> Result<RadioStatus, RadioError<SPIE>> {
        let mut register = SecurityControlRegister0Builder::default().build().unwrap();
        let _ = self.read_register(&mut register)?;
        register.sec_rx_key_sel = key_0;
        self.write_register(&register)
    }

    /// Read the part number of the radio
    pub fn read_part_number(&mut self) -> Result<u16, RadioError<SPIE>> {
        let mut lower_16_register = LowerManufacturerIDBuilder::default().build().unwrap();
        let _ = self.read_register(&mut lower_16_register)?;
        let mut upper_16_register = UpperManufacturerIDBuilder::default().build().unwrap();
        let _ = self.read_register(&mut upper_16_register)?;
        Ok(upper_16_register.part_num << 4 | (lower_16_register.part_num as u16))
    }

    /// Read the manufacturer id of the radio
    pub fn read_manufacturer(&mut self) -> Result<u16, RadioError<SPIE>> {
        let mut register = LowerManufacturerIDBuilder::default().build().unwrap();
        let _ = self.read_register(&mut register)?;
        Ok(register.manufacturer_id)
    }

    /// Read the version number of the radio
    pub fn version_number(&mut self) -> Result<u8, RadioError<SPIE>> {
        let mut register = UpperManufacturerIDBuilder::default().build().unwrap();
        let _ = self.read_register(&mut register);
        Ok(register.version)
    }

    /// Send Data
    pub fn send(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        todo!()
    }

    /// Check the whether data has been received and return the data if
    /// it exists.
    pub fn receive(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        todo!()
    }

    pub fn set_key_0(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        todo!()
    }

    pub fn set_key_1(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        todo!()
    }

    pub fn set_short_address(&mut self, value: u16) -> Result<RadioStatus, RadioError<SPIE>> {
        let value = value.to_le_bytes();
        let mut buffer = [0x6A | 1 << 7, 0x1 << 2, value[0], value[1]];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    pub fn set_ieee_address(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        todo!()
    }

    pub fn set_pan_id(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        todo!()
    }

    pub fn encrypt(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        todo!()
    }

    pub fn set_tx_nonce(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        todo!()
    }

    pub fn set_rx_nonce(&mut self) -> Result<RadioStatus, RadioError<SPIE>> {
        todo!()
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