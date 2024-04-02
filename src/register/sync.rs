//!
//! Sync Word Register
//! 

use super::Register;
use derive_builder::Builder;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(error(validation_error = false)))]
pub struct SyncWordRegister {
    // Synchronization Word.  The SYNCWORD is processed from the
    // least significant nibble (F at reset) to the most significant
    // nibble (A at reset)
    // SYNCWORD is used both during modulation (where 0xF's are replaced)
    // with 0x0's) and during demodulation (where 0xF's are not required for
    // frame synchronisation).  In reception an implicit zero is required before
    // the first symbol acquired by SYNCWORD.
    // The rest value is compliant with IEEE 802.15.4
    #[builder(default = "0xA70F")]
    pub sync_word: u16,
}

impl Register for SyncWordRegister {
    fn register_value(&self) -> u16 {
        self.sync_word
    }

    fn address(&self) -> u8 { 0x14 }

    fn fill_from_buffer(&mut self, buffer: [u8; 3]) {
        *self = u16::from_le_bytes(buffer[1..3].try_into().unwrap()).into();
    }
}

impl From<u16> for SyncWordRegister {
    fn from(value: u16) -> Self {
        Self {
            sync_word: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_word_to_value() {
        let sync_register = SyncWordRegisterBuilder::default()
            .sync_word(0x1234)
            .build()
            .unwrap();

        assert_eq!(
            sync_register.register_value(),
            0x1234,
        )
    }

    #[test]
    fn test_sync_word_from_u16() {
        let value: u16 = 0x1234;

        let expected_sync_register = SyncWordRegisterBuilder::default()
            .sync_word(0x1234)
            .build()
            .unwrap();

        assert_eq!(
            expected_sync_register,
            value.into(),
        )
    }
}