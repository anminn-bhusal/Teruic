//This is turuic OS..
//Because we are writing a bare-metal kernel, we disable the Rust standard library
//and bypass the standard C runtime initialization (`crt0`).

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod editor;
mod allocator;
mod c_abi;
mod gui;
mod interrupts;
mod memory;
mod loader;
mod print;
mod serial;
mod shell;
mod c_runner;
mod task;
mod vfs;
mod vga;
mod java;

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

async fn async_background_task() {
    serial_println!("Async Background Task Running");
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // 1. Clear VGA Buffer First
    vga::clear_screen();

    // 2. Setup Memory & Heap First (Required for print/formatting)
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap initialization failed!");

    // 3. Initialize VFS
    vfs::VFS.lock().init();

    // Pre-load a valid x86_64 binary into VFS for testing (after Heap and VFS init)
    let test_bin: &[u8] = &[0x90, 0x90, 0xC3]; // NOP, NOP, RET
    crate::vfs::VFS.lock().write_file("test.bin", test_bin.to_vec());

    // 4. Render GUI Status Bar at Top Row
    gui::UI::draw_header("Bare-Metal Core Active");

    // 5. Print Welcome Banner to Screen
    println!("\n--- Teruic OS v0.1.0 ---");
    println!("Kernel Heap: ACTIVE");
    println!("Virtual File System: ACTIVE");
    println!("Type 'help' for available commands.\n");
    print!("teruic> ");

    // 6. Setup Interrupts & PICs safely
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    // 7. Start Async Executor Loop
    let mut executor = Executor::new();
    executor.spawn(Task::new(async_background_task()));
    executor.run();
}