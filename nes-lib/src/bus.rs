use core::fmt::Debug;
use core::write;

use super::cart::Cart;

pub type Addr = u16;

pub trait Bus {
    fn read(&mut self, addr: Addr) -> u8;
    fn write(&mut self, addr: Addr, value: u8);
}

pub struct NesBus<C: Cart> {
    iram: [u8; 0x800],
    ppu: [u8; 0x8], // will be replaced by PPU
    apu_io: [u8; 0x18], // will be replaced by APU/IO
    cart: C,
}

impl<C: Cart> NesBus<C> {
    pub fn new(cart: C) -> Self {
        Self {
            iram: [0; 0x800],
            ppu: [0; 0x8],
            apu_io: [0; 0x18],
            cart,
        }
    }
}

impl<C: Cart> Bus for NesBus<C> {
    fn read(&mut self, addr: Addr) -> u8 {
        0
    }

    fn write(&mut self, addr: Addr, value: u8) {
        
    }
}

impl<C: Cart> Debug for NesBus<C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Bus {{ cart: {}, OTHER_MEM }}", core::str::from_utf8(&self.cart.name()).unwrap())
    }
}
