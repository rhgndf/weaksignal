use std::fmt::Display;

use crate::{
    callsignmap::CallsignMap,
    utils::{bool_slice_to_u32, bool_slice_to_u8},
};

use super::{callsign::Callsign58, hashedcallsign::HashedCallsign};

#[derive(Debug, Clone)]
pub struct NonStdCall {
    pub callsign1: HashedCallsign<12>,
    pub callsign2: Callsign58,
    pub callsign_swap: bool,
    pub message_word: u8,
    pub has_cq: bool,
}
impl NonStdCall {
    pub fn from_bits(message: &[bool; 77], callsign_map: &CallsignMap) -> Self {
        Self {
            callsign1: HashedCallsign::from_hash(
                bool_slice_to_u32::<12>(&message[0..12].try_into().unwrap()),
                callsign_map,
            ),
            callsign2: Callsign58::from_bits(&message[12..70].try_into().unwrap()),
            callsign_swap: message[70],
            message_word: bool_slice_to_u8::<2>(&message[71..73].try_into().unwrap()),
            has_cq: message[73],
        }
    }
    fn message_word_to_text(r2: u8) -> &'static str {
        match r2 {
            0 => "",
            1 => "RRR",
            2 => "RR73",
            3 => "73",
            _ => "???",
        }
    }
    pub fn callsigns(&self) -> Vec<String> {
        vec![self.callsign1.to_string(), self.callsign2.to_string()]
    }
}
impl Display for NonStdCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.callsign_swap {
            write!(
                f,
                "{} {} {}",
                if self.has_cq {
                    "CQ".to_string()
                } else {
                    self.callsign2.to_string()
                },
                self.callsign1.to_string(),
                Self::message_word_to_text(self.message_word),
            )
        } else {
            write!(
                f,
                "{} {} {}",
                if self.has_cq {
                    "CQ".to_string()
                } else {
                    self.callsign1.to_string()
                },
                self.callsign2.to_string(),
                Self::message_word_to_text(self.message_word),
            )
        }
    }
}
