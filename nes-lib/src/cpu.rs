mod instrs;
use crate::Bus;

use self::instrs::generate_lut;

#[derive(Debug)]
pub struct Cpu<B: Bus> {
    pub pc: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub s: u8,
    pub p: u8,

    lut: [InstrFp<B>; 256],
}

type InstrFp<B> = fn (cpu: &mut Cpu<B>, bus: &mut B);
type Lut<B> = [InstrFp<B>; 256];

pub enum AddrMode {
    Immediate,
    Absolute,
    ZeroPage,
    AbsoluteX,
    AbsoluteY,
    ZeroPageX,
    ZeroPageY,
    Indirect,
    PreIndex,
    PostIndex,
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
            AddrMode::Immediate => {
                self.read_from_pc(bus)
            }
            AddrMode::Absolute => {
                let addr = u16::from_le_bytes([
                    self.read_from_pc(bus),
                    self.read_from_pc(bus),
                ]);
                bus.read(addr)
            }
            AddrMode::ZeroPage => {
                let addr = u16::from(self.read_from_pc(bus));
                bus.read(addr)
            }
            AddrMode::AbsoluteX => {
                let addr = u16::from_le_bytes([
                    self.read_from_pc(bus),
                    self.read_from_pc(bus),
                ]);
                bus.read(addr.wrapping_add(self.x.into()))
            }
            AddrMode::AbsoluteY => {
                let addr = u16::from_le_bytes([
                    self.read_from_pc(bus),
                    self.read_from_pc(bus),
                ]);
                bus.read(addr.wrapping_add(self.y.into()))
            }
            AddrMode::ZeroPageX => {
                let addr = u16::from(self.read_from_pc(bus));
                bus.read(addr.wrapping_add(self.x.into()))
            }
            AddrMode::ZeroPageY => {
                let addr = u16::from(self.read_from_pc(bus));
                bus.read(addr.wrapping_add(self.y.into()))
            }
            _ => panic!()
        }
    }

    fn read_from_pc(&mut self, bus: &mut B) -> u8 {
        let val = bus.read(self.pc);
        self.pc += 1;
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
            s: 0,
            p: 0,
            lut: generate_lut(),
        }
    }
}
