#![no_std]
#![no_main]

mod emu;

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let cart = emu::cart::NoMapperCart::new(b"Cart Name");
    let nes = emu::Nes::new(cart);

    println!("Hello world!");
    loop {
        println!("{:?}", nes);
        delay.delay_ms(500u32);
    }
}
