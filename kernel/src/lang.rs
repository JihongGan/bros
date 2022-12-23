use core::panic::PanicInfo;

use crate::{println, sbi::shutdown};

#[no_mangle]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(test)]
    println!("[failed]\n");

    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{}\n{}\n",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked\n{}\n", info.message().unwrap());
    }
    shutdown()
}
