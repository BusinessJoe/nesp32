use crate::{Bus, Cpu};

#[derive(Debug)]
pub struct Nes<B: Bus> {
    pub cpu: Cpu<B>,
    pub bus: B,
}

impl<B: Bus> Nes<B> {
    pub fn new(bus: B) -> Self {
        Self {
            cpu: Cpu::new(),
            bus,
        }
    }
}
