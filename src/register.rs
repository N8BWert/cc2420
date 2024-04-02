//!
//! Register Definitions for the CC2420 Radio
//! 

#![allow(unused)]

pub mod main_control;
use main_control::MainControlRegister;
pub use main_control::MainControlRegisterBuilder;

pub mod modem_control;
use modem_control::{ModemControlRegister0, ModemControlRegister1};
pub use modem_control::{ModemControlRegister0Builder, ModemControlRegister1Builder};

pub mod rssi;
use rssi::{RSSIRegister};
pub use rssi::RSSIRegisterBuilder;

pub mod sync;
use sync::SyncWordRegister;
pub use sync::SyncWordRegisterBuilder;

pub mod transmit_control;
use transmit_control::TransmitControlRegister;
pub use transmit_control::TransmitControlRegisterBuilder;

pub mod receive_control;
use receive_control::{ReceiveControlRegister0, ReceiveControlRegister1};
pub use receive_control::{ReceiveControlRegister0Builder, ReceiveControlRegister1Builder};

pub mod frequency_synthesis;
use frequency_synthesis::FrequencySynthesizerRegister;
pub use frequency_synthesis::FrequencySynthesizerRegisterBuilder;

pub mod security_control;
use security_control::{SecurityControlRegister0, SecurityControlRegister1};
pub use security_control::{SecurityControlRegister0Builder, SecurityControlRegister1Builder};

pub mod battery_monitor;
use battery_monitor::BatteryMonitorRegister;
pub use battery_monitor::BatteryMonitorRegisterBuilder;

pub mod io_configuration;
use io_configuration::{IOConfigurationRegister, IOConfigurationRegister1};
pub use io_configuration::{IOConfigurationRegisterBuilder, IOConfigurationRegister1Builder};

pub mod manufacturer_id;
use manufacturer_id::{LowerManufacturerID, UpperManufacturerID};
pub use manufacturer_id::{LowerManufacturerIDBuilder, UpperManufacturerIDBuilder};

pub mod fsm;
use fsm::FiniteStateMachineConstants;
pub use fsm::FiniteStateMachineConstantsBuilder;

pub mod override_registers;
use override_registers::{AndOverrideRegister, OrOverrideRegister};
pub use override_registers::{AndOverrideRegisterBuilder, OrOverrideRegisterBuilder};

pub mod agc;
use agc::AGCControlRegister;
pub use agc::AGCControlRegisterBuilder;

/// Encode the value of struct registers to their u16 representation
pub trait Register {
    /// Encode a register to a u16
    fn register_value(&self) -> u16;
    /// Get the address of a register
    fn address(&self) -> u8;
    /// Decipher a register from the buffer
    fn fill_from_buffer(&mut self, buffer: [u8; 3]);
    // Get the read address of a register
    fn read_address(&self) -> u8 { self.address() }
    // Get the write address of a register
    fn write_address(&self) -> u8 { self.address() | 1 << 6 }
    /// Get the write_value of a register
    fn write_value(&self) -> [u8; 3] {
        let register_value = self.register_value().to_le_bytes();
        [self.write_address(), register_value[0], register_value[1]]
    }
}