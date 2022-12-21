// ref. http://osblog.stephenmarz.com/ch2.html

const IER_RX_ENABLE: u8 = 1 << 0;
const IER_TX_ENABLE: u8 = 1 << 1;
const FCR_FIFO_ENABLE: u8 = 1 << 0;
const FCR_FIFO_CLEAR: u8 = 3 << 1; // clear the content of the two FIFOs
const LCR_EIGHT_BITS: u8 = 3 << 0;
const LCR_BAUD_LATCH: u8 = 1 << 7; // special mode to set baud rate
const LSR_RX_READY: u8 = 1 << 0; // input is waiting to be read from RHR
const LSR_TX_IDLE: u8 = 1 << 5; // THR can accept another character to send

const UART_BUF_SIZE: usize = 32;

enum Register {
    RHR, // receive holding register (for input bytes)
    THR, // transmit holding register (for output bytes)
    IER, // interrupt enable register
    FCR, // FIFO control register
    ISR, // interrupt status register
    LCR, // line control register
    LSR, // line status register
}

impl Register {
    fn addr(&self, base: usize) -> *mut u8 {
        match *self {
            Register::RHR | Register::THR => (base + 0) as *mut u8,
            Register::IER => (base + 1) as *mut u8,
            Register::FCR | Register::ISR => (base + 2) as *mut u8,
            Register::LCR => (base + 3) as *mut u8,
            Register::LSR => (base + 5) as *mut u8,
        }
    }
}

pub struct UART {
    base: usize,
}

impl UART {
    pub const fn new(base: usize) -> UART {
        UART { base }
    }

    pub fn init(&mut self) {
        // disable interrupts
        self.write(Register::IER, 0);

        // special mode to set baud rate
        self.write(Register::LCR, LCR_BAUD_LATCH);

        // LSB for baud rate of 38.4K
        self.write(Register::RHR, 0x03);
        // MSB for baud rate of 38.4K
        self.write(Register::IER, 0x00);

        // leave set-baud mode, and set word length to 8 bits, no parity
        self.write(Register::LCR, LCR_EIGHT_BITS);

        // enable FIFO, clear them
        self.write(Register::FCR, FCR_FIFO_ENABLE | FCR_FIFO_CLEAR);

        // enable transmit and receive interrupts
        self.write(Register::IER, IER_RX_ENABLE | IER_TX_ENABLE);
    }

    fn read(&self, reg: Register) -> u8 {
        unsafe { reg.addr(self.base).read_volatile() }
    }

    fn write(&self, reg: Register, data: u8) {
        unsafe { reg.addr(self.base).write_volatile(data) }
    }

    pub fn getc(&self) -> Option<u8> {
        if self.read(Register::LSR) & LSR_RX_READY == 1 {
            Some(self.read(Register::RHR))
        } else {
            None
        }
    }

    pub fn putc(&self, c: u8) {
        self.write(Register::THR, c);
    }
}
