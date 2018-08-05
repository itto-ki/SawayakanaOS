#![feature(panic_implementation)]
#![no_std]

use core::panic::PanicInfo;

#[panic_implementation]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn kernel_main() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    let hello: &[u8] = b"SawayakanaAsa";
    for (i, &byte) in hello.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0b;
        }
    }

    loop {}
}
