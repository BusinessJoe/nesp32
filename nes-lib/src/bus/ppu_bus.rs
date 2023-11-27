extern crate alloc;

use crate::{
    cart::{Cart, DeferredRead, DeferredWrite},
    Addr,
};

#[cfg(feature = "debug")]
use crate::nes::{default_event_cb, EventCb};

/// Memory only directly accessable by the PPU.
pub struct PpuBus {
    pub vram: [u8; 0x800],
}

/// Routes reads and writes through the cartridge before hitting the PPU bus.
pub struct PpuBusWithCart<'a, C: Cart> {
    pub cart: &'a mut C,
    pub ppu_bus: &'a mut PpuBus,

    #[cfg(feature = "debug")]
    pub on_event: EventCb,
}

impl PpuBus {
    pub fn with_cart<'a, C: Cart>(&'a mut self, cart: &'a mut C) -> PpuBusWithCart<C> {
        PpuBusWithCart {
            cart,
            ppu_bus: self,

            #[cfg(feature = "debug")]
            on_event: default_event_cb(),
        }
    }
}

impl<'a, C: Cart> PpuBusWithCart<'a, C> {
    pub fn read(&mut self, addr: Addr) -> u8 {
        match self.cart.read(addr) {
            Ok(x) => x,
            Err(deferred) => self.deferred_read(deferred),
        }
    }

    pub fn write(&mut self, addr: Addr, val: u8) {
        if let Some(deferred) = self.cart.write(addr, val) {
            self.deferred_write(deferred)
        }
    }

    fn deferred_read(&self, deferred: DeferredRead) -> u8 {
        match deferred {
            DeferredRead::VRAM(addr) => {
                let val = self.ppu_bus.vram[addr];
                #[cfg(feature = "debug")]
                {
                    //(self.on_event)(EventSource::VramRead { addr, val })
                }
                val
            }
        }
    }

    fn deferred_write(&mut self, deferred: DeferredWrite) {
        match deferred {
            DeferredWrite::VRAM(addr, val) => {
                #[cfg(feature = "debug")]
                {
                    //(self.on_event)(EventSource::VramWrite { addr, val })
                }
                self.ppu_bus.vram[addr] = val
            }
        }
    }
}
