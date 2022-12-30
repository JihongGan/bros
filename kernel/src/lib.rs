#![no_std]
#![no_main]
#![feature(panic_info_message, asm_const, custom_test_frameworks)]
#![test_runner(crate::run_tests)]
#![reexport_test_harness_main = "tmain"]

mod lang;
mod sbi;

#[macro_use]
mod console;

use core::arch::global_asm;

use crate::sbi::shutdown;

global_asm!(include_str!("entry.S"));

// params
const NCPU: usize = 1;
#[no_mangle]
static stack0: [u8; NCPU * 4096] = [0; NCPU * 4096];

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    clear_bss();

    println!("\n\n\nWhat's up, BROS?\n");

    #[cfg(test)]
    tmain();

    shutdown()
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

// test infra
#[cfg(test)]
fn run_tests(tests: &[&dyn Testable]) {
    use crate::sbi::shutdown;

    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    println!("{} tests passed!\n", tests.len());
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("{}...\t", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}
