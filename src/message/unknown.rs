
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Unknown {
    pub bits: [bool; 77],
}
impl Unknown {
    pub fn from_bits(message: &[bool; 77]) -> Self {
        Self { bits: *message }
    }
}
impl Display for Unknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        for bit in self.bits.iter() {
            if *bit {
                text.push('1');
            } else {
                text.push('0');
            }
        }
        write!(f, "{}", text)
    }
}
