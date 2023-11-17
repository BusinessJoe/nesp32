#![no_std]
#![no_main]

use nes_lib;

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let rom = include_bytes!("../nestest.nes");
    let cart = nes_lib::cart::NoMapperCart::new(b"Cart Name", rom);
    let bus = nes_lib::NesBus::new(cart);
    let nes = nes_lib::Nes::new(bus);

    println!("Hello world!");
    loop {
        println!("pc: {}", nes.cpu.pc);
        delay.delay_ms(500u32);
    }
}
