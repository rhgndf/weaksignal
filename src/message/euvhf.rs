use std::fmt::Display;

use crate::{
    callsignmap::CallsignMap,
    utils::{bool_slice_to_u16, bool_slice_to_u32, bool_slice_to_u8},
};

use super::{grid::Grid6, hashedcallsign::HashedCallsign};

#[derive(Debug, Clone)]
pub struct EUVHF {
    pub callsign1: HashedCallsign<12>,
    pub callsign2: HashedCallsign<22>,
    pub has_r: bool,
    pub signal_strength: u8,
    pub serial_number: u16,
    pub grid: Grid6,
}
impl EUVHF {
    pub fn from_bits(message: &[bool; 77], callsign_map: &CallsignMap) -> Self {
        Self {
            callsign1: HashedCallsign::from_hash(
                bool_slice_to_u32::<12>(&message[0..12].try_into().unwrap()),
                callsign_map,
            ),
            callsign2: HashedCallsign::from_hash(
                bool_slice_to_u32::<22>(&message[12..34].try_into().unwrap()),
                callsign_map,
            ),
            has_r: message[34],
            signal_strength: bool_slice_to_u8::<3>(&message[35..38].try_into().unwrap()),
            serial_number: bool_slice_to_u16::<11>(&message[38..49].try_into().unwrap()),
            grid: Grid6::from_bits(&message[49..74].try_into().unwrap()),
        }
    }
    pub fn callsigns(&self) -> Vec<String> {
        vec![self.callsign1.to_string(), self.callsign2.to_string()]
    }
}
impl Display for EUVHF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}{}{:04} {}",
            self.callsign1.to_string(),
            self.callsign2.to_string(),
            if self.has_r { "R " } else { "" },
            self.signal_strength + 52,
            self.serial_number,
            self.grid.to_string(),
        )
    }
}
