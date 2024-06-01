#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
mod vga_buffer;

static HELLO: &[u8] = b"Hello Sakshi, welcome to Ranjan's Kernel OS!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    //let mut writer = vga_buffer::WRITER.lock();
    //writer.write_str("HELLO AGAIN WORLD < WELCOME ").unwrap();
    //write!(writer, ", some numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello World{}", "!");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
