#![no_std]
#![no_main]
#[macro_use]
extern crate user_lib;

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
    const P: u32 = 3;
    const STEP: usize = 1_000_000_000_000_000_000; // 1e18
    const MOD: u32 = 10007;
    println!(
        "quick_power: {} ^ {} % {} = {}",
        P, STEP, MOD,
        quick_power(P as u64, STEP as u64, MOD as u64)
    );
    0
}
