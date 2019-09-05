#![no_std]

mod drivers;
#[macro_use]
mod printk;
mod gdt;

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn kernel_main(magic: u64, addr: u64) -> ! {

    // printk!("Sawayakana{}", "Asa");
    // printk!("Hello {}", "World");
    // printk!("{} + {} = {}", 1, 2, 3);
    // printk!("Magic = 0x{:x}", magic);
    // printk!("Addr  = 0x{:x}", addr);
    printk!("#############################################################################");
    printk!(" ____                                  _                      ___  ____  ");
    printk!("/ ___|  __ ___      ____ _ _   _  __ _| | ____ _ _ __   __ _ / _ \\/ ___| ");
    printk!("\\___ \\ / _` \\ \\ /\\ / / _` | | | |/ _` | |/ / _` | '_ \\ / _` | | | \\___ \\ ");
    printk!(" ___) | (_| |\\ V  V / (_| | |_| | (_| |   < (_| | | | | (_| | |_| |___) |");
    printk!("|____/ \\__,_| \\_/\\_/ \\__,_|\\__, |\\__,_|_|\\_\\__,_|_| |_|\\__,_|\\___/|____/ ");
    printk!("                           |___/                                         ");
    printk!("#############################################################################");
    loop {}
}
