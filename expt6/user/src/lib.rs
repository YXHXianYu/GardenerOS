#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod syscall;
mod lang_items;

/* == sys write == */
use syscall::console_getchar;
use syscall::sys_write;
use syscall::sys_exit;
use syscall::sys_yield;
use syscall::sys_get_time;

pub fn write(fd: usize, buf: &[u8]) -> usize {
    sys_write(fd, buf)
}

pub fn exit(exit_code: i32) -> usize {
    sys_exit(exit_code)
}

pub fn yield_() -> usize {
    sys_yield()
}

pub fn getchar() -> u8 {
    console_getchar() as u8
}

pub fn get_time() -> isize {
    sys_get_time() as isize
}

/* == main == */
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}
