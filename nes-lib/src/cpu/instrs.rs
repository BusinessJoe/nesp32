use crate::{Addr, Bus, Cpu};

use super::{
    status_register::{Sr, SrUpdate},
    AddrMode, Lut,
};

macro_rules! with_addressing_modes {
    ( $func_name:ident, $func_suffix:ident, $addr_mode:expr ) => {
        paste::item! {
            fn [< $func_name _ $func_suffix >] <B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
                let arg = cpu.read_arg(bus, $addr_mode);
                $func_name(cpu, bus, arg)
            }
        }
    };
}

macro_rules! with_addressing_modes_addr {
    ( $func_name:ident, $func_suffix:ident, $addr_mode:expr ) => {
        paste::item! {
            fn [< $func_name _ $func_suffix >] <B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
                let addr = cpu.read_addr(bus, $addr_mode);
                $func_name(cpu, bus, addr)
            }
        }
    };
}

pub const fn generate_lut<B: Bus>() -> Lut<B> {
    let mut lut: Lut<B> = [panic_fp; 256];
    let mut i: usize = 0;
    while i <= 255 {
        lut[i] = match i as u8 {
            // ADC
            0x69 => adc_immediate,
            0x65 => adc_zeropage,
            0x75 => adc_indexedzx,
            0x6D => adc_absolute,
            0x7D => adc_absolutex,
            0x79 => adc_absolutey,
            0x61 => adc_indirectx,
            0x71 => adc_indirecty,

            // AND
            0x29 => and_immediate,
            0x25 => and_zeropage,
            0x35 => and_indexedzx,
            0x2D => and_absolute,
            0x3D => and_absolutex,
            0x39 => and_absolutey,
            0x21 => and_indirectx,
            0x31 => and_indirecty,

            // ASL
            0x0A => asl_acc,
            0x06 => asl_zeropage,
            0x16 => asl_indexedzx,
            0x0E => asl_absolute,
            0x1E => asl_absolutex,

            0xEA => nop_implied,
            // Illegal NOPs
            0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => nop_implied,
            0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 => nop_immediate,
            0x04 | 0x44 | 0x64 => nop_zeropage,
            0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 => nop_indexedzx,
            0x0C => nop_absolute,
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => nop_absolutex,
            _ => panic_fp,
        };
        i += 1;
    }
    lut
}

// Utility function for overflow flag
fn add_overflows(a1: u8, a2: u8, res: u8) -> bool {
    let sign_acc = (a1 & 0b1000_0000) != 0;
    let sign_value = (a2 & 0b1000_0000) != 0;
    let sign_result = (res & 0b1000_0000) != 0;
    (sign_acc == sign_value) && (sign_acc != sign_result)
}

/* OPS */

fn panic_fp<B: Bus>(_: &mut Cpu<B>, _: &mut B) {
    panic!()
}

fn nop_implied<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.prefetch(bus)
}

fn nop<B: Bus>(_: &mut Cpu<B>, _: &mut B, _: u8) {}

with_addressing_modes!(nop, immediate, AddrMode::Immediate);
with_addressing_modes!(nop, zeropage, AddrMode::ZeroPage);
with_addressing_modes!(nop, indexedzx, AddrMode::IndexedZX);
with_addressing_modes!(nop, absolute, AddrMode::Absolute);
with_addressing_modes!(nop, absolutex, AddrMode::AbsoluteX { force_cycle: false });

fn adc<B: Bus>(cpu: &mut Cpu<B>, _: &mut B, arg: u8) {
    let prev = cpu.a;
    let cin = if cpu.get_flag(Sr::C) { 1 } else { 0 };

    let (tmp, c1) = prev.overflowing_add(arg);
    let (res, c2) = tmp.overflowing_add(cin);
    let v = add_overflows(prev, arg, res);

    cpu.a = res;

    cpu.update_flags(
        SrUpdate {
            c: Some(c1 | c2),
            v: Some(v),
            ..SrUpdate::num_flags(res)
        }
        .result(),
    );
}

with_addressing_modes!(adc, immediate, AddrMode::Immediate);
with_addressing_modes!(adc, zeropage, AddrMode::ZeroPage);
with_addressing_modes!(adc, indexedzx, AddrMode::IndexedZX);
with_addressing_modes!(adc, absolute, AddrMode::Absolute);
with_addressing_modes!(adc, absolutex, AddrMode::AbsoluteX { force_cycle: false });
with_addressing_modes!(adc, absolutey, AddrMode::AbsoluteY { force_cycle: false });
with_addressing_modes!(adc, indirectx, AddrMode::IndirectX);
with_addressing_modes!(adc, indirecty, AddrMode::IndirectY);

fn and<B: Bus>(cpu: &mut Cpu<B>, _: &mut B, arg: u8) {
    cpu.a &= arg;
    cpu.update_flags(
        SrUpdate {
            ..SrUpdate::num_flags(cpu.a)
        }
        .result(),
    );
}

with_addressing_modes!(and, immediate, AddrMode::Immediate);
with_addressing_modes!(and, zeropage, AddrMode::ZeroPage);
with_addressing_modes!(and, indexedzx, AddrMode::IndexedZX);
with_addressing_modes!(and, absolute, AddrMode::Absolute);
with_addressing_modes!(and, absolutex, AddrMode::AbsoluteX { force_cycle: false });
with_addressing_modes!(and, absolutey, AddrMode::AbsoluteY { force_cycle: false });
with_addressing_modes!(and, indirectx, AddrMode::IndirectX);
with_addressing_modes!(and, indirecty, AddrMode::IndirectY);

fn asl<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B, addr: Addr) {
    let arg = bus.read(addr);
    let cout = arg >> 7 == 1;
    let res = arg << 1;
    bus.write(addr, arg);
    bus.write(addr, res);
    cpu.update_flags(
        SrUpdate {
            c: Some(cout),
            ..SrUpdate::num_flags(res)
        }
        .result(),
    );
}

fn asl_acc<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.prefetch(bus);
    let cout = cpu.a >> 7 == 1;
    cpu.a = cpu.a << 1;
    cpu.update_flags(
        SrUpdate {
            c: Some(cout),
            ..SrUpdate::num_flags(cpu.a)
        }
        .result(),
    );
}

with_addressing_modes_addr!(asl, zeropage, AddrMode::ZeroPage);
with_addressing_modes_addr!(asl, indexedzx, AddrMode::IndexedZX);
with_addressing_modes_addr!(asl, absolute, AddrMode::Absolute);
with_addressing_modes_addr!(asl, absolutex, AddrMode::AbsoluteX { force_cycle: true });
