extern crate alloc;

mod ppu_bus;

use crate::ppu::{Ppu, Screen};

#[cfg(feature = "debug")]
use crate::nes::{EventCb, default_event_cb};

use self::ppu_bus::PpuBus;
pub use self::ppu_bus::PpuBusWithCart;

use super::cart::Cart;

pub type Addr = u16;

pub trait Bus {
    fn read(&mut self, addr: Addr) -> u8;
    fn write(&mut self, addr: Addr, value: u8);
    fn catch_up(&mut self, time: u128);
    fn debug_chr(&self) -> Option<[u8; 0x2000]>;
}

pub trait PpuBusT {
    fn read(&mut self, addr: Addr) -> u8;
    fn write(&mut self, addr: Addr, value: u8);
}

pub struct NesBus<C: Cart, S: Screen> {
    pub iram: [u8; 0x800],
    pub apu_io: [u8; 0x18], // will be replaced by APU/IO
    pub cart: C,
    pub ppu: Ppu<S>,
    pub ppu_bus: PpuBus,
    pub time: u128,

    #[cfg(feature = "debug")]
    pub on_event: EventCb,
}

impl<C: Cart, S: Screen> NesBus<C, S> {
    pub fn new(cart: C, ppu: Ppu<S>) -> Self {
        Self {
            iram: [0; 0x800],
            apu_io: [0; 0x18],
            cart,
            ppu,
            ppu_bus: PpuBus { vram: [0; 0x800] },
            time: 0,

            #[cfg(feature = "debug")]
            on_event: default_event_cb(),
        }
    }
}

impl<C: Cart, S: Screen> Bus for NesBus<C, S> {
    fn read(&mut self, addr: Addr) -> u8 {
        let a = usize::from(addr);
        let val = match addr {
            0x0000..=0x1fff => self.iram[a % 0x800],
            // PPU registers (8 bytes) and mirrors.
            0x2000..=0x3fff => {
                self.ppu.catch_up(self.time);
                let mut ppu_bus = self.ppu_bus.with_cart(&mut self.cart);
                #[cfg(feature = "debug")]
                {
                    ppu_bus.on_event = self.on_event.clone();
                }
                self.ppu.read(0x2000 + addr % 8, &mut ppu_bus)
            }
            0x4000..=0x4017 => self.apu_io[a - 0x4000],
            // Normally disabled APU and I/O functionality.
            0x4018..=0x401f => 0,
            0x4020..=0xffff => self.cart.read(addr).unwrap(),
        };

        #[cfg(feature = "debug")]
        {
            //(self.on_event)(EventSource::BusRead{ addr, val });
        }

        val
    }

    fn write(&mut self, addr: Addr, value: u8) {
        let a = usize::from(addr);
        match addr {
            0x0000..=0x1fff => self.iram[a % 0x800] = value,
            // PPU registers (8 bytes) and mirrors.
            0x2000..=0x3fff => {
                self.ppu.catch_up(self.time);
                let mut ppu_bus = self.ppu_bus.with_cart(&mut self.cart);
                #[cfg(feature = "debug")]
                {
                    ppu_bus.on_event = self.on_event.clone();
                }
                self.ppu.write(0x2000 + addr % 8, value, &mut ppu_bus);
            }
            0x4000..=0x4017 => self.apu_io[a - 0x4000] = value,
            // Normally disabled APU and I/O functionality.
            0x4018..=0x401f => {}
            0x4020..=0xffff => {
                self.cart.write(addr, value).unwrap();
            }
        }

        #[cfg(feature = "debug")]
        {
            //(self.on_event)(EventSource::BusWrite{ addr, val: value });
        }
    }

    fn catch_up(&mut self, time: u128) {
        debug_assert!(time >= self.time);
        self.time = time;
    }

    fn debug_chr(&self) -> Option<[u8; 0x2000]> {
        self.cart.chr()
    }
}
