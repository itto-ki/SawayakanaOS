#![no_std]

mod drivers;
#[macro_use]
mod printk;

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn kernel_main(magic: u64, addr: u64) -> ! {

    printk!("Sawayakana{}", "Asa");
    printk!("Hello {}", "World");
    printk!("{} + {} = {}", 1, 2, 3);
    printk!("Magic = 0x{:x}", magic);
    printk!("Addr  = 0x{:x}", addr);
    loop {}
}
