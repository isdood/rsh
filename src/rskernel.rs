// src/rskernel.rs
#![no_std]
#![no_main]

mod cpuinit;

use core::panic::PanicInfo;

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
