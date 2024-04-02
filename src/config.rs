//!
//! Quick Configuration for the CC2420 Radio
//! 

use derive_builder::Builder;

/// Ease-of-use configuration for the CC2420 Radio Module
#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(error(validation_error = false)))]
pub struct Configuration {
    // Should the module be a PAN Coordinator
    #[builder(default = "false")]
    pub pan_coordinator: bool,
    // Address Decoding Enabled
    #[builder(default = "true")]
    pub address_decoding: bool,
    // CRC Enabled
    #[builder(default = "true")]
    pub enable_crc: bool,
    // Auto Acknowledge Enabled
    #[builder(default = "false")]
    pub auto_acknowledge: bool,
    // Preamble Length (n+1 bytes)
    #[builder(default = "2")]
    pub preamble_length: u8,
    // Sync Word
    #[builder(default = "[0xA7, 0x0F]")]
    pub sync_word: [u8; 2],
    // 16-bit short address
    #[builder(default = "[0x12, 0x34]")]
    pub short_address: [u8; 2],
    // 16-bit PAN identifier
    #[builder(default = "[0x12, 0x34]")]
    pub pan_identifier: [u8; 2],
    // 64-bit IEEE Address
    #[builder(default = "[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]")]
    pub ieee_address: [u8; 8],
    // 128-bit Encryption Key (Tx)
    #[builder(
        default = "[0x00u8; 16]"
    )]
    pub tx_encryption_key: [u8; 16],
    // 128-bit Encryption Key (Rx)
    #[builder(
        default = "[0x00u8; 16]"
    )]
    pub rx_decryption_key: [u8; 16],
}