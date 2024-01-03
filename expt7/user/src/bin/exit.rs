#![no_std]
#![no_main]
#[macro_use]
extern crate user_lib;

fn f(i: i32) {
    f(i+1);
}

#[no_mangle]
fn main() -> i32 {
    f(0);
    0
}
