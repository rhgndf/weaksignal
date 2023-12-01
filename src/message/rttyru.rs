use std::fmt::Display;

use crate::{utils::{bool_slice_to_u16, bool_slice_to_u8}, callsignmap::CallsignMap};

use super::{callsign::Callsign, messagedata::S13};

#[derive(Debug, Clone)]
pub struct RTTYRU {
    pub has_tu: bool,
    pub callsign1: Callsign,
    pub callsign2: Callsign,
    pub has_r: bool,
    pub signal_strength: u8,
    pub s13: S13,
}
impl RTTYRU {
    pub fn from_bits(message: &[bool; 77], callsign_map: &CallsignMap) -> Self {
        Self {
            has_tu: message[0],
            callsign1: Callsign::from_bits(&message[1..29].try_into().unwrap(), callsign_map),
            callsign2: Callsign::from_bits(&message[29..57].try_into().unwrap(), callsign_map),
            has_r: message[57],
            signal_strength: bool_slice_to_u8::<3>(&message[58..61].try_into().unwrap()),
            s13: S13::new(bool_slice_to_u16::<13>(
                &message[61..74].try_into().unwrap(),
            )),
        }
    }
    pub fn callsigns(&self) -> Vec<String> {
        vec![self.callsign1.to_string(), self.callsign2.to_string()]
    }
}
impl Display for RTTYRU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} {} {} {}",
            if self.has_tu { "TU " } else { "" },
            self.callsign1.to_string(),
            self.callsign2.to_string(),
            (self.signal_strength as u16) * 10 + 529,
            self.s13.to_string(),
        )
    }
}
