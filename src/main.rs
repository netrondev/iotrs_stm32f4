#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

// use panic_halt as _;
use core::panic::PanicInfo;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;

use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let p = pac::Peripherals::take().unwrap();

    let gpioc = p.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();

    let mut counter: usize = 0;

    loop {
        for _ in 0..10_000 {
            led.set_high();
        }
        for _ in 0..10_000 {
            led.set_low();
        }

        counter += 1;

        rprintln!("Counter {}", counter);

        if counter > 10 {
            panic!("This is an intentional panic.");
        }
    }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {} // You might need a compiler fence in here.
}
