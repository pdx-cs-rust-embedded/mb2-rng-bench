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

    #[cfg(feature = "blocked")]
    const BLOCK_SIZE: u64 = 1000;
    #[cfg(not(feature = "blocked"))]
    const BLOCK_SIZE: u64 = 1;

    while timer.read() < 1_000_000 {
        if cfg!(feature = "blocked") {
            let mut buf = [0; BLOCK_SIZE as usize];
            rng.random(&mut buf);
            for b in buf {
                sum ^= b;
            }
        } else {
            sum ^= rng.random_u8();
        }
        count += BLOCK_SIZE;
    }
    rprintln!("{} {}", count, sum);
    loop {
        core::hint::spin_loop();
    }
}
