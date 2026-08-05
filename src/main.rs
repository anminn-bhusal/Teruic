// I also dont know fully what is happening here 

// telling compiler to not go to runtime before main as well as no std libraries
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)] // <-- Needed for interrupt handlers


//submoduless
mod interrupts;
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

    // Initialize Interrupt Descriptor Table
    interrupts::init_idt();
    println!("IDT Initialized successfully!");

    // Trigger a test breakpoint exception
    x86_64::instructions::interrupts::int3();

    println!("Execution continued successfully after breakpoint exception!");

    loop {}
}