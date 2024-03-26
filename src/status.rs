//!
//! Status of the CC2420 Module
//! 

/// Status of the radio
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RadioStatus {
    // Indicates whether the 16 MHz oscillator is running or not
    // 0: The 16 MHz crystal oscillator is not running
    // 1: The 16 MHz crystal oscillator is running
    pub xosx_stable: bool,
    // Indicates whether an FIFO underflow has occurred during
    // transmission.  Must be cleared manually with a SFLUSHTX
    // command strobe
    // 0: No underflow has occurred
    // 1: An underflow has occurred
    pub tx_underflow: bool,
    // Indicates whether the encryption module is busy
    // 0: Encryption module is idle
    // 1: Encryption module is busy
    pub enc_busy: bool,
    // Indicates whether RF transmission is active
    // 0: RF Transmission is idle
    // 1: RF Transmission is active
    pub tx_active: bool,
    // Indicated whether the frequency synthesizer PLL is in lock or not
    // 0: The PLL is out of lock
    // 1: The PLL is in lock
    pub lock: bool,
    // Indicates whether the RSSI value is valid or not.
    // 0: The RSSI value is not valid
    // 1: The RSSI value is valid, always true when reception has been
    // enabled at least 8 symbol periods (128 us)
    pub rssi_valud: bool,
}

impl From<u8> for RadioStatus {
    fn from(value: u8) -> Self {
        Self {
            xosx_stable: (value & 1 << 6) != 0,
            tx_underflow: (value & 1 << 5) != 0,
            enc_busy: (value & 1 << 4) != 0,
            tx_active: (value & 1 << 3) != 0,
            lock: (value & 1 << 2) != 0,
            rssi_valud: (value & 1 << 1) != 0,
        }
    }
}