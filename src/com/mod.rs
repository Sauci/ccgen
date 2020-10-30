use heapless::Vec;
use heapless::consts::U32;
use stm32f1::stm32f103::Interrupt;
use stm32f1::stm32f103::interrupt;
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

/// Initialize UART communication
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
    // 115200 bps -> 19.5 = fck / (16 * Baudrate) = 36e6 / (16 * 115200)

    ser.cr1.modify(|_, w| w
        .m().m8()               // 8-bit length
        .te().enabled()         // transmission enabled
        .re().enabled()         // reception enabled
        .rxneie().enabled()     // interrupt on rx register not empty enabled
    );

    ser.cr2.modify(|_, w| w
        .stop().stop1()       // stop 1 bit
    );

    ser.brr.write(|w| w
        .div_mantissa().bits(19)    
        .div_fraction().bits(8)     // 0.5 * 16 = 8
    );

    // Configure NVIC
    unsafe {
        NVIC::unmask(Interrupt::USART2);
    }

    ser.cr1.modify(|_, w| w.ue().enabled()); // start peripheral
}

/// Read received data
/// 
/// **Arguments**
/// 
/// * dat: reception buffer
/// 
/// **Return value**
/// 
/// * Ok(usize): reception buffer could be read, number of read bytes returned
/// * Err(()): reception buffer couldn't be read, should not append at the moment
pub fn read_data(dat: &mut[u8]) -> Result<usize, ()> {
    let mut recv = 0;
    let buf = unsafe { &mut BUF };
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

/// Send data through UART, not blocking
/// 
/// **Arguments**
/// 
/// * dat: buffer to send
/// 
/// **Return value**
/// 
/// Either:
/// * Ok(()): buffer ok to be sent
/// * Err(()): sending buffer full
pub fn send_data(dat: &[u8]) -> Result<(), ()> {
    let ser = periph!(USART2);
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
    buf.status = SerSts::Sending;
    ser.cr1.modify(|_, w| w.txeie().enabled());

    Ok(())
}

/// Get current UART driver status, either:
/// * Idle: driver finished last transmission, nothing received since then
/// * Receiving: driver received something
/// * Sending: driver is currently sending data
/// * Error: error reported due to buffer overflow
pub fn get_state() -> SerSts {
    unsafe { BUF.status }
}

#[interrupt]
fn USART2() {
    let usart = periph!(USART2);
    let mut buffer = unsafe { &mut BUF };

    if usart.sr.read().txe().bits() && usart.cr1.read().txeie().is_enabled() {
        if let Some(data) = buffer.wr_buf.pop() {
            usart.dr.write(|w| w.dr().bits(data as u16));
        } else {
            // Disable transmission interupt if nothing to send
            usart.cr1.modify(|_, w| w.txeie().disabled());
            buffer.status = SerSts::Idle;
        }
    }

    if usart.sr.read().rxne().bit_is_set() {
        let recv = usart.dr.read().bits();
        match buffer.rd_buf.push(recv as u8) {
            Ok(()) => {
                buffer.status = SerSts::Receiving;
                buffer.recv_nr += 1;
            },
            Err(_) => {
                buffer.status = SerSts::Error;
            },
        }
    }
}