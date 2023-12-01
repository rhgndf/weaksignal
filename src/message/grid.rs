use std::fmt::Display;

use crate::utils::{bool_slice_to_u16, bool_slice_to_u32};

#[derive(Debug, Clone)]
pub struct Grid4 {
    pub has_r: bool,
    pub grid: u16,
}
impl Grid4 {
    pub fn from_bits_with_r(message: &[bool; 15], has_r: bool) -> Self {
        Self::from_u16(bool_slice_to_u16(message), has_r)
    }
    pub fn from_u16(n: u16, has_r: bool) -> Self {
        Self {
            grid: n,
            has_r: has_r,
        }
    }
    fn to_grid(n: u16) -> String {
        let mut n = n;
        let mut grid = String::new();
        if n <= 32400 {
            //'0' + n % 10
            grid.push((b'0' + (n % 10) as u8) as char);
            n /= 10;
            grid.push((b'0' + (n % 10) as u8) as char);
            n /= 10;
            grid.push((b'A' + (n % 18) as u8) as char);
            n /= 18;
            grid.push((b'A' + (n % 18) as u8) as char);
            return grid.chars().rev().collect();
        }
        n -= 32400;
        if n == 1 {
            return String::new();
        }
        if n == 2 {
            return String::from("RRR");
        }
        if n == 3 {
            return String::from("RR73");
        }
        if n == 4 {
            return String::from("73");
        }
        if n < 35 {
            return format!("-{:02}", 35 - n);
        }
        return format!("+{:02}", n - 35);
    }
}
impl Display for Grid4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_r {
            write!(f, "R{}", Grid4::to_grid(self.grid))
        } else {
            write!(f, "{}", Grid4::to_grid(self.grid))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid6 {
    pub grid: u32,
}
impl Grid6 {
    pub fn from_bits(message: &[bool; 25]) -> Self {
        Self::from_u32(bool_slice_to_u32(message))
    }
    pub fn from_u32(n: u32) -> Self {
        Self { grid: n }
    }
    fn to_grid(n: u32) -> String {
        let mut n = n;
        let mut grid = String::new();
        grid.push((b'A' + (n % 24) as u8) as char);
        n /= 24;
        grid.push((b'A' + (n % 24) as u8) as char);
        n /= 24;
        grid.push((b'0' + (n % 10) as u8) as char);
        n /= 10;
        grid.push((b'0' + (n % 10) as u8) as char);
        n /= 10;
        grid.push((b'A' + (n % 18) as u8) as char);
        n /= 18;
        grid.push((b'A' + (n % 18) as u8) as char);
        return grid.chars().rev().collect();
    }
}
impl Display for Grid6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Grid6::to_grid(self.grid))
    }
}
