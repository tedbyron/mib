#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::hal::prelude::*;
use microbit::hal::timer::Timer;
use microbit::Board;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut row3 = board.display_pins.row3;

    board.display_pins.col3.set_low().unwrap();

    loop {
        row3.set_low().unwrap();
        rprintln!("dark");
        timer.delay_ms(1_000_u16);
        row3.set_high().unwrap();
        rprintln!("light");
        timer.delay_ms(1_000_u16);
    }
}
