use std::fmt::Display;

use crate::utils::{bool_slice_to_u128, char_lookup};

#[derive(Debug, Clone)]
pub struct FreeText {
    pub text: String,
}
const FREETEXT_CHARSET: &str = " 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ+-./?";
impl FreeText {
    pub fn from_bits(message: &[bool; 77]) -> Self {
        Self {
            text: Self::to_text(&message[..71].try_into().unwrap()),
        }
    }
    pub fn to_text(message: &[bool; 71]) -> String {
        let mut message = bool_slice_to_u128(message);
        let mut text = String::new();
        for _ in 0..13 {
            let c = message % 42;
            message /= 42;
            text.push(char_lookup(c as usize, FREETEXT_CHARSET));
        }
        text.chars().rev().collect()
    }
}
impl Display for FreeText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}