// When the CPU encounters an error (like dividing by zero or accessing invalid memory),
//  or when hardware signals an event (like a keypress), it pauses execution 
// and looks up a handler function in a 
// central table called the Interrupt Descriptor Table (IDT). 

// so we are creating this file for interrupt handling

use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // Register handler for Breakpoint Exception (used for debugging)
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        // Register handler for Double Fault Exception (prevents triple faults)
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

/// Loads the IDT into the CPU register
pub fn init_idt() {
    IDT.load();
}

/// Handler for Breakpoint Exception (INT3)
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("\n[EXCEPTION: BREAKPOINT]\n{:#?}", stack_frame);
}

/// Handler for Double Fault Exception
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("[EXCEPTION: DOUBLE FAULT]\n{:#?}", stack_frame);
}