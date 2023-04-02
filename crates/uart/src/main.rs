#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt::entry;
use heapless::Vec;
use microbit::hal::prelude::*;
use microbit::hal::uarte::{self, Baudrate, Parity};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

mod serial_setup;

use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = microbit::Board::take().unwrap();
    let serial = uarte::Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );
    let mut serial = UartePort::new(serial);
    let mut buf = Vec::<_, 64>::new();

    loop {
        buf.clear();

        loop {
            let b = nb::block!(serial.read()).unwrap();
            nb::block!(serial.write(b)).unwrap();
            nb::block!(serial.flush()).unwrap();

            if buf.push(b).is_err() {
                write!(serial, "uh oh buffer full\r\n").unwrap();
                break;
            }

            if b == b'\r' || b == b'\n' {
                write!(serial, "\r\n").unwrap();
                for &b in buf.iter().rev().chain(&[b'\r', b'\n']) {
                    nb::block!(serial.write(b)).unwrap();
                }
                break;
            }
        }

        nb::block!(serial.flush()).unwrap();
    }
}
