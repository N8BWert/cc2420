//!
//! Rust Definition for the Battery Monitor Control Register
//! 

use super::Register;

use alloc::string::String;

use derive_builder::Builder;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Builder)]
#[builder(no_std, build_fn(validate = "Self::validate"))]
pub struct BatteryMonitorRegister {
    // Battery monitor comparator output, read only. BATT_OK is valid
    // 5 us after BATTMON_EN has been asserted and 
    // BATTMON_VOLTAGE has been programmed.
    #[builder(default = "true")]
    pub battmon_ok: bool,
    // Battery monitor enabled
    // 0: Battery monitor is disabled
    // 1: Battery monitor is enabled
    #[builder(default = "false")]
    pub battmon_en: bool,
    // Battery monitor toggle voltage.  The toggle voltage is given by:
    // V = 1.25V * (72 - BATTMON_VOLTAGE) / 27
    #[builder(default = "0")]
    pub battmon_voltage: u8,
}

impl Register for BatteryMonitorRegister {
    fn register_value(&self) -> u16 {
        let mut value = 0;

        if self.battmon_en {
            value |= 1 << 5;
        }

        value |= self.battmon_voltage as u16;

        value
    }

    fn address(&self) -> u8 { 0x1B }

    fn from_buffer(&mut self, buffer: [u8; 3]) {
        *self = u16::from_le_bytes(buffer[1..3].try_into().unwrap()).into();
    }
}

impl From<u16> for BatteryMonitorRegister {
    fn from(value: u16) -> Self {
        Self {
            battmon_ok: (((1 << 6) & value) != 0),
            battmon_en: (((1 << 5) & value) != 0),
            battmon_voltage: (value & 0x1F) as u8,
        }
    }
}

impl BatteryMonitorRegisterBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(battmon_voltage) = self.battmon_voltage {
            if battmon_voltage > 31 {
                return Err("Invalid BATTMON_VOLTAGE. 0<=BATTMON_VOLTAGE<=31".into());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battmon_en_value() {
        let battery_monitor = BatteryMonitorRegisterBuilder::default()
            .battmon_en(true)
            .build()
            .unwrap();

        assert_eq!(
            battery_monitor.register_value(),
            0b010_0000,
        )
    }

    #[test]
    fn test_battmon_voltage_value() {
        let battery_monitor = BatteryMonitorRegisterBuilder::default()
            .battmon_voltage(3)
            .build()
            .unwrap();

        assert_eq!(
            battery_monitor.register_value(),
            0b000_0011,
        )
    }

    #[test]
    fn test_battery_monitor_from_u16() {
        let value = 0b0_1_11111;

        let expected_battery_monitor = BatteryMonitorRegisterBuilder::default()
            .battmon_ok(false)
            .battmon_en(true)
            .battmon_voltage(0x1F)
            .build()
            .unwrap();

        assert_eq!(
            expected_battery_monitor,
            value.into(),
        )
    }
}