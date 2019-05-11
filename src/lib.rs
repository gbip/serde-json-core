#![allow(dead_code)]
use serde_derive::Deserialize;

use serde_json_core::de::{from_slice, Error as DError};

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum IOState {
    On,
    Off,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Pneumatic {
    pub pumps: [IOState; 2],
    pub pump_intensity: [u16; 2],
    pub valves: [IOState; 6],
}

impl Pneumatic {
    fn from_json_slice(slice: &[u8]) -> Result<Self, DError> {
        from_slice(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn io_ser() {
        let _b = Pneumatic::from_json_slice(
            "{\"pump_intensity\":[0,0],\"pumps\":[\"On\",\"On\"],\"valves\":[\"Off\",\"On\",\"On\",\"Off\",\"On\",\"Off\"]}".as_bytes()).unwrap();
        let c = Pneumatic::from_json_slice(
            "{\"tirette\":\"Triggered\",\"buzzer\":\"PlayErrorSound\"}".as_bytes(),
        );
        assert!(c.is_err());
    }

}
