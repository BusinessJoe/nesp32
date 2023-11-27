extern crate alloc;

use crate::{Addr, bus::PpuBusWithCart, cart::Cart};

#[cfg(feature = "debug")]
use crate::nes::{EventCb, default_event_cb};

pub enum PixelSource {
    Screen,
    Left,
}

pub type Color = (u8, u8, u8);

pub trait Screen {
    fn put_pixel(&mut self, row: u16, col: u16, c: Color);
}

pub struct Ppu<S: Screen> {
    regs: [u8; 0x8],
    oam: [u8; 0x100],
    ppu_ctrl: u8,
    ppu_status: u8,
    ppu_addr: u16,
    addr_latch: Option<u8>,
    screen: S,
    time: u128,

    #[cfg(feature = "debug")]
    on_event: EventCb,
}

impl<S: Screen> Ppu<S> {
    pub fn new(screen: S) -> Self {
        Self {
            regs: [0; 0x8],
            oam: [0; 0x100],
            ppu_ctrl: 0,
            ppu_status: 0,
            ppu_addr: 0,
            addr_latch: None,
            screen,
            time: 0,

            #[cfg(feature = "debug")]
            on_event: default_event_cb()
        }
    }

    pub fn catch_up(&mut self, time: u128) {
        debug_assert!(time >= self.time);
        let elapsed = time - self.time;
        self.time = time;

        for _ in 0..elapsed {
            self.tick();
        }
    }

    pub fn read<'a, C: Cart>(&mut self, addr: Addr, ppu_bus: &mut PpuBusWithCart<'a, C>) -> u8 {
        match addr {
            0x2002 => {
                self.addr_latch = None;
                self.ppu_status
            }
            0x2007 => {
                let inc: u16 = if self.ppu_ctrl & 0b100 == 0 { 1 } else { 32 };
                let val = ppu_bus.read(self.ppu_addr);
                self.ppu_addr = self.ppu_addr.wrapping_add(inc);
                val
            }
            0x2000..=0x2007 => self.regs[usize::from(addr - 0x2000)],
            _ => unreachable!(),
        }
    }

    pub fn write<'a, C: Cart>(&mut self, addr: Addr, val: u8, ppu_bus: &mut PpuBusWithCart<'a, C>) {
        match addr {
            0x2000 => self.ppu_ctrl = val,
            0x2006 => match self.addr_latch {
                None => self.addr_latch = Some(val),
                Some(x) => {
                    let address: u16 = u16::from(x) << 8 | u16::from(val);
                    self.ppu_addr = address % 0x4000;
                }
            },
            0x2007 => {
                let inc: u16 = if self.ppu_ctrl & 0b100 == 0 { 1 } else { 32 };
                ppu_bus.write(self.ppu_addr, val);
                self.ppu_addr = self.ppu_addr.wrapping_add(inc);
            }
            0x2001..=0x2007 => self.regs[usize::from(addr - 0x2000)] = val,
            _ => unreachable!(),
        }
    }

    fn tick(&mut self) {
        self.ppu_status = 0b1000_0000;
    }
}

