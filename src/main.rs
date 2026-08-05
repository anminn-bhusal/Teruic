// I also dont know fully what is happening here 

// telling compiler to not go to runtime before main as well as no std libraries
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)] // Needed for interrupt handlers

mod interrupts;
mod print;
mod shell;
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
    
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    println!("Kernel initialization complete.\n");
    print!("teruic> ");

    loop {
        // Prevent CPU from running 100% hot while idle
        x86_64::instructions::hlt();
    }
}