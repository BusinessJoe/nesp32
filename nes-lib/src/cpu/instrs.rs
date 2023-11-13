use crate::{Cpu, Bus};

use super::{Lut, AddrMode};


pub const fn generate_lut<B: Bus>() -> Lut<B> {
    let mut lut: Lut<B> = [panic_fp; 256];
    let mut i: usize = 0;
    while i <= 255 {
        lut[i] = match i as u8 {
            0xEA => nop_implied,
            // Illegal NOPs
            0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => nop_implied,
            0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 => nop_immediate,
            0x04 | 0x44 | 0x64 => nop_zeropage,
            0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 => nop_zeropage_x,
            0x0C => nop_absolute,
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => nop_absolute_x,
            0 ..= 255 => panic_fp,
        };
        i += 1;
    }
    lut
}

fn panic_fp<B: Bus>(_: &mut Cpu<B>, _: &mut B) {
    panic!()
}

fn nop_implied<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.prefetch(bus)
}

fn nop_immediate<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.read_arg(bus, AddrMode::Immediate);
}

fn nop_zeropage<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.read_arg(bus, AddrMode::ZeroPage);
}

fn nop_zeropage_x<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.read_arg(bus, AddrMode::ZeroPageX);
}

fn nop_absolute<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.read_arg(bus, AddrMode::Absolute);
}

fn nop_absolute_x<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.read_arg(bus, AddrMode::AbsoluteX);
}

