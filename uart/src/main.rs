#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::hal::prelude::*;
use microbit::hal::uarte::{self, Baudrate, Parity};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

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

    loop {
        let b = nb::block!(uarte.read()).unwrap();
        rprintln!("{}", char::from(b));
    }
}
