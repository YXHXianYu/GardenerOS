#![allow(unused)]

// pub fn console_putchar(c: usize) {
//     #[allow(deprecated)]
//     sbi_rt::legacy::console_putchar(c);
// }
// 
// pub fn console_getchar() -> usize {
//     #[allow(deprecated)]
//     sbi_rt::legacy::console_getchar()
// }
// 
// pub fn shutdown() -> ! {
//     use sbi_rt::{system_reset, NoReason, Shutdown};
//     system_reset(Shutdown, NoReason);
//     panic!("It should shutdown!");
// }


use core::arch::asm;

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

// const SBI_STOP_EXTENSION: usize = 0x48534D;
// const SBI_STOP_FUNCTION: usize = 1;
const SBI_EID_SRST: usize = 0x53525354;
const SBI_SYSTEM_RESET: usize = 0;

#[inline(always)]
fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!("ecall",
             in("x10") arg0,
             in("x11") arg1,
             in("x12") arg2,
             in("x16") fid,
             in("x17") eid,
             lateout("x10") ret
        );
    }
    ret
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, 0, c, 0, 0);
}

pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0, 0)
}

pub fn shutdown_deprecated() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0, 0);
    panic!("It should shutdown! (deprecated shutdown function)");
}

pub fn shutdown() -> ! {
    sbi_call(SBI_EID_SRST, SBI_SYSTEM_RESET, 0, 0, 0);
    panic!("It should shutdown!");
}
