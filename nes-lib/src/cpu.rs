mod instrs;
mod status_register;

use crate::Bus;

use self::instrs::generate_lut;

#[derive(Debug)]
pub struct Cpu<B: Bus> {
    pub pc: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sr: u8,
    pub sp: u8,

    lut: [InstrFp<B>; 256],
}

type InstrFp<B> = fn(cpu: &mut Cpu<B>, bus: &mut B);
type Lut<B> = [InstrFp<B>; 256];

pub enum AddrMode {
    Immediate,
    Absolute,
    ZeroPage,
    AbsoluteX,
    AbsoluteY,
    IndexedZX,
    IndexedZY,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
}

impl<B: Bus> Cpu<B> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self, bus: &mut B) {
        // Fetch opcode.
        let opcode = self.read_from_pc(bus);

        // Execute opcode.
        let fp = self.lut[usize::from(opcode)];
        fp(self, bus);
    }

    pub fn read_arg(&mut self, bus: &mut B, addr_mode: AddrMode) -> u8 {
        match addr_mode {
            AddrMode::Immediate => self.read_from_pc(bus),
            AddrMode::Absolute => {
                let addr = u16::from_le_bytes([self.read_from_pc(bus), self.read_from_pc(bus)]);
                bus.read(addr)
            }
            AddrMode::AbsoluteX => {
                let addr = u16::from_le_bytes([self.read_from_pc(bus), self.read_from_pc(bus)]);
                let new_addr = addr.wrapping_add(self.x.into());
                // Crossing page boundary
                if addr >> 8 != new_addr >> 8 {
                    bus.read(addr & 0xff00 | new_addr & 0xff);
                }
                bus.read(new_addr)
            }
            AddrMode::AbsoluteY => {
                let addr = u16::from_le_bytes([self.read_from_pc(bus), self.read_from_pc(bus)]);
                let new_addr = addr.wrapping_add(self.y.into());
                // Crossing page boundary
                if addr >> 8 != new_addr >> 8 {
                    bus.read(addr & 0xff00 | new_addr & 0xff);
                }
                bus.read(new_addr)
            }
            AddrMode::ZeroPage => {
                let addr = u16::from(self.read_from_pc(bus));
                bus.read(addr)
            }
            AddrMode::IndexedZX => {
                let addr = u16::from(self.read_from_pc(bus));
                bus.read(addr);
                bus.read((addr + u16::from(self.x)) % 256)
            }
            AddrMode::IndexedZY => {
                let addr = u16::from(self.read_from_pc(bus));
                bus.read(addr);
                bus.read((addr + u16::from(self.y)) % 256)
            }
            AddrMode::IndirectX => {
                let base = self.read_from_pc(bus);
                bus.read(base.into());
                let addr = base.wrapping_add(self.x);
                let low = bus.read(addr.into());
                let high = bus.read(addr.wrapping_add(1).into());
                let addr = u16::from_le_bytes([low, high]);
                bus.read(addr)
            }
            AddrMode::IndirectY => {
                let addr = self.read_from_pc(bus);
                let low = bus.read(addr.into());
                let high = bus.read(addr.wrapping_add(1).into());
                let addr = u16::from_le_bytes([low, high]);
                let new_addr = addr.wrapping_add(self.y.into());

                // Crossing page boundary
                if u16::from(addr) >> 8 != new_addr >> 8 {
                    bus.read(addr & 0xff00 | new_addr & 0xff);
                }

                bus.read(new_addr)
            }
            _ => todo!(),
        }
    }

    pub fn prefetch(&self, bus: &mut B) {
        bus.read(self.pc);
    }

    fn read_from_pc(&mut self, bus: &mut B) -> u8 {
        let val = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        val
    }
}

impl<B: Bus> Default for Cpu<B> {
    fn default() -> Self {
        Self {
            pc: 0,
            a: 0,
            x: 0,
            y: 0,
            sr: 0,
            sp: 0,
            lut: generate_lut(),
        }
    }
}
