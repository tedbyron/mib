#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit as _;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello World");
    loop {}
}
