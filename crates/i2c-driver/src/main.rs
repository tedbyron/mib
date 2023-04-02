#![no_std]
#![no_main]

use cortex_m_rt::entry;
use lsm303agr::{AccelOutputDataRate, Lsm303agr, Measurement};
use microbit as _;
use microbit::hal::Twim;
use microbit::pac::twim0::frequency::FREQUENCY_A;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = microbit::Board::take().unwrap();
    let i2c = Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut sensor = Lsm303agr::new_with_i2c(i2c);

    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();

    loop {
        if sensor.accel_status().unwrap().xyz_new_data {
            let Measurement { x, y, z } = sensor.accel_data().unwrap();
            rprintln!("Acceleration: ({}, {}, {})", x, y, z);
        }
    }
}
