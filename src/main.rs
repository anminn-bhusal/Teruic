//This is turuic OS..
//Because we are writing a bare-metal kernel, we disable the Rust standard library
//and bypass the standard C runtime initialization (`crt0`).


#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod allocator;
mod interrupts;
mod memory;
mod print;
mod serial;
mod shell;
mod task;
mod vga;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use task::{executor::Executor, Task};
use x86_64::VirtAddr;

entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    serial_println!("[KERNEL PANIC] {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

async fn example_task() {
    println!("-> Async Task 1 executed in background!");
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("--- Welcome to Teruic OS ---");

    // 1. Interrupts & Hardware Setup
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    // 2. Memory & Heap Setup
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap initialization failed!");

    println!("Kernel Heap Allocator initialized successfully.");

    // 3. Multitasking Executor Setup
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));

    println!("\nKernel initialization complete.\n");
    print!("teruic> ");

    // 4. Hand off execution control to the Async Executor Loop
    executor.run();
}