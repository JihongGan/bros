#![no_std]
#![no_main]

use core::panic::PanicInfo;

// params
const NCPU: usize = 4;

#[no_mangle]
static stack0: [u8; NCPU * 4096] = [0; NCPU * 4096];

#[no_mangle]
pub extern "C" fn start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
