#![allow(dead_code, unused_assignments, unused_variables)]
// ===== Output =====
use core::fmt::{self, Write};
use super::write;

const STDOUT: usize = 1;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

// ===== Input =====

use crate::syscall::console_getchar;

pub fn getchar() -> u8 {
    console_getchar() as u8
}

pub fn read_u64() -> u64 {
    read_usize() as u64
}

pub fn read_usize() -> usize {
    let mut ans: usize = 0;
    let mut ch: u8;
    loop { ch = getchar(); if b'0' <= ch && ch <= b'9' { break; }}
    loop {
        if b'0' <= ch && ch <= b'9' {
            ans = ans * 10 + ch as usize - '0' as usize;
        } else { break; }
        ch = getchar();
    }
    return ans;
}

pub fn read_i64() -> i64 {
    let mut ans: i64 = 0;
    let mut neg: i64 = 1;
    let mut ch: u8;
    loop { ch = getchar(); if b'0' <= ch && ch <= b'9' || ch == b'-' { break; }}
    loop {
        if b'0' <= ch && ch <= b'9' {
            ans = ans * 10 + ch as i64 - '0' as i64;
        } else if ch == b'-' {
            neg = -1;
        } else {
            break;
        }
        ch = getchar();
    }
    neg * ans
}

pub fn read_f64() -> f64 {
    let mut ans: f64 = 0.0;
    let mut neg: f64 = 1.0;
    let mut dec: f64 = 0.0;
    let mut ch: u8;
    loop { ch = getchar(); if b'0' <= ch && ch <= b'9' || ch == b'-' || ch == b'.' { break; }}
    loop {
        if b'0' <= ch && ch <= b'9' {
            if dec == 0.0 {
                ans = ans * 10.0 + ch as f64 - b'0' as f64;
            } else {
                ans = ans + dec * (ch as f64 - b'0' as f64);
                dec *= 0.1;
            }
        } else if ch == b'-' {
            neg = -1.0;
        } else if ch == b'.' {
            dec = 0.1;
        } else {
            break;
        }
        ch = getchar();
    }
    neg * ans
}
