use std::fmt::Display;

use crate::callsignmap::CallsignMap;

use super::{callsign::Callsign, grid::Grid4};

#[derive(Debug, Clone)]
pub struct Standard<const SUFFIX: char> {
    pub callsign1: Callsign,
    pub callsign2: Callsign,
    pub grid: Grid4,
}
impl<const SUFFIX: char> Standard<SUFFIX> {
    pub fn from_bits(message: &[bool; 77], callsign_map: &CallsignMap) -> Self {
        let callsign1_suffix = if message[28] { Some(SUFFIX) } else { None };
        let callsign2_suffix = if message[57] { Some(SUFFIX) } else { None };
        let has_r = message[58];
        Self {
            callsign1: Callsign::from_bits_with_suffix(
                &message[0..28].try_into().unwrap(),
                callsign1_suffix,
                callsign_map,
            ),
            callsign2: Callsign::from_bits_with_suffix(
                &message[29..57].try_into().unwrap(),
                callsign2_suffix,
                callsign_map,
            ),
            grid: Grid4::from_bits_with_r(&message[59..74].try_into().unwrap(), has_r),
        }
    }
    pub fn callsigns(&self) -> Vec<String> {
        vec![self.callsign1.to_string(), self.callsign2.to_string()]
    }
}
impl<const SUFFIX: char> Display for Standard<SUFFIX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.callsign1.to_string(),
            self.callsign2.to_string(),
            self.grid.to_string(),
        )
    }
}
