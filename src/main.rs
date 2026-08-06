//This is turuic OS..
//Because we are writing a bare-metal kernel, we disable the Rust standard library
//and bypass the standard C runtime initialization (`crt0`).



#![no_std] // Do not link the Rust standard library (requires operating system primitives)
#![no_main] // Disable all standard Rust main functions and entry points
#![feature(abi_x86_interrupt)] // Enable the calling convention required for x86 interrupt handlers


// Enable allocation support features
extern crate alloc;

mod allocator;
mod interrupts;
mod memory;
mod print;
mod serial;
mod shell;
mod vga;

use alloc::{boxed::Box, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

// Define kernel entry point via bootloader macro
entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    serial_println!("[KERNEL PANIC] {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("--- Welcome to Teruic OS ---");

    // 1. Interrupts & IDT setup
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    // 2. Memory & Heap Initialization
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap initialization failed!");

    println!("Kernel Heap Allocator initialized successfully!");

    // 3. Test Dynamic Memory Allocations
    let x = Box::new(42);
    let vec = vec![1, 2, 3, 4, 5];
    println!("Heap test -> Box value: {}, Vec: {:?}", x, vec);

    println!("\nKernel initialization complete.\n");
    print!("teruic> ");

    loop {
        x86_64::instructions::hlt();
    }
}