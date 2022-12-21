use core::arch::asm;

use crate::NCPU;

#[link_section = ".text.entry"]
#[no_mangle]
pub unsafe extern "C" fn _start() {
    asm!(
        "la sp, stack0",
        "li a0, 1024 * {ncpu}",
        "csrr a1, mhartid",
        "addi a1, a1, 1",
        "mul a0, a0, a1",
        "add sp, sp, a0",
        "call kmain",
        ncpu = const NCPU,
    );
}
