use crate::{Cpu, Bus};
use crate::cart::Cart;

#[derive(Debug)]
pub struct Nes<B: Bus> {
    cpu: Cpu,
    bus: B,
}

impl<B: Bus> Nes<B> {
    pub fn new(bus: B) -> Self {
        Self {
            cpu: Cpu::new(),
            bus,
        }
    }
}
