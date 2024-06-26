//!
//! Drive for the CC2420 IEEE 802.15.4 Compatible Radio Module
//! 

#![no_std]

extern crate alloc;
use alloc::string::ToString;

use core::cmp::min;

use alloc::vec;
use alloc::vec::Vec;

use embedded_hal::spi::{SpiDevice, Mode, MODE_0};
use embedded_hal::digital::InputPin;
use embedded_hal::delay::DelayNs;

mod ram;
use ram::Ram;

mod register;
use register::*;

pub mod error;
pub use error::RadioError;

pub mod status;
pub use status::RadioStatus;

pub mod strobe;
pub use strobe::Strobe;

pub mod config;
pub use config::Configuration;

pub const RADIO_SPI_MODE: Mode = MODE_0;
pub const MAX_SCLK_FREQUENCY: u32 = 10_000_000;

// Delay (for configuration) to wait before checking the register value has
// been updated
const REGISTER_WRITE_DELAY_US: u32 = 100;
// Delay (for configuration) to wait before checking the value in RAM has
// been updated
const RAM_WRITE_DELAY_US: u32 = 100;

pub struct Radio<SPI, SPIE, SFD, GPIOE, FIFO> where
    SPI: SpiDevice<u8, Error=SPIE>,
    SFD: InputPin<Error=GPIOE>,
    FIFO: InputPin<Error=GPIOE> {
    // Whether or not the radio is powered up
    pub powered_up: bool,
    // SPI Peripheral Device
    spi: SPI,
    // Data Sent Interrupt
    sfd: SFD,
    // Data Received Interrupt
    fifo: FIFO,
}

