use std::fmt::Display;

use crate::{callsignmap::CallsignMap, utils::bool_slice_to_u8};

use super::callsign::Callsign;

const RAC_SECTION: [&str; 84] = [
    "AB ", "AK ", "AL ", "AR ", "AZ ", "BC ", "CO ", "CT ", "DE ", "EB ", "EMA", "ENY", "EPA",
    "EWA", "GA ", "GTA", "IA ", "ID ", "IL ", "IN ", "KS ", "KY ", "LA ", "LAX", "MAR", "MB ",
    "MDC", "ME ", "MI ", "MN ", "MO ", "MS ", "MT ", "NC ", "ND ", "NE ", "NFL", "NH ", "NL ",
    "NLI", "NM ", "NNJ", "NNY", "NT ", "NTX", "NV ", "OH ", "OK ", "ONE", "ONN", "ONS", "OR ",
    "ORG", "PAC", "PR ", "QC ", "RI ", "SB ", "SC ", "SCV", "SD ", "SDG", "SF ", "SFL", "SJV",
    "SK ", "SNJ", "STX", "SV ", "TN ", "UT ", "VA ", "VI ", "VT ", "WCF", "WI ", "WMA", "WNY",
    "WPA", "WTX", "WV ", "WWA", "WY ", "DX ",
];

#[derive(Debug, Clone)]
pub struct FieldDay {
    pub callsign1: Callsign,
    pub callsign2: Callsign,
    pub has_r: bool,
    pub transmitters: u8,
    pub class: char,
    pub rac_section: u8,
}
impl FieldDay {
    pub fn from_bits(
        message: &[bool; 77],
        transmitter_offset: u8,
        callsign_map: &CallsignMap,
    ) -> Self {
        let class = b'A' + bool_slice_to_u8::<3>(&message[61..64].try_into().unwrap());
        Self {
            callsign1: Callsign::from_bits(&message[0..28].try_into().unwrap(), callsign_map),
            callsign2: Callsign::from_bits(&message[28..56].try_into().unwrap(), callsign_map),
            has_r: message[56],
            transmitters: bool_slice_to_u8::<3>(&message[57..61].try_into().unwrap())
                - transmitter_offset,
            class: class as char,
            rac_section: bool_slice_to_u8::<7>(&message[64..71].try_into().unwrap()),
        }
    }
    pub fn callsigns(&self) -> Vec<String> {
        vec![self.callsign1.to_string(), self.callsign2.to_string()]
    }
}
impl Display for FieldDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rac_section = if self.rac_section < 84 {
            RAC_SECTION[self.rac_section as usize]
        } else {
            "DX "
        };
        write!(
            f,
            "{} {} {}{}{} {}",
            self.callsign1.to_string(),
            self.callsign2.to_string(),
            if self.has_r { "R " } else { "" },
            self.transmitters,
            self.class,
            rac_section.trim(),
        )
    }
}
