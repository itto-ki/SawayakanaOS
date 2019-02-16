#![no_std]

mod drivers;
#[macro_use]
mod printk;

use drivers::vga::VGA_BUFFER;
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn kernel_main() -> ! {

    printk!("Sawayakana{}", "Asa");
    printk!("Hello {}", "World");
    printk!("{} + {} = {}", 1, 2, 3);
    loop {}
}
