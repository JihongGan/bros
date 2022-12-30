#![no_std]
#![no_main]

use user::println;

#[no_mangle]
fn main() -> i32 {
    println!("What's up, UMode?");
    0
}
