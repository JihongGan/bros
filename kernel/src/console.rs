use crate::uart::UART;
use core::fmt::{self, Write};

pub const CONSOLE_UART_BASE: usize = 0x1000_0000;

pub struct Stdout {
    uart: UART,
}

impl Stdout {
    fn new() -> Stdout {
        Stdout {
            uart: UART::new(CONSOLE_UART_BASE),
        }
    }

    pub fn print(args: fmt::Arguments) {
        Stdout::new().write_fmt(args).unwrap();
    }
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            self.uart.putc(c);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::Stdout::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::Stdout::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
