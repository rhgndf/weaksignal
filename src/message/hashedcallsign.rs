use std::fmt::Display;

use crate::callsignmap::CallsignMap;

#[derive(Debug, Clone)]
pub struct HashedCallsign<const SIZE: usize> {
    pub callsign: Option<String>,
    pub hashed: u32,
}

impl<const SIZE: usize> HashedCallsign<SIZE> {
    pub fn from_hash(hash: u32, map: &CallsignMap) -> Self {
        let callsign = match SIZE {
            10 => map.get_10(hash as u16),
            12 => map.get_12(hash as u16),
            22 => map.get_22(hash),
            _ => None,
        };
        Self {
            callsign: callsign.cloned(),
            hashed: hash,
        }
    }
}
impl<const SIZE: usize> Display for HashedCallsign<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(callsign) = &self.callsign {
            write!(f, "{}", callsign)
        } else {
            write!(f, "<{}>", self.hashed)
        }
    }
}
