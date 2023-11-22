use crate::{Bus, Cpu};

#[derive(Debug)]
pub struct Nes<B: Bus> {
    pub cpu: Cpu<B>,
    pub bus: B,
    pub time: u128,
}

impl<B: Bus> Nes<B> {
    pub fn new(bus: B) -> Self {
        let mut cpu = Cpu::new();
        let mut bus = bus;
        let pc = u16::from_le_bytes([bus.read(0xfffc), bus.read(0xfffd)]);
        cpu.pc = pc;

        Self {
            cpu,
            bus,
            time: 0,
        }
    }

    pub fn tick(&mut self) {
        self.time += 1;
        self.bus.catch_up(self.time);
        self.cpu.catch_up(self.time, &mut self.bus);
    }
}
