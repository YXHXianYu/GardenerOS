#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use core::arch::global_asm;

extern crate alloc;

#[macro_use]
extern crate bitflags;

#[macro_use]
mod console;

mod lang_items;
mod sbi;
mod syscall;
mod trap;
mod loader;
mod config;
mod task;
mod timer;
mod mm;
mod sync;


/* stack memory settings */
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/* clear bss segment */
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(
            sbss as usize as *mut u8,
            ebss as usize - sbss as usize,
        ).fill(0);
    }
}

// This main is for system with allocation
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    mm::init();
    println!("[kernel] back to world!");
    mm::remap_test();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}

// This main is preemptive system
#[no_mangle]
pub fn rust_main_expt5() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    trap::init();
    println!("[kernel] Trap initialized.");
    loader::load_apps();
    println!("[kernel] Applications loaded.");
    trap::enable_timer_interrupt();
    println!("[kernel] Enabled timer interrupt.");
    timer::set_next_trigger();
    println!("[kernel] Have set next trigger.");
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}

// This main is multi-tasks system
#[no_mangle]
pub fn rust_main_expt4() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    trap::init();
    loader::load_apps();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}

#[no_mangle]
pub fn rust_main_expt3() -> ! {
    clear_bss();
    println!("[Kernel] Hello, world!");
    trap::init();
    // batch::init();
    // batch::run_next_app();
    panic!("Unreachable in rust_main!");
}

#[no_mangle]
pub fn rust_main_expt2() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    clear_bss();
    println!("Hello, world!"); println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    println!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    println!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    println!("Hello, world!");
    panic!("Shutdown machine!");
}

// /* system call */
// use core::arch::asm;
// 
// const SYSCALL_EXIT: usize = 93;
// const SYSCALL_WRITE: usize = 64;
// 
// fn syscall(id: usize, args: [usize; 3]) -> isize {
//     let mut ret: isize;
//     unsafe {
//         asm!("ecall",
//              in("x10") args[0],
//              in("x11") args[1],
//              in("x12") args[2],
//              in("x17") id,
//              lateout("x10") ret
//         );
//     }
//     ret
// }
// 
// /* system exit */
// pub fn sys_exit(xstate: i32) -> isize {
//     syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
// }
// 
// /* system write */
// pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
//     syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
// }

// /* Standard Output */
// struct Stdout;
// 
// impl Write for Stdout {
//     fn write_str(&mut self, s: &str) -> fmt::Result {
//         sys_write(1, s.as_bytes());
//         Ok(())
//     }
// }
// 
// pub fn print(args: fmt::Arguments) {
//     Stdout.write_fmt(args).unwrap();
// }
// 
// /* Standard Output Macro */
// use core::fmt::{self, Write};
// 
// #[macro_export]
// macro_rules! print {
//     ($fmt: literal $(, $($arg: tt)+)?) => {
//         $crate::console::print(format_args!($fmt $(, $($arg)+)?));
//     }
// }
// 
// #[macro_export]
// macro_rules! println {
//     ($fmt: literal $(, $($arg: tt)+)?) => {
//         print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
//     }
// }

/* SBI Standard Output */
// use crate::sbi::console_putchar;
// use crate::println;
