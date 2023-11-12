use crate::emu::{Cpu, Bus};
use crate::emu::cart::Cart;

#[derive(Debug)]
pub struct Nes<C: Cart> {
    cpu: Cpu,
    bus: Bus<C>,
}

impl<C: Cart> Nes<C> {
    pub fn new(cart: C) -> Self {
        Self {
            cpu: Cpu::new(),
            bus: Bus::new(cart),
        }
    }
}
