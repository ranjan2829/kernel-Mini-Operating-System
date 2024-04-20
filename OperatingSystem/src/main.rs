#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
