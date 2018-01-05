use core::fmt::*;
use kernel::hil::uart::{self, UART};
use kernel::process;
use mk66::{self, gpio};

pub struct Writer {
    initialized: bool,
}

pub static mut WRITER: Writer = Writer { initialized: false };

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        let uart = unsafe { &mut mk66::uart::UART0 };
        if !self.initialized {
            self.initialized = true;
            uart.init(uart::UARTParams {
                baud_rate: 115200,
                stop_bits: uart::StopBits::One,
                parity: uart::Parity::None,
                hw_flow_control: false,
            });
            uart.enable_tx();
        }

        for c in s.bytes() {
            uart.send_byte(c);
        }
        while !uart.tx_ready() {}

        Ok(())
    }
}

#[cfg(not(test))]
#[no_mangle]
#[allow(unused_variables)]
#[lang="panic_fmt"]
pub unsafe extern "C" fn panic_fmt(args: Arguments, file: &'static str, line: u32) -> ! {
    let writer = &mut WRITER;
    let _ = writer.write_fmt(format_args!("\r\n\nKernel panic at {}:{}:\r\n\t\"", file, line));
    let _ = write(writer, args);
    let _ = writer.write_str("\"\r\n");

    // Print version of the kernel
    let _ = writer.write_fmt(format_args!("\tKernel version {}\r\n", env!("TOCK_KERNEL_VERSION")));

    // Print fault status once
    let procs = &mut process::PROCS;
    if procs.len() > 0 {
        procs[0].as_mut().map(|process| { process.fault_str(writer); });
    }

    // print data about each process
    let _ = writer.write_fmt(format_args!("\r\n---| App Status |---\r\n"));
    let procs = &mut process::PROCS;
    for idx in 0..procs.len() {
        procs[idx].as_mut().map(|process| { process.statistics_str(writer); });
    }

    // blink the panic signal
    gpio::PC05.release_claim();
    let led = gpio::PC05.claim_as_gpio(); 
    led.enable_output();
    loop {
        for _ in 0..1000000 {
            led.clear();
        }
        for _ in 0..100000 {
            led.set();
        }
        for _ in 0..1000000 {
            led.clear();
        }
        for _ in 0..500000 {
            led.set();
        }
    }
}

#[macro_export]
macro_rules! print {
        ($($arg:tt)*) => (
            {
                use core::fmt::write;
                let writer = unsafe { &mut $crate::io::WRITER };
                let _ = write(writer, format_args!($($arg)*));
            }
        );
}

#[macro_export]
macro_rules! println {
        ($fmt:expr) => (print!(concat!($fmt, "\n")));
            ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
