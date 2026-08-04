// communicates with the UART (Universal Asynchronous Receiver-Transmitter) serial chip (COM1)
// built into x86 hardware. While vga.rs writes pixels directly to a virtual screen monitor, 
//serial.rs sends text character-by-character through an I/O port (0x3F8) to the outside world.

//why we need serial port output in os??
// 1. Real-Time Logging Outside the Virtual Machine
// 2. Diagnostic Output When the Screen Fails
// 3. Copying Logs for Debugging
// 4. Automated Testing (CI/CD)

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        // 0x3F8 is the standard I/O port address for COM1
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

/// Prints to the host via the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host via the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}