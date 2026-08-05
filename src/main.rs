//This is turuic OS..
//Because we are writing a bare-metal kernel, we disable the Rust standard library
//and bypass the standard C runtime initialization (`crt0`).



#![no_std] // Do not link the Rust standard library (requires operating system primitives)
#![no_main] // Disable all standard Rust main functions and entry points
#![feature(abi_x86_interrupt)] // Enable the calling convention required for x86 interrupt handlers

mod interrupts;
mod print;
mod serial;
mod shell;
mod vga;

use core::panic::PanicInfo;

/// Called whenever a kernel panic occurs.
/// Logs error details to VGA display and serial port, then halts execution.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    serial_println!("[KERNEL PANIC] {}", info);

    loop {
        // Sleep CPU on panic to prevent 100% CPU usage loop
        x86_64::instructions::hlt();
    }
}

/// Kernel entry point called directly by the bootloader (`bootloader` crate).
/// The `no_mangle` attribute ensures the compiler exports the exact symbol name `_start`.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("--- Welcome to Teruic OS ---");

    // 1. Initialize the Interrupt Descriptor Table (IDT)
    interrupts::init_idt();

    // 2. Remap and initialize the 8259 Programmable Interrupt Controllers (PICs)
    unsafe { interrupts::PICS.lock().initialize() };

    // 3. Enable hardware interrupts in CPU EFLAGS register (sti instruction)
    x86_64::instructions::interrupts::enable();

    println!("Kernel initialization complete.\n");
    print!("teruic> ");

    // 4. Main Kernel Loop
    loop {
        // Sleep the CPU until the next hardware interrupt fires.
        // Keeps host CPU usage at ~0% while idle.
        x86_64::instructions::hlt();
    }
}