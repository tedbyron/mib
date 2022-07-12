#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::hal::prelude::*;
use microbit::hal::timer::Timer;
use microbit::Board;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[allow(clippy::missing_const_for_fn, clippy::empty_loop)]
#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    board.display_pins.col1.set_low().unwrap();
    let mut row1 = board.display_pins.row1;

    loop {
        row1.set_low().unwrap();
        rprintln!("dark");
        timer.delay_ms(1_000_u16);
        row1.set_high().unwrap();
        rprintln!("light");
        timer.delay_ms(1_000_u16);
    }
}
