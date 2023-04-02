#![no_std]
#![no_main]

use core::fmt::Write;
use core::str;

use cortex_m_rt::entry;
use heapless::Vec;
use lsm303agr::{AccelOutputDataRate, Lsm303agr, MagOutputDataRate, Measurement};
use microbit::hal::prelude::*;
use microbit::hal::uarte::{Baudrate, Parity, Uarte};
use microbit::hal::Twim;
use microbit::pac::twim0::frequency::FREQUENCY_A;
use nb::block;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

static mut TX_BUF: [u8; 1] = [0];
static mut RX_BUF: [u8; 1] = [0];

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = microbit::Board::take().unwrap();

    let serial = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );
    let (mut tx, mut rx) = serial
        .split(unsafe { &mut TX_BUF }, unsafe { &mut RX_BUF })
        .unwrap();
    let mut input = Vec::<u8, 64>::new();

    let i2c = Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut sensor = Lsm303agr::new_with_i2c(i2c)
        .into_mag_continuous()
        .ok()
        .unwrap();
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz50).unwrap();

    'outer: loop {
        input.clear();
        block!(tx.flush()).unwrap();

        loop {
            let b = block!(rx.read()).unwrap();
            block!(tx.write(b)).unwrap();
            block!(tx.flush()).unwrap();

            if b == b'\r' || b == b'\n' {
                break;
            }

            if input.push(b).is_err() {
                write!(tx, " input buffer is full!\r\n").unwrap();
                continue 'outer;
            }
        }

        write!(tx, "\r\n").unwrap();

        match str::from_utf8(&input).unwrap().trim() {
            acc if acc.starts_with("acc") => {
                while !sensor.accel_status().unwrap().xyz_new_data {}
                let Measurement { x, y, z } = sensor.accel_data().unwrap();
                write!(tx, "Accelerometer: ({x}, {y}, {z})\r\n").unwrap();
            }
            mag if mag.starts_with("mag") => {
                while !sensor.mag_status().unwrap().xyz_new_data {}
                let Measurement { x, y, z } = sensor.mag_data().unwrap();
                write!(tx, "Magnetometer: ({x}, {y}, {z})\r\n").unwrap();
            }
            _ => write!(tx, "Unknown command\r\n").unwrap(),
        }
    }
}
