#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod lang_items;
mod uart;

#[macro_use]
mod console;

// asm
use core::arch::global_asm;
global_asm!(include_str!("entry.S"));

// params
const NCPU: usize = 4;
#[no_mangle]
static stack0: [u8; NCPU * 4096] = [0; NCPU * 4096];

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub extern "C" fn start() -> ! {
    clear_bss();
    println!("What's up BROS");
    panic!("Shutting down...")
}
