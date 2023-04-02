#![no_std]
#![no_main]

use core::f32::consts::FRAC_PI_8;

use cortex_m_rt::entry;
use libm::{atan2f, sqrtf};
use lsm303agr::{AccelOutputDataRate, Lsm303agr, MagOutputDataRate, Measurement};
use microbit::display::blocking::Display;
use microbit::hal::twim::Twim;
use microbit::hal::Timer;
use microbit::pac::twim0::frequency::FREQUENCY_A;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

mod calibration;
mod led;

use calibration::{calc_calibration, calibrated_measurement};
use led::Direction;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let i2c = Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut sensor = Lsm303agr::new_with_i2c(i2c)
        .into_mag_continuous()
        .ok()
        .unwrap();
    sensor.init().unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz10).unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();

    let calibration = calc_calibration(&mut sensor, &mut display, &mut timer);
    rprintln!("Calibration: {:#?}", calibration);

    loop {
        while !sensor.mag_status().unwrap().xyz_new_data {}
        let data = sensor.mag_data().unwrap();
        let Measurement { x, y, z } = calibrated_measurement(data, &calibration);
        let (x, y, z) = (x as f32, y as f32, z as f32);
        let theta = atan2f(y, x);

        let dir = match theta {
            theta if theta < -7.0 * FRAC_PI_8 => Direction::West,
            theta if theta < -5.0 * FRAC_PI_8 => Direction::SouthWest,
            theta if theta < -3.0 * FRAC_PI_8 => Direction::South,
            theta if theta < -FRAC_PI_8 => Direction::SouthEast,
            theta if theta < FRAC_PI_8 => Direction::East,
            theta if theta < 3.0 * FRAC_PI_8 => Direction::NorthEast,
            theta if theta < 5.0 * FRAC_PI_8 => Direction::North,
            theta if theta < 7.0 * FRAC_PI_8 => Direction::NorthWest,
            _ => Direction::West,
        };

        display.show(&mut timer, dir.to_led_display(), 200);

        #[allow(clippy::suboptimal_flops)]
        let magnitude = sqrtf(x * x + y * y + z * z); // nT
        rprintln!("{:.2} μT", magnitude / 1000.0)
    }
}
