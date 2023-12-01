use std::fmt::Display;

use crate::{
    callsignmap::CallsignMap,
    utils::{bool_slice_to_u32, bool_slice_to_u64, char_lookup},
};

use super::hashedcallsign::HashedCallsign;

#[derive(Debug, Clone)]
pub struct Callsign28 {
    pub c28: u32,
    pub suffix: Option<char>,
}

#[derive(Debug, Clone)]
pub enum Callsign {
    C28(Callsign28),
    H22(HashedCallsign<22>),
}

const ALPHANUMERIC: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHANUMERIC_SPACE: &str = " 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHA_SPACE: &str = " ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMERIC: &str = "0123456789";

impl Callsign {
    pub fn from_bits(message: &[bool; 28], callsign_map: &CallsignMap) -> Self {
        let num = bool_slice_to_u32(message);
        Self::from_u32(num, None, callsign_map)
    }
    pub fn from_bits_with_suffix(
        message: &[bool; 28],
        suffix: Option<char>,
        callsign_map: &CallsignMap,
    ) -> Self {
        let num = bool_slice_to_u32(message);
        Self::from_u32(num, suffix, callsign_map)
    }
    pub fn from_u32(n: u32, suffix: Option<char>, callsign_map: &CallsignMap) -> Self {
        if n >= 2063592 && n < 6257896 {
            Self::H22(HashedCallsign::from_hash(n - 2063592, callsign_map))
        } else {
            Self::C28(Callsign28::from_u32(n, suffix))
        }
    }
}
impl Display for Callsign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Callsign::C28(n) => write!(f, "{}", n),
            Callsign::H22(h) => write!(f, "{}", h),
        }
    }
}

impl Callsign28 {
    pub fn from_u32(n: u32, suffix: Option<char>) -> Self {
        Self { c28: n, suffix }
    }
    pub fn to_standard_call(n: u32) -> String {
        let mut n = n;
        let mut call = String::with_capacity(6);
        call.push(char_lookup((n % 27) as usize, ALPHA_SPACE));
        n /= 27;
        call.push(char_lookup((n % 27) as usize, ALPHA_SPACE));
        n /= 27;
        call.push(char_lookup((n % 27) as usize, ALPHA_SPACE));
        n /= 27;
        call.push(char_lookup((n % 10) as usize, NUMERIC));
        n /= 10;
        call.push(char_lookup((n % 36) as usize, ALPHANUMERIC));
        n /= 36;
        call.push(char_lookup((n % 37) as usize, ALPHANUMERIC_SPACE));
        call.trim().chars().rev().collect()
    }
    pub fn num_to_str(n: u32) -> String {
        let mut s = String::new();
        let mut n = n;
        while n > 0 {
            s.push(char_lookup((n % 27) as usize, ALPHA_SPACE));
            n /= 27;
        }
        s.trim().chars().rev().collect()
    }
    pub fn to_call(n: u32) -> String {
        if n == 0 {
            return String::from("DE");
        }
        if n == 1 {
            return String::from("QRZ");
        }
        if n == 2 {
            return String::from("CQ");
        }
        if n < 1004 {
            return format!("CQ {:03}", n - 3);
        }
        if n < 2063592 {
            return format!("CQ {}", Self::num_to_str(n - 1003));
        }
        if n < 6257896 {
            return format!("<{}>", n - 2063592);
        }
        Self::to_standard_call(n - 6257896)
    }
}

impl Display for Callsign28 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(suffix) = self.suffix {
            write!(f, "{}/{}", Callsign28::to_call(self.c28), suffix)
        } else {
            write!(f, "{}", Callsign28::to_call(self.c28))
        }
    }
}

const NONSTD_CHARSET: &str = " 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ/";

#[derive(Debug, Clone)]
pub struct Callsign58 {
    pub c58: u64,
}
impl Callsign58 {
    pub fn from_bits(message: &[bool; 58]) -> Self {
        Self::from_u64(bool_slice_to_u64(message))
    }
    pub fn from_u64(n: u64) -> Self {
        Self { c58: n }
    }
    pub fn to_call(&self) -> String {
        let mut c58 = self.c58;
        let mut text = String::new();
        for _ in 0..11 {
            let c = c58 % 38;
            c58 /= 38;
            text.push(char_lookup(c as usize, NONSTD_CHARSET));
        }
        text.trim().chars().rev().collect()
    }
}
impl Display for Callsign58 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_call())
    }
}
