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
    #[cfg(feature = "biased")]
    rng.set_debiasing(false);
    let mut timer = Timer::new(board.TIMER0);
    timer.start(!0);


    #[cfg(feature = "blocked")]
    const BLOCK_SIZE: usize = 1000;
    #[cfg(not(feature = "blocked"))]
    const BLOCK_SIZE: usize = 1;

    let mut count = 0u64;
    let mut sum = 0;
    let mut one_bits = 0u64;

    while timer.read() < 1_000_000 {
        if cfg!(feature = "blocked") {
            let mut buf = [0; BLOCK_SIZE];
            rng.random(&mut buf);
            for b in buf {
                if cfg!(feature = "measure_bias") {
                    one_bits += b.count_ones() as u64;
                } else {
                    sum ^= b;
                }
            }
        } else {
            let b = rng.random_u8();
            if cfg!(feature = "measure_bias") {
                one_bits += b.count_ones() as u64;
            } else {
                sum ^= b;
            }
        }
        count += BLOCK_SIZE as u64;
    }

    if cfg!(feature = "measure_bias") {
        rprintln!("{} {} {}", count, one_bits, 8 * count - one_bits);
    } else {
        rprintln!("{} {}", count, sum);
    }

    loop {
        core::hint::spin_loop();
    }
}
