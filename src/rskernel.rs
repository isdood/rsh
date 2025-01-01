// src/rskernel.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use crate::cpuinit;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize the CPU
    cpuinit::init();

    loop {
        // Main kernel loop
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}