#![no_std]
#![no_main]

mod lang_items;

// asm
use core::arch::global_asm;
global_asm!(include_str!("entry.S"));

// params
const NCPU: usize = 4;
#[no_mangle]
static stack0: [u8; NCPU * 4096] = [0; NCPU * 4096];

static SUP: &[u8] = b"Wassup BROS";

#[no_mangle]
pub extern "C" fn start() -> ! {
    loop {}
}