impl<SPI, SPIE, SFD, GPIOE, FIFO> Radio<SPI, SPIE, SFD, GPIOE, FIFO> where
    SPI: SpiDevice<u8, Error=SPIE>,
    SFD: InputPin<Error=GPIOE>,
    FIFO: InputPin<Error=GPIOE> {
    pub fn new(spi: SPI, sfd: SFD, fifo: FIFO) -> Self {
        Self {
            powered_up: false,
            spi,
            sfd,
            fifo,
        }
    }

    /// Apply a given configuration to the radio and starting the crystal oscillator on the radio.
    pub fn configure(&mut self, config: Configuration, delay: &mut dyn DelayNs) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        // Modem Configuration
        let modem_config = ModemControlRegister0Builder::default()
            .pan_coordinator(config.pan_coordinator)
            .adr_decode(config.address_decoding)
            .auto_crc(config.enable_crc)
            .auto_ack(config.auto_acknowledge)
            .preamble_length(config.preamble_length)
            .build()
            .map_err(|e| { RadioError::InvalidConfiguration(e.to_string()) })?;
        self.write_register(&modem_config)?;
        delay.delay_us(REGISTER_WRITE_DELAY_US);
        let mut found_modem_config = ModemControlRegister0Builder::default().build().unwrap();
        self.read_register(&mut found_modem_config)?;
        if found_modem_config != modem_config {
            return Err(RadioError::FailedConfiguration("Configuration of Modem Failed"));
        }

        // Sync Word Configuration
        let sync_word = SyncWordRegisterBuilder::default()
            .sync_word(u16::from_le_bytes(config.sync_word))
            .build()
            .unwrap();
        self.write_register(&sync_word)?;
        delay.delay_us(REGISTER_WRITE_DELAY_US);
        let mut found_sync_word = SyncWordRegisterBuilder::default().build().unwrap();
        self.read_register(&mut found_sync_word)?;
        if found_sync_word != sync_word {
            return Err(RadioError::FailedConfiguration("Configuration of Sync Word Failed"));
        }

        // Set Short Address
        self.set_short_address(u16::from_le_bytes(config.short_address))?;
        delay.delay_us(RAM_WRITE_DELAY_US);
        let found_short_address = self.read_short_address()?.to_le_bytes();
        if found_short_address != config.short_address {
            return Err(RadioError::FailedConfiguration("Configuration of Short Address Failed"));
        }

        // Set Pan ID
        self.set_pan_id(u16::from_le_bytes(config.pan_identifier))?;
        delay.delay_us(RAM_WRITE_DELAY_US);
        let found_pan_id = self.read_pan_id()?.to_le_bytes();
        if found_pan_id != config.pan_identifier {
            return Err(RadioError::FailedConfiguration("Configuration of Pan ID Failed"));
        }

        // Set IEEE Address
        self.set_ieee_address(config.ieee_address)?;
        delay.delay_us(RAM_WRITE_DELAY_US);
        let found_ieee_address = self.read_ieee_address()?;
        if found_ieee_address != config.ieee_address {
            return Err(RadioError::FailedConfiguration("Configuration of IEEE Address Failed"));
        }

        // Set Tx Encryption Key
        self.set_key_1(config.tx_encryption_key)?;
        delay.delay_us(RAM_WRITE_DELAY_US);
        let found_tx_key = self.read_key_1()?;
        if found_tx_key != config.tx_encryption_key {
            return Err(RadioError::FailedConfiguration("Configuration of Tx Encryption Key Failed"));
        }

        // Set Rx Decryption Key
        self.set_key_0(config.rx_decryption_key)?;
        delay.delay_us(RAM_WRITE_DELAY_US);
        let found_rx_key = self.read_key_0()?;
        if found_rx_key != config.rx_decryption_key {
            return Err(RadioError::FailedConfiguration("Configuration of Rx Decryption Key Failed"));
        }

        // Start up the crystal oscillator
        let mut status = self.xosc_on()?;
        while !status.xosx_stable {
            status = self.status()?;
            delay.delay_us(100);
        }

        self.powered_up = true;

        // Start to Calibrate Tx Frequency
        self.calibrate_tx()
    }

    /// Power up the Radio
    pub fn power_up(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = [Strobe::XOSCOn.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        self.powered_up = true;
        Ok(buffer[0].into())
    }

    /// Power down the Radio
    pub fn power_down(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = [Strobe::DisableRxTx.opcode()];
        self.spi.write(&buffer).map_err(RadioError::SpiError)?;
        buffer[0] = Strobe::XOSCOff.opcode();
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        self.powered_up = false;
        Ok(buffer[0].into())
    }

    /// Reset the Radio
    pub fn reset(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let register = MainControlRegisterBuilder::default().reset_n(false).build().unwrap();
        self.write_register(&register)
    }

    /// Set the sync word of the Radio
    pub fn set_sync_word(&mut self, value: u16) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let register = SyncWordRegisterBuilder::default().sync_word(value).build().unwrap();
        self.write_register(&register)
    }

    /// Select the key to use for standalone AES encryption
    pub fn set_standalone_key(&mut self, key_0: bool) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut register = SecurityControlRegister0Builder::default().build().unwrap();
        let _ = self.read_register(&mut register)?;
        register.sec_sa_key_sel = key_0;
        self.write_register(&register)
    }
    
    /// Select the key to use for tx AES encryption
    pub fn set_tx_key(&mut self, key_0: bool) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut register = SecurityControlRegister0Builder::default().build().unwrap();
        let _ = self.read_register(&mut register)?;
        register.sec_tx_key_sel = key_0;
        self.write_register(&register)
    }

    /// Select the key to use for rx AES encryption
    pub fn set_rx_key(&mut self, key_0: bool) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut register = SecurityControlRegister0Builder::default().build().unwrap();
        let _ = self.read_register(&mut register)?;
        register.sec_rx_key_sel = key_0;
        self.write_register(&register)
    }

    /// Read the part number of the radio
    pub fn read_part_number(&mut self) -> Result<u16, RadioError<SPIE, GPIOE>> {
        let mut lower_16_register = LowerManufacturerIDBuilder::default().build().unwrap();
        let _ = self.read_register(&mut lower_16_register)?;
        let mut upper_16_register = UpperManufacturerIDBuilder::default().build().unwrap();
        let _ = self.read_register(&mut upper_16_register)?;
        Ok(upper_16_register.part_num << 4 | (lower_16_register.part_num as u16))
    }

    /// Read the manufacturer id of the radio
    pub fn read_manufacturer(&mut self) -> Result<u16, RadioError<SPIE, GPIOE>> {
        let mut register = LowerManufacturerIDBuilder::default().build().unwrap();
        let _ = self.read_register(&mut register)?;
        Ok(register.manufacturer_id)
    }

    /// Read the version number of the radio
    pub fn version_number(&mut self) -> Result<u8, RadioError<SPIE, GPIOE>> {
        let mut register = UpperManufacturerIDBuilder::default().build().unwrap();
        let _ = self.read_register(&mut register);
        Ok(register.version)
    }

    /// Send a Frame (<=128 Bytes of Data)
    pub fn send_frame(&mut self, data: &[u8], cca: bool) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let _ = self.flush_tx_fifo()?;
        if data.len() > 128 {
            return Err(RadioError::InvalidBufferLenth { expected: 128, found: data.len() });
        }

        let mut buffer = [0u8; 129];
        buffer[0] = Strobe::TxFifo.opcode();
        buffer[1..(1+data.len())].copy_from_slice(data);
        self.spi.transfer_in_place(&mut buffer[..(1+data.len())]).map_err(RadioError::SpiError)?;

        let mut buffer = [0u8];
        if cca {
            buffer[0] = Strobe::EnableTxCCA.opcode();
        } else {
            buffer[0] = Strobe::EnableTx.opcode();
        }
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Send Data
    pub fn send(&mut self, data: &[u8], cca: bool, delay: &mut dyn DelayNs) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let _ = self.flush_tx_fifo()?;

        for start in 0..(data.len()/128) {
            let mut data_buffer = [0u8; 129];
            data_buffer[0] = Strobe::TxFifo.opcode();
            data_buffer[1..129].copy_from_slice(&data[start*128..(start+1)*128]);
            self.spi.write(&data_buffer).map_err(RadioError::SpiError)?;

            let buffer = if cca { [Strobe::EnableTxCCA.opcode()] } else { [Strobe::EnableTx.opcode()] };
            self.spi.write(&buffer).map_err(RadioError::SpiError)?;

            while self.sfd.is_low().map_err(RadioError::GpioError)? {
                delay.delay_us(100);
            }
        }

        let final_frame = &data[data.len()/128..];
        let mut data_buffer = [0u8; 129];
        data_buffer[0] = Strobe::TxFifo.opcode();
        data_buffer[1..(1+final_frame.len())].copy_from_slice(final_frame);
        self.spi.write(&data_buffer[..(1+final_frame.len())]).map_err(RadioError::SpiError)?;

        let mut buffer = if cca { [Strobe::EnableTxCCA.opcode()] } else { [Strobe::EnableTx.opcode()] };
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;

        Ok(buffer[0].into())
    }

    /// Read the Data from the TX FIFO (Presumably only used for testing)
    pub fn read_tx_fifo(&mut self) -> Result<[u8; 128], RadioError<SPIE, GPIOE>> {
        let mut buffer = [0u8; 128];
        let _ = self.read_ram(Ram::TxFifo, &mut buffer)?;
        Ok(buffer)
    }

    /// Check if Data is Ready (the FIFO pin can also be configured as an
    /// interrupt, which completes the same functionality as this)
    pub fn data_ready(&mut self) -> Result<bool, RadioError<SPIE, GPIOE>> {
        self.fifo.is_high().map_err(RadioError::GpioError)
    }

    /// Start Receiving Data
    pub fn start_receiving(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = [Strobe::EnableRx.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Read data from the RX FIFO (equal to the length of the buffer) into a
    /// given buffer, returning the radio status
    pub fn receive(&mut self, buffer: &mut [u8]) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let data_len = min(129, buffer.len());
        let mut read_buffer = [0u8; 129];
        read_buffer[0] = Strobe::RxFifo.opcode();
        self.spi.transfer_in_place(&mut read_buffer[..=data_len]).map_err(RadioError::SpiError)?;
        buffer[..].copy_from_slice(&read_buffer[1..=data_len]);
        Ok(buffer[0].into())
    }

    /// Not sure why you would want to do this, but the use case is outlined in
    /// the datasheet for testing, so this is included for continuity sake
    pub fn write_rx_fifo(&mut self, data: [u8; 128]) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        self.write_ram(Ram::RxFifo, &data)
    }

    /// Read the entire contents of the RX FIFO.  In general, receive() should be
    /// used in place of this function, however it may be useful for debugging
    /// purposes.
    pub fn read_rx_fifo(&mut self) -> Result<[u8; 128], RadioError<SPIE, GPIOE>> {
        let mut buffer = [0u8; 128];
        let _ = self.read_ram(Ram::RxFifo, &mut buffer)?;
        Ok(buffer)
    }

    /// Set the Encryption / Decryption Key 0's value in RAM.
    pub fn set_key_0(&mut self, key: [u8; 16]) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        self.write_ram(Ram::Key0, &key)
    }

    /// Read the Encryption / Decryption Key 0's value from RAM.
    pub fn read_key_0(&mut self) -> Result<[u8; 16], RadioError<SPIE, GPIOE>> {
        let mut buffer = [0u8; 16];
        let _ = self.read_ram(Ram::Key0, &mut buffer)?;
        Ok(buffer)
    }

    /// Set the Encryption / Decryption Key 1's value in RAM.
    pub fn set_key_1(&mut self, key: [u8; 16]) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        self.write_ram(Ram::Key1, &key)
    }

    /// Read the Encryption / Decryption Key 1's value from RAM.
    pub fn read_key_1(&mut self) -> Result<[u8; 16], RadioError<SPIE, GPIOE>> {
        let mut buffer = [0u8; 16];
        let _ = self.read_ram(Ram::Key1, &mut buffer)?;
        Ok(buffer)
    }

    /// Set the 16-bit short address used for address recognition.
    /// 
    /// Note: The value is passed in as a u16 and converted to big
    /// endian bytes.
    pub fn set_short_address(&mut self, value: u16) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let bytes = value.to_be_bytes();
        self.write_ram(Ram::ShortAddress, &bytes)
    }

    /// Read the 16-bit short address for address recognition.
    /// 
    /// Note: The value returned is a u16 converted from big endian
    /// bytes.
    pub fn read_short_address(&mut self) -> Result<u16, RadioError<SPIE, GPIOE>> {
        let mut buffer = [0u8; 2];
        let _ = self.read_ram(Ram::ShortAddress, &mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    /// Set the 64-bit IEEE address of the current node, used for
    /// address recognition
    pub fn set_ieee_address(&mut self, address: [u8; 8]) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        self.write_ram(Ram::IEEEAddress, &address)
    }

    /// Read the 64-bit IEEE address of the current node.  Used for
    /// address recognition
    pub fn read_ieee_address(&mut self) -> Result<[u8; 8], RadioError<SPIE, GPIOE>> {
        let mut buffer = [0u8; 8];
        let _ = self.read_ram(Ram::IEEEAddress, &mut buffer)?;
        Ok(buffer)
    }

    /// Set the 16-bit PAN identifier for address recognition.
    /// 
    /// Note: The value is passed in as a u16 and converted to big
    /// endian bytes.
    pub fn set_pan_id(&mut self, value: u16) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let bytes = value.to_be_bytes();
        self.write_ram(Ram::PanID, &bytes)
    }

    /// Read the 16-bit PAN identifier for address recognition.
    /// 
    /// Note: The value returned is a u16 converted from big endian
    /// bytes.
    pub fn read_pan_id(&mut self) -> Result<u16, RadioError<SPIE, GPIOE>> {
        let mut buffer = [0u8; 2];
        let _ = self.read_ram(Ram::PanID, &mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    /// Encrypt 128-bits of data using AES encryption and the selected key, using
    /// data as an intermediary buffer
    /// 
    /// TODO: Check the timing for this
    pub fn encrypt(&mut self, mut data: [u8; 16]) -> Result<[u8; 16], RadioError<SPIE, GPIOE>> {
        let _ = self.write_ram(Ram::EncryptionBuffer, &data)?;
        let _ = self.aes_encryption()?;
        let _ = self.read_ram(Ram::EncryptionBuffer, &mut data)?;
        Ok(data)
    }

    /// Set the Nonce used in TX in-line authentication and transmitter
    /// counter for in-line encryption
    pub fn set_tx_nonce(&mut self, value: [u8; 16]) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        self.write_ram(Ram::TxNonce, &value)
    }

    /// Read the Nonce used for TX in-line authentication and transmitter
    /// counter used for in-line encryption
    pub fn read_tx_nonce(&mut self) -> Result<[u8; 16], RadioError<SPIE, GPIOE>> {
        let mut buffer = [0u8; 16];
        let _ = self.read_ram(Ram::TxNonce, &mut buffer)?;
        Ok(buffer)
    }

    /// Set the Nonce used for RX in-line authentication or receiver counter for
    /// in-line decryption
    pub fn set_rx_nonce(&mut self, value: [u8; 16]) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        self.write_ram(Ram::RxNonce, &value)
    }

    /// Read the Nonce used for RX in-line authentication or receiver counter for
    /// in line-decryption
    pub fn read_rx_nonce(&mut self) -> Result<[u8; 16], RadioError<SPIE, GPIOE>> {
        let mut buffer = [0u8; 16];
        let _ = self.read_ram(Ram::RxNonce, &mut buffer)?;
        Ok(buffer)
    }

    /// Read the status of the radio
    pub fn status(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = [Strobe::ReadStatus.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Turn on the radio's crystal oscillator
    pub fn xosc_on(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = [Strobe::XOSCOn.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Calibrate the frequency for Tx.
    pub fn calibrate_tx(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = [Strobe::CalibrateFrequency.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Enable Rx Mode
    pub fn enable_rx(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = [Strobe::EnableRx.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Enable Tx Mode
    pub fn enable_tx(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = [Strobe::EnableTx.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// If CCA indicates a clear channel, enable calibration and switch to tx mode
    pub fn cca_enable_tx(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = [Strobe::EnableTxCCA.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Safely flush the rx fifo (reading a byte first)
    pub fn flush_rx_fifo(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        // TODO: Read 1 Byte from FIFO
        let mut buffer = [Strobe::FlushRx.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Flush the tx fifo
    pub fn flush_tx_fifo(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = [Strobe::FlushTx.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Send an acknowledge frame, with pending field cleared.
    pub fn acknowledge_cleared(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = [Strobe::Ack.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Send an acknowledge frame, with pending field set.
    pub fn acknowledge_set(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = [Strobe::AckPend.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Enable decryption in-line of the RX FIFO
    pub fn enable_decryption(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = [Strobe::RxDecryption.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Enable encryption in-line of the TX FIFO
    pub fn enable_encryption(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = [Strobe::TxEncryption.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// AES Stand alone encryption.
    pub fn aes_encryption(&mut self) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = [Strobe::AesEncryption.opcode()];
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Write some register value into a given register
    pub fn write_register(&mut self, register: &dyn register::Register) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = register.write_value();
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Read the register value into itself and return the status
    /// 
    /// TODO: I'm not happy with this definition, so I may change it in the future.
    /// Mostly, I thought it would be neat if register was a trait instead of an enum,
    /// but that seems to have made definitions a bit more odd than I would have liked.
    pub fn read_register(&mut self, register: &mut dyn register::Register) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        let mut buffer = [0u8; 3];
        buffer[0] = register.read_address();
        self.spi.transfer_in_place(&mut buffer).map_err(RadioError::SpiError)?;
        let status = buffer[0].into();
        register.fill_from_buffer(buffer);
        Ok(status)
    }

    /// Write to a given location in RAM.
    fn write_ram(&mut self, ram: Ram, data: &[u8]) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        if data.len() != ram.length() {
            return Err(RadioError::InvalidBufferLenth { expected: ram.length(), found: data.len() });
        }
        let mut buffer = Vec::with_capacity(2 + data.len());
        let address = ram.write_address();
        buffer.push(address.0);
        buffer.push(address.1);
        for byte in data {
            buffer.push(*byte);
        }
        self.spi.transfer_in_place(buffer.as_mut_slice()).map_err(RadioError::SpiError)?;
        Ok(buffer[0].into())
    }

    /// Read from a given location in RAM.
    fn read_ram(&mut self, ram: Ram, buffer: &mut [u8]) -> Result<RadioStatus, RadioError<SPIE, GPIOE>> {
        if buffer.len() != ram.length() {
            return Err(RadioError::InvalidBufferLenth { expected: ram.length(), found: buffer.len() });
        }
        let mut write_buffer = vec![0u8; 2 + buffer.len()];
        let address = ram.read_address();
        write_buffer[0] = address.0;
        write_buffer[1] = address.1;
        self.spi.transfer_in_place(&mut write_buffer).map_err(RadioError::SpiError)?;
        buffer[..].copy_from_slice(&write_buffer.as_slice()[2..]);
        Ok(write_buffer[0].into())
    }
}