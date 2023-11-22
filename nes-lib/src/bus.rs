use crate::ppu::{Ppu, Screen};

use super::cart::Cart;

pub type Addr = u16;

pub trait Bus {
    fn read(&mut self, addr: Addr) -> u8;
    fn write(&mut self, addr: Addr, value: u8);
    fn catch_up(&mut self, time: u128);
}

pub struct NesBus<C: Cart, S: Screen> {
    pub iram: [u8; 0x800],
    pub apu_io: [u8; 0x18], // will be replaced by APU/IO
    pub cart: C,
    pub ppu: Ppu<S>,
    pub time: u128,

    #[cfg(feature = "debug")]
    on_read_cb: fn(addr: Addr),
    #[cfg(feature = "debug")]
    on_write_cb: fn(addr: Addr, val: u8),
}

impl<C: Cart, S: Screen> NesBus<C, S> {
    pub fn new(cart: C, ppu: Ppu<S>) -> Self {
        Self {
            iram: [0; 0x800],
            apu_io: [0; 0x18],
            cart,
            ppu,
            time: 0,
            #[cfg(feature = "debug")]
            on_read_cb: |_| {},
            #[cfg(feature = "debug")]
            on_write_cb: |_, _| {},
        }
    }
}

#[cfg(feature = "debug")]
impl<C: Cart, S: Screen> NesBus<C, S> {
    pub fn on_read(&mut self, callback: fn(Addr)) {
        self.on_read_cb = callback;
    }

    pub fn on_write(&mut self, callback: fn(Addr, u8)) {
        self.on_write_cb = callback;
    }
}

#[cfg(not(feature = "debug"))]
impl<C: Cart, S: Screen> NesBus<C, S> {
    pub fn on_read<F>(&mut self, _: F)
    where
        F: Fn(Addr),
    {
        panic!()
    }

    pub fn on_write<F>(&mut self, _: F)
    where
        F: Fn(Addr, u8),
    {
        panic!()
    }
}

impl<C: Cart, S: Screen> Bus for NesBus<C, S> {
    fn read(&mut self, addr: Addr) -> u8 {
        #[cfg(feature = "debug")]
        (self.on_read_cb)(addr);

        let a = usize::from(addr);
        match addr {
            0x0000..=0x1fff => self.iram[a % 0x800],
            // PPU registers (8 bytes) and mirrors.
            0x2000..=0x3fff => 0,
            0x4000..=0x4017 => self.apu_io[a - 0x4000],
            // Normally disabled APU and I/O functionality.
            0x4018..=0x401f => 0,
            0x4020..=0xffff => self.cart.read(addr),
        }
    }

    fn write(&mut self, addr: Addr, value: u8) {
        #[cfg(feature = "debug")]
        (self.on_write_cb)(addr, value);

        let a = usize::from(addr);
        match addr {
            0x0000..=0x1fff => self.iram[a % 0x800] = value,
            // PPU registers (8 bytes) and mirrors.
            0x2000..=0x3fff => {}
            0x4000..=0x4017 => self.apu_io[a - 0x4000] = value,
            // Normally disabled APU and I/O functionality.
            0x4018..=0x401f => {}
            0x4020..=0xffff => self.cart.write(addr, value),
        }
    }

    fn catch_up(&mut self, time: u128) {
        self.time = time;
    }
}
