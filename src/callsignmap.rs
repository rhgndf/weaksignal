use std::{collections::{HashMap, hash_map}, num::Wrapping};

use static_init::dynamic;

#[derive(Clone, Debug, Default)]
pub struct CallsignMap {
    pub map: HashMap<u32, String>,
    pub map12: HashMap<u16, u32>,
}

fn compute_char_map() -> [u8; 256] {
    let mut char_map = [0; 256];
    let table = " 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ/";
    for i in 0..table.len() {
        char_map[table.as_bytes()[i] as usize] = i as u8;
    }
    char_map
}

#[dynamic]
pub static CHAR_MAP: [u8; 256] = compute_char_map();

impl CallsignMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            map12: HashMap::new(),
        }
    }
    pub(crate) fn hash(callsign: &String) -> u32 {
        // right justify to 11
        let callsign = format!("{:<11}", callsign);
        let hash = callsign
            .chars()
            .fold(0, |acc, c| acc * 38 + CHAR_MAP[c as usize] as u64);
        ((Wrapping(hash) * Wrapping(47055833459)).0 >> (64 - 22)) as u32
    }
    pub fn insert(&mut self, callsign: &String) -> u32 {
        let hash = Self::hash(&callsign);
        self.map.insert(hash as u32, callsign.clone());
        self.map12.insert((hash >> 10) as u16, hash as u32);
        hash
    }
    pub fn remove(&mut self, callsign: &String) {
        let hash = Self::hash(&callsign);
        self.map.remove(&hash);
        self.map12.remove(&((hash >> 10) as u16));
    }
    pub fn get_22(&self, hash: u32) -> Option<&String> {
        self.map.get(&hash)
    }
    pub fn get_12(&self, hash: u16) -> Option<&String> {
        self.map12.get(&hash).and_then(|&hash| self.map.get(&hash))
    }
    pub fn get_10(&self, hash: u16) -> Option<&String> {
        (0..4)
            .map(|i| (hash << 2) + i)
            .find_map(|hash| self.get_12(hash))
    }
    pub fn iter(&self) -> hash_map::Iter<'_, u32, String> {
        self.map.iter()
    }
}
