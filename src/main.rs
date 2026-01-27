#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::board::Board;
use nrf52833_hal::{Rng, Timer};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut rng = Rng::new(board.RNG);
    #[cfg(feature = "unwhitened")]
    rng.set_whitening(false);
    let mut timer = Timer::new(board.TIMER0);
    timer.start(!0);
    let mut sum = 0;
    let mut count = 0u64;
    while timer.read() < 1_000_000 {
        sum ^= rng.random_u8();
        count += 1;
    }
    rprintln!("{} {}", count, sum);
    loop {
        core::hint::spin_loop();
    }
}
