// I also dont know fully what is happening here 

// telling compiler to not go to runtime before main as well as no std libraries
#![no_std]
#![no_main]


// submoduless 
mod print;
mod vga;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("--- Welcome to Teruic OS ---");
    println!("VGA Driver initialized!");
    println!("Testing formatting: Numbers: {}, Hex: {:#x}", 42, 0xDEADBEEFu32);

    loop {}
}