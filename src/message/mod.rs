use std::{fmt::Display, collections::HashMap};

use crate::callsignmap::CallsignMap;

use self::messagedata::MessageData;

mod callsign;
mod dxpedition;
mod euvhf;
mod fieldday;
mod freetext;
mod grid;
mod hashedcallsign;
mod messagedata;
mod nonstdcall;
mod rttyru;
mod standard;
mod telemetry;
mod unknown;

#[derive(Debug, Clone)]
pub struct Message {
    snr: f32,
    pub freq_bin_hz: u32,
    pub time_offset_ms: i64,
    pub data: MessageData,
}

impl Message {
    pub fn from_bits(
        snr: f32,
        freq_bin_hz: u32,
        time_offset_ms: i64,
        message: &[bool; 77],
        callsign_map: &CallsignMap,
    ) -> Message {
        Message {
            snr,
            freq_bin_hz,
            time_offset_ms,
            data: MessageData::from_bits(message, callsign_map),
        }
    }
    pub fn callsigns(&self) -> Vec<String> {
        self.data.callsigns()
    }

    pub fn deduplicate_signals(messages: Vec<Message>) -> Vec<Message> {
        let mut map = HashMap::new();
        messages.iter().for_each(|message| {
            let key = message.data.to_string();
            map.entry(key).and_modify(|e: &mut &Message| {
                if e.snr < message.snr {
                    *e = message;
                }
            }).or_insert(message);
        });
        map.into_iter().map(|(_, v)| v.clone()).collect()
    }

}
impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:5.1} {:.1} {:4} {}",
            self.snr,
            self.time_offset_ms as f32 / 1000.0,
            self.freq_bin_hz,
            self.data.to_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::message::{
        callsign::Callsign58,
        grid::{Grid4, Grid6}, freetext::FreeText,
    };

    use super::{callsign::Callsign, *};

    #[test]
    fn callsign28() {
        let callsign = Callsign::from_u32(10214965, None, &CallsignMap::new());
        assert_eq!(callsign.to_string(), "K1ABC");
    }

    #[test]
    fn callsign58_pj4k1abc() {
        let callsign = Callsign58::from_u64(166563865821947300);
        assert_eq!(callsign.to_string(), "PJ4/K1ABC");
    }

    #[test]
    fn callsign58_yw18fifa() {
        let callsign = Callsign58::from_u64(225199321060198248);
        assert_eq!(callsign.to_string(), "YW18FIFA");
    }

    #[test]
    fn grid4_fn20() {
        let grid = Grid4::from_u16(10320, false);
        assert_eq!(grid.to_string(), "FN20");
    }

    #[test]
    fn grid4_minus11() {
        let grid = Grid4::from_u16(32424, false);
        assert_eq!(grid.to_string(), "-11");
    }

    #[test]
    fn grid4_plus02() {
        let grid = Grid4::from_u16(32437, false);
        assert_eq!(grid.to_string(), "+02");
    }

    #[test]
    fn grid4_rrr() {
        let grid = Grid4::from_u16(32402, false);
        assert_eq!(grid.to_string(), "RRR");
    }

    #[test]
    fn grid4_73() {
        let grid = Grid4::from_u16(32404, false);
        assert_eq!(grid.to_string(), "73");
    }

    #[test]
    fn grid4_empty() {
        let grid = Grid4::from_u16(32401, false);
        assert_eq!(grid.to_string(), "");
    }

    #[test]
    fn grid6() {
        let grid = Grid6::from_u32(9153543);
        assert_eq!(grid.to_string(), "IO91NP");
    }

    #[test]
    fn freetext_message() {
        let f71: Vec<i32> = vec![
            0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0,
            0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1,
            1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut f71_bool = f71.iter().map(|&x| x == 1).collect::<Vec<bool>>();
        f71_bool.extend(vec![false; 6]);
        let freetext = FreeText::from_bits(f71_bool.as_slice().try_into().unwrap());
        assert_eq!(freetext.to_string(), "TNX BOB 73 GL");
    }

    #[test]
    fn test_stuff() {}
}
