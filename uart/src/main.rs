#![no_main]
#![no_std]

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
    let mut uarte = UartePort::new(serial);
    let mut buf = Vec::<_, 64>::new();

    loop {
        buf.clear();

        loop {
            let b = nb::block!(uarte.read()).unwrap();
            nb::block!(uarte.write(b)).unwrap();
            nb::block!(uarte.flush()).unwrap();

            if buf.push(b).is_err() {
                write!(uarte, "uh oh buffer full\r\n").unwrap();
                break;
            }

            if b == b'\r' || b == b'\n' {
                write!(uarte, "\r\n").unwrap();
                for &b in buf.iter().rev().chain(&[b'\r', b'\n']) {
                    nb::block!(uarte.write(b)).unwrap();
                }
                break;
            }
        }

        nb::block!(uarte.flush()).unwrap();
    }
}
