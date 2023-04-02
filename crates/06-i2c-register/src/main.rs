#![no_std]
#![no_main]

use core::sync::atomic::{self, Ordering};

use cortex_m_rt::entry;
use microbit::hal::pac::twim0::frequency::FREQUENCY_A;
use microbit::hal::prelude::*;
use microbit::hal::twim::Twim;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

// https://www.st.com/resource/en/datasheet/lsm303agr.pdf
/// LSM303AGR registers.
mod lsm303agr {
    /// Accelerometer i2c slave address.
    pub const ACC_ADDR: u8 = 0b0011001;
    /// Magnetometer i2c slave address.
    pub const MAG_ADDR: u8 = 0b0011110;
    /// Accelerometer identifier.
    pub const WHO_AM_I_A: u8 = 0x0F;
    /// Magnetometer identifier.
    pub const WHO_AM_I_M: u8 = 0x4F;
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = microbit::Board::take().unwrap();
    let mut i2c = Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);

    let (mut acc, mut mag) = ([0], [0]);

    i2c.write_read(lsm303agr::ACC_ADDR, &[lsm303agr::WHO_AM_I_A], &mut acc)
        .unwrap();
    i2c.write_read(lsm303agr::MAG_ADDR, &[lsm303agr::WHO_AM_I_M], &mut mag)
        .unwrap();

    rprintln!("Accelerometer ID: {:08b}", acc[0]);
    rprintln!(" Magnetometer ID: {:08b}", mag[0]);

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
