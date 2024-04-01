//!
//! Error when operating the CC2420 Module
//! 

/// Error that occurs during the operation of the CC2420 Module.
pub enum RadioError<SPIE, GPIOE> {
    InvalidBufferLenth{expected: usize, found: usize},
    GpioError(GPIOE),
    SpiError(SPIE),
}