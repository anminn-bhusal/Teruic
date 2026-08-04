// I also dont know fully what is happening here 

// telling compiler to not go to runtime before main as well as no std libraries
#![no_std]
#![no_main]


//submoduless
mod print;
mod serial;
mod vga;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    serial_println!("[KERNEL PANIC] {}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("--- Welcome to Teruic OS ---");
    serial_println!("--- Teruic OS Serial Console Active ---");

    println!("VGA Output: Ready");
    serial_println!("Serial Log: Initialization successful.");

    loop {}
}