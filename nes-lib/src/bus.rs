use core::fmt::Debug;
use core::write;

use crate::ppu::{Screen, Ppu};

use super::cart::Cart;

pub type Addr = u16;

pub trait Bus {
    fn read(&mut self, addr: Addr) -> u8;
    fn write(&mut self, addr: Addr, value: u8);
}

pub struct NesBus<C: Cart, S: Screen> {
    iram: [u8; 0x800],
    apu_io: [u8; 0x18], // will be replaced by APU/IO
    cart: C,
    ppu: Ppu<S>,
}

impl<C: Cart, S: Screen> NesBus<C, S> {
    pub fn new(cart: C, ppu: Ppu<S>) -> Self {
        Self {
            iram: [0; 0x800],
            apu_io: [0; 0x18],
            cart,
            ppu,
        }
    }
}

impl<C: Cart, S: Screen> Bus for NesBus<C, S> {
    fn read(&mut self, addr: Addr) -> u8 {
        0
    }

    fn write(&mut self, addr: Addr, value: u8) {}
}

impl<C: Cart, S: Screen> Debug for NesBus<C, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Bus {{ cart: {}, OTHER_MEM }}",
            core::str::from_utf8(&self.cart.name()).unwrap()
        )
    }
}
