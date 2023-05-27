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

    // let gpioc = p.GPIOC.split();
    // let mut led = gpioc.pc13.into_push_pull_output();

    let gpiod = p.GPIOD.split();

    let mut led_green = gpiod.pd12.into_push_pull_output();
    let mut led_orange = gpiod.pd13.into_push_pull_output();
    let mut led_red = gpiod.pd14.into_push_pull_output();
    let mut led_blue = gpiod.pd15.into_push_pull_output();

    let mut counter: usize = 0;

    loop {
        for _ in 0..10_000 {
            led_green.set_high();
        }
        for _ in 0..10_000 {
            led_green.set_low();
        }

        for _ in 0..10_000 {
            led_orange.set_high();
        }
        for _ in 0..10_000 {
            led_orange.set_low();
        }

        for _ in 0..10_000 {
            led_red.set_high();
        }
        for _ in 0..10_000 {
            led_red.set_low();
        }

        for _ in 0..10_000 {
            led_blue.set_high();
        }
        for _ in 0..10_000 {
            led_blue.set_low();
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
