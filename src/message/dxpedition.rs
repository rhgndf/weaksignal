use std::fmt::Display;

use crate::{
    callsignmap::CallsignMap,
    utils::{bool_slice_to_u32, bool_slice_to_u8},
};

use super::{callsign::Callsign, hashedcallsign::HashedCallsign};

#[derive(Debug, Clone)]
pub struct Dxpedition {
    pub callsign1: Callsign,
    pub callsign2: Callsign,
    pub hashed_callsign: HashedCallsign<10>,
    pub signal_strength: u8,
}

impl Dxpedition {
    pub fn from_bits(message: &[bool; 77], callsign_map: &CallsignMap) -> Self {
        Self {
            callsign1: Callsign::from_bits(&message[0..28].try_into().unwrap(), callsign_map),
            callsign2: Callsign::from_bits(&message[28..56].try_into().unwrap(), callsign_map),
            hashed_callsign: HashedCallsign::from_hash(
                bool_slice_to_u32::<10>(&message[56..66].try_into().unwrap()),
                callsign_map,
            ),
            signal_strength: bool_slice_to_u8::<5>(&message[66..71].try_into().unwrap()),
        }
    }
    pub fn callsigns(&self) -> Vec<String> {
        vec![
            self.callsign1.to_string(),
            self.callsign2.to_string(),
            self.hashed_callsign.to_string(),
        ]
    }
}
impl Display for Dxpedition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.callsign1.to_string(),
            self.callsign2.to_string(),
            self.hashed_callsign.to_string(),
            (self.signal_strength as i8) * 2 - 30
        )
    }
}
