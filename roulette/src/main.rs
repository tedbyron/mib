#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::display::blocking::Display;
use microbit::hal::timer::Timer;
use microbit::Board;
use panic_halt as _;

#[rustfmt::skip]
const PIXELS: [(usize, usize); 16] = [
    (0, 0), (0, 1), (0, 2), (0, 3), (0, 4),
                                    (1, 4),
                                    (2, 4),
                                    (3, 4),
                                    (4, 4),
                            (4, 3),
                    (4, 2),
            (4, 1),
    (4, 0),
    (3, 0),
    (2, 0),
    (1, 0)
];

#[allow(clippy::missing_const_for_fn, clippy::empty_loop)]
#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds = [[0; 5]; 5];
    let (mut last_col, mut last_row) = (0, 0);

    loop {
        for (col, row) in PIXELS {
            leds[last_col][last_row] = 0;
            leds[col][row] = 1;
            display.show(&mut timer, leds, 50);
            (last_col, last_row) = (col, row);
        }
    }
}
