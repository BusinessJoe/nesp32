use core::fmt::Debug;

use super::cart::Cart;

pub type Addr = u16;

pub struct Bus<C: Cart> {
    iram: [u8; 0x800],
    ppu: [u8; 0x8],
    apu_io: [u8; 0x18],
    cart: C,
}

impl<C: Cart> Bus<C> {
    pub fn new(cart: C) -> Self {
        Self {
            iram: [0; 0x800],
            ppu: [0; 0x8],
            apu_io: [0; 0x18],
            cart,
        }
    }
}

impl<C: Cart> Debug for Bus<C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Bus {{ cart: {} MEM }}", core::str::from_utf8(&self.cart.name()).unwrap())
    }
}
