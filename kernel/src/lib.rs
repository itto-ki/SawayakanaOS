#![feature(panic_implementation)]
#![no_std]


mod drivers;

use drivers::vga::VGA_BUFFER;
use core::panic::PanicInfo;

#[panic_implementation]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn kernel_main() -> ! {

    unsafe {
        VGA_BUFFER.write_str("SawayakanaAsa");
        VGA_BUFFER.flush();
    }

    loop {}
}
