use crate::{Bus, Cpu};

#[cfg(feature = "debug")]
use crate::Addr;

pub struct Nes<B: Bus> {
    pub cpu: Cpu<B>,
    pub bus: B,
    pub time: u128,

    #[cfg(feature = "debug")]
    pub on_event: EventCb,
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

            #[cfg(feature = "debug")]
            on_event: default_event_cb()
        }
    }

    pub fn tick(&mut self) {
        self.time += 1;
        self.bus.catch_up(self.time);
        self.cpu.catch_up(self.time, &mut self.bus);
    }
}

#[cfg(feature = "debug")]
pub type EventCb = fn(EventSource);
#[cfg(feature = "debug")]
pub fn default_event_cb() -> EventCb {
    |_| {}
}
#[cfg(feature = "debug")]
pub enum EventSource {
    BusRead { addr: Addr, val: u8 },
    BusWrite { addr: Addr, val: u8 },
    VramRead { addr: usize, val: u8 },
    VramWrite { addr: usize, val: u8 },
}
