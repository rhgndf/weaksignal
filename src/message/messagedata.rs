use std::fmt::Display;

use crate::{callsignmap::CallsignMap, utils::bool_slice_to_u8};

use super::{
    dxpedition::Dxpedition, euvhf::EUVHF, fieldday::FieldDay, freetext::FreeText,
    nonstdcall::NonStdCall, rttyru::RTTYRU, standard::Standard, telemetry::Telemetry,
    unknown::Unknown,
};

#[derive(Debug, Clone)]
pub enum MessageData {
    FreeText(FreeText),
    Dxpedition(Dxpedition),
    FieldDay(FieldDay),
    Telemetry(Telemetry),
    StandardR(Standard<'R'>),
    StandardP(Standard<'P'>),
    RTTYRU(RTTYRU),
    NonStdCall(NonStdCall),
    EUVHF(EUVHF),
    Unknown(Unknown),
}

impl MessageData {
    pub fn from_bits(message: &[bool; 77], callsign_map: &CallsignMap) -> MessageData {
        let i3 = bool_slice_to_u8::<3>(&message[74..77].try_into().unwrap());
        match i3 {
            0 => {
                let n3 = bool_slice_to_u8::<3>(&message[71..74].try_into().unwrap());
                match n3 {
                    0 => MessageData::FreeText(FreeText::from_bits(message)),
                    1 => MessageData::Dxpedition(Dxpedition::from_bits(message, callsign_map)),
                    3 => MessageData::FieldDay(FieldDay::from_bits(message, 1, callsign_map)),
                    4 => MessageData::FieldDay(FieldDay::from_bits(message, 17, callsign_map)),
                    5 => MessageData::Telemetry(Telemetry::from_bits(message)),
                    _ => MessageData::Unknown(Unknown::from_bits(message)),
                }
            }
            1 => MessageData::StandardR(Standard::from_bits(message, callsign_map)),
            2 => MessageData::StandardP(Standard::from_bits(message, callsign_map)),
            3 => MessageData::RTTYRU(RTTYRU::from_bits(message, callsign_map)),
            4 => MessageData::NonStdCall(NonStdCall::from_bits(message, callsign_map)),
            5 => MessageData::EUVHF(EUVHF::from_bits(message, callsign_map)),
            _ => MessageData::Unknown(Unknown::from_bits(message)),
        }
    }
    pub fn callsigns(&self) -> Vec<String> {
        match self {
            MessageData::Dxpedition(m) => m.callsigns(),
            MessageData::FieldDay(m) => m.callsigns(),
            MessageData::StandardR(m) => m.callsigns(),
            MessageData::StandardP(m) => m.callsigns(),
            MessageData::RTTYRU(m) => m.callsigns(),
            MessageData::NonStdCall(m) => m.callsigns(),
            MessageData::EUVHF(m) => m.callsigns(),
            _ => vec![],
        }
    }
    pub fn message_type_as_string(&self) -> &str {
        match self {
            MessageData::FreeText(_) => "FreeText",
            MessageData::Dxpedition(_) => "Dxpedition",
            MessageData::FieldDay(_) => "FieldDay",
            MessageData::Telemetry(_) => "Telemetry",
            MessageData::StandardR(_) => "StandardR",
            MessageData::StandardP(_) => "StandardP",
            MessageData::RTTYRU(_) => "RTTYRU",
            MessageData::NonStdCall(_) => "NonStdCall",
            MessageData::EUVHF(_) => "EUVHF",
            MessageData::Unknown(_) => "Unknown",
        }
    }
}
impl Display for MessageData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageData::FreeText(m) => write!(f, "{}", m.to_string()),
            MessageData::Dxpedition(m) => write!(f, "{}", m.to_string()),
            MessageData::FieldDay(m) => write!(f, "{}", m.to_string()),
            MessageData::Telemetry(m) => write!(f, "{}", m.to_string()),
            MessageData::StandardR(m) => write!(f, "{}", m.to_string()),
            MessageData::StandardP(m) => write!(f, "{}", m.to_string()),
            MessageData::RTTYRU(m) => write!(f, "{}", m.to_string()),
            MessageData::NonStdCall(m) => write!(f, "{}", m.to_string()),
            MessageData::EUVHF(m) => write!(f, "{}", m.to_string()),
            MessageData::Unknown(m) => write!(f, "{}", m.to_string()),
        }
    }
}

const STATES_AND_PROVINCES: [&str; 65] = [
    "AL ", "AK ", "AZ ", "AR ", "CA ", "CO ", "CT ", "DE ", "FL ", "GA ", "HI ", "ID ", "IL ",
    "IN ", "IA ", "KS ", "KY ", "LA ", "ME ", "MD ", "MA ", "MI ", "MN ", "MS ", "MO ", "MT ",
    "NE ", "NV ", "NH ", "NJ ", "NM ", "NY ", "NC ", "ND ", "OH ", "OK ", "OR ", "PA ", "RI ",
    "SC ", "SD ", "TN ", "TX ", "UT ", "VT ", "VA ", "WA ", "WV ", "WI ", "WY ", "NB ", "NS ",
    "QC ", "ON ", "MB ", "SK ", "AB ", "BC ", "NWT", "NF ", "LB ", "NU ", "YT ", "PEI", "DC ",
];

#[derive(Debug, Clone)]
pub enum S13 {
    SerialNumber(u16),
    StatesAndProvinces(u8),
}
impl S13 {
    pub fn new(n: u16) -> Self {
        if n < 8001 {
            Self::SerialNumber(n)
        } else {
            Self::StatesAndProvinces((n - 8001) as u8)
        }
    }
}
impl Display for S13 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerialNumber(n) => write!(f, "{:04}", n),
            Self::StatesAndProvinces(n) => {
                if *n < 65 {
                    write!(f, "{}", STATES_AND_PROVINCES[*n as usize].to_owned())
                } else {
                    write!(f, "")
                }
            }
        }
    }
}
