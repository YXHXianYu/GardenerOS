#![no_std]
#![no_main]
#[macro_use]
extern crate user_lib;
use crate::user_lib::console::read_u64;

fn quick_power(base: u64, expo: u64, modu: u64) -> u64 {
    let mut a: u64 = base;
    let mut n: u64 = expo;
    let mut ans: u64 = 1;
    while n >= 1 {
        if n & 1 != 0 { ans = ans * a % modu; }
        a = a * a % modu;
        n >>= 1;
    }
    ans
}

#[no_mangle]
fn main() -> i32 {
    print!("base    : ");
    let base: u64 = read_u64();
    print!("{}\n", base);

    print!("exponent: ");
    let expo: u64 = read_u64();
    print!("{}\n", expo);

    print!("modulus : ");
    let modu: u64 = read_u64();
    print!("{}\n", modu);

    println!(
        "quick_power: {} ^ {} % {} = {}",
        base, expo, modu,
        quick_power(base, expo, modu)
    );
    0
}
