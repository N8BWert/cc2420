//!
//! Strobes (basically instructions) to send to the chip
//! to perform duties
//! 

/// Single Byte Instructions sent to the CC2420 Module
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Strobe {
    // SNOP
    // No Operation (has no other effect than reading out status-bits)
    ReadStatus = 0x00,
    // SXOSCON
    // Turn on the crystal oscillator (set XOSC16M_PD = 0 and
    // BIAS_PD = 0)
    XOSCOn = 0x01,
    // STXCAL
    // Enable and calibrate frequency synthesizer for TX;
    // Go from RX / TX to a wait state where only the synthesizer
    // is running.
    CalibrateFrequency = 0x02,
    // SRXON
    // Enable RX
    EnableRx = 0x03,
    // STXON
    // Enable TX after calibration (if not already performed)
    // Stat TX in-line encryption if SPI_SEC_MODE != 0
    EnableTx = 0x04,
    // STXONCCA
    // If CCA indicates a clear channel:
    //      Enable calibration, then TX.
    //      Start in-line encryption if SPI_SEC_MODE != 0
    // Else
    //      Do nothing
    EnableTxCCA = 0x05,
    // SRFOFF
    // Disable RX/TX and frequency synthesizer
    DisableRxTx = 0x06,
    // SXOSCOFF
    // Turn off the crystal oscillator and RF
    XOSCOff = 0x07,
    // SFLUSHRX
    // Flush the RX FIFO buffer and reset the demodulator. Always
    // read at least one byte from the RX FIFO before issuing the
    // SFLUSHRX command strobe
    FlushRx = 0x08,
    // SFLUSHTX
    // Flush the TX FIFO buffer
    FlushTx = 0x09,
    // SACK
    // Send acknowledge frame, with pending field cleared
    Ack = 0x0A,
    // SACKPEND
    // Send acknowledge frame, with pending field set
    AckPend = 0x0B,
    // SRXDEC
    // Start RX FIFO in-line decryption / authentication (as set by
    // SPI_SEC_MODE)
    RxDecryption = 0x0C,
    // STXENC
    // Start TX FIFO in-line dencryption / authentication (as set by
    // SPI_SEC_MODE), without starting TX.
    TxEncryption = 0x0D,
    // SAES
    // AES Stand alone encryption strobe. SPISEC_MODE is not
    // required to be 0, but the encryption module must be idle. If not,
    // the strobe is ignored
    AesEncryption = 0x0E,
    // Write to TX FIFO Location
    TxFifo = 0x3E,
    // Read from RX FIFO Location
    RxFifo = 0x3F,
}

impl Strobe {
    pub fn opcode(self) -> u8 {
        self as u8
    }
}