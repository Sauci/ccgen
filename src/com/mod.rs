use heapless::Vec;
use heapless::consts::U32;
use stm32f1::stm32f103::Interrupt;
use cortex_m::peripheral::NVIC;

use crate::periph;

#[derive(Copy, Clone)]
pub enum SerSts {
    Sending,
    Receiving,
    Idle,
    Error,
}

struct SerRwBuf {
    wr_buf: Vec<u8, U32>,
    rd_buf: Vec<u8, U32>,
    status: SerSts,
    recv_nr: usize,
}

static mut BUF: SerRwBuf = SerRwBuf {
    wr_buf : Vec(heapless::i::Vec::new()),
    rd_buf : Vec(heapless::i::Vec::new()),
    status : SerSts::Idle,
    recv_nr: 0,
};

pub fn init() {
    let rcc = periph!(RCC);
    let pa = periph!(GPIOA);
    let ser = periph!(USART2);

    // Enable clocks for peripherals
    rcc.apb2enr.modify(|_, w| w.iopaen().enabled());
    rcc.apb1enr.modify(|_, w| w.usart2en().enabled());

    // Configure port according to P.166/167 reference manual for USART configuration
    // 
    pa.crl.modify(|_, w| w
        // GPIOA2: TX -> alternative push-pull
        .mode2().output()
        .cnf2().alt_push_pull()
        // GPIOA3: RX -> input floating (open_drain)
        .mode3().input()
        .cnf3().open_drain()
    );

    // Configuration:
    // 8-bit frame
    // 1 stop bit
    // 115200 bps -> 39.0625 = fck / (16 * Baudrate) = 72e6 / (16 * 115200)

    ser.cr1.modify(|_, w| w
        .m().m8()               // 8-bit length
        .te().enabled()         // transmission enabled
        .re().enabled()         // reception enabled
        .txeie().enabled()      // interrupt on tx register empty enabled
        .rxneie().enabled()     // interrupt on rx register not empty enabled
    );

    ser.cr2.modify(|_, w| w
        .stop().stop1()       // stop 1 bit
    );

    ser.brr.write(|w| w
        .div_mantissa().bits(39)    
        .div_fraction().bits(1)     // 0.0625 * 16 = 1
    );

    // Configure NVIC
    unsafe {
        NVIC::unmask(Interrupt::USART2);
    }

    ser.cr1.modify(|_, w| w.ue().enabled()); // start peripheral
}

pub fn read_data(dat: &mut[u8]) -> Result<usize, ()> {
    let mut recv = 0;
    let mut buf = unsafe { &mut BUF };
    for d in dat.iter_mut() {
        match buf.rd_buf.pop() {
            Some(val) => {
                *d = val;
                recv += 1;
            },
            None => break,
        };
    }
    Ok(recv)
}

pub fn send_data(dat: &[u8]) -> Result<(), ()> {
    let mut ser = periph!(USART2);
    let mut buf = unsafe { &mut BUF };
    for d in dat.iter() {
        match buf.wr_buf.push(*d) {
            Ok(()) => continue,
            Err(_) => {
                buf.status = SerSts::Error;
                return Err(());
            }
        }
    }

    ser.cr1.modify(|_, w| w.txeie().enabled());

    Ok(())
}

pub fn get_state() -> SerSts {
    unsafe { BUF.status }
}

pub fn ser_handler() {
    let usart = periph!(USART2);
    let status = usart.sr.read().bits();
    let mut buffer = unsafe { &mut BUF };

    if usart.sr.read().txe().bits() {
        if let Some(data) = buffer.wr_buf.pop() {
            usart.dr.write(|w| w.dr().bits(data as u16));
        } else {
            // Disable transmission interupt if nothing to send
            usart.cr1.modify(|_, w| w.txeie().disabled()); 
        }
    }

    if usart.sr.read().rxne().bit_is_set() {
        match buffer.rd_buf.push(usart.dr.read().bits() as u8) {
            Ok(()) => {
                buffer.status = SerSts::Receiving;
                buffer.recv_nr += 1;
            },
            Err(data) => {
                buffer.status = SerSts::Error;
            },
        }
    }
}