//!
//! Error when operating the CC2420 Module
//! 

use alloc::string::String;

/// Error that occurs during the operation of the CC2420 Module.
pub enum RadioError<SPIE, GPIOE> {
    InvalidBufferLenth{expected: usize, found: usize},
    InvalidConfiguration(String),
    FailedConfiguration(&'static str),
    GpioError(GPIOE),
    SpiError(SPIE),
}