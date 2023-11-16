use crate::{Addr, Bus, Cpu};

use super::{
    status_register::{Sr, SrUpdate},
    AddrMode, Lut,
};

macro_rules! with_addressing_mode {
    ( $func_name:ident, $func_suffix:ident, $addr_mode:expr ) => {
        paste::item! {
            fn [< $func_name _ $func_suffix >] <B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
                let arg = cpu.read_arg(bus, $addr_mode);
                $func_name(cpu, bus, arg)
            }
        }
    };
}

macro_rules! with_addressing_mode_addr {
    ( $func_name:ident, $func_suffix:ident, $addr_mode:expr ) => {
        paste::item! {
            fn [< $func_name _ $func_suffix >] <B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
                let addr = cpu.read_addr(bus, $addr_mode);
                $func_name(cpu, bus, addr)
            }
        }
    };
}

macro_rules! todo_op {
    ( $msg:expr ) => {
        |_, _| todo!($msg)
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

            // Conditional branches
            0x90 => bcc,
            0xB0 => bcs,
            0xF0 => beq,
            0x30 => bmi,
            0xD0 => bne,
            0x10 => bpl,
            0x50 => bvc,
            0x70 => bvs,

            // BIT
            0x24 => bit_zeropage,
            0x2C => bit_absolute,

            // BRK
            0x00 => todo_op!("Too lazy to do break"),

            // Clears
            0x18 => clc,
            0xD8 => todo_op!("Clear decimal mode not supported"),
            0x58 => cli,
            0xB8 => clv,

            // CMP
            0xC9 => cmp_immediate,
            0xC5 => cmp_zeropage,
            0xD5 => cmp_indexedzx,
            0xCD => cmp_absolute,
            0xDD => cmp_absolutex,
            0xD9 => cmp_absolutey,
            0xC1 => cmp_indirectx,
            0xD1 => cmp_indirecty,

            // CPX
            0xE0 => cpx_immediate,
            0xE4 => cpx_zeropage,
            0xEC => cpx_absolute,

            // CPY
            0xC0 => cpy_immediate,
            0xC4 => cpy_zeropage,
            0xCC => cpy_absolute,

            // DEC
            0xC6 => dec_zeropage,
            0xD6 => dec_indexedzx,
            0xCE => dec_absolute,
            0xDE => dec_absolutex,
            
            // DEX
            0xCA => dex,

            // DEY
            0x88 => dey,

            // EOR
            0x49 => eor_immediate,
            0x45 => eor_zeropage,
            0x55 => eor_indexedzx,
            0x4D => eor_absolute,
            0x5D => eor_absolutex,
            0x59 => eor_absolutey,
            0x41 => eor_indirectx,
            0x51 => eor_indirecty,

            // INC
            0xE6 => inc_zeropage,
            0xF6 => inc_indexedzx,
            0xEE => inc_absolute,
            0xFE => inc_absolutex,

            // INX
            0xE8 => inx,

            // INY
            0xC8 => iny,

            // JMP
            0x4C => jmp_absolute,
            0x6C => jmp_indirect,

            // JSR
            0x20 => jsr,

            // LDA
            0xA9 => lda_immediate,
            0xA5 => lda_zeropage,
            0xB5 => lda_indexedzx,
            0xAD => lda_absolute,
            0xBD => lda_absolutex,
            0xB9 => lda_absolutey,
            0xA1 => lda_indirectx,
            0xB1 => lda_indirecty,

            // LDX
            0xA2 => ldx_immediate,
            0xA6 => ldx_zeropage,
            0xB6 => ldx_indexedzy,
            0xAE => ldx_absolute,
            0xBE => ldx_absolutey,

            // LDY
            0xA0 => ldy_immediate,
            0xA4 => ldy_zeropage,
            0xB4 => ldy_indexedzx,
            0xAC => ldy_absolute,
            0xBC => ldy_absolutex,

            // LSR
            0x4A => lsr_acc,
            0x46 => lsr_zeropage,
            0x56 => lsr_indexedzx,
            0x4E => lsr_absolute,
            0x5E => lsr_absolutex,

            // NOP
            0xEA => nop_implied,

            // ORA
            0x09 => ora_immediate,
            0x05 => ora_zeropage,
            0x15 => ora_indexedzx,
            0x0D => ora_absolute,
            0x1D => ora_absolutex,
            0x19 => ora_absolutey,
            0x01 => ora_indirectx,
            0x11 => ora_indirecty,

            // PHA
            0x48 => pha,

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

with_addressing_mode!(nop, immediate, AddrMode::Immediate);
with_addressing_mode!(nop, zeropage, AddrMode::ZeroPage);
with_addressing_mode!(nop, indexedzx, AddrMode::IndexedZX);
with_addressing_mode!(nop, absolute, AddrMode::Absolute);
with_addressing_mode!(nop, absolutex, AddrMode::AbsoluteX { force_cycle: false });

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

with_addressing_mode!(adc, immediate, AddrMode::Immediate);
with_addressing_mode!(adc, zeropage, AddrMode::ZeroPage);
with_addressing_mode!(adc, indexedzx, AddrMode::IndexedZX);
with_addressing_mode!(adc, absolute, AddrMode::Absolute);
with_addressing_mode!(adc, absolutex, AddrMode::AbsoluteX { force_cycle: false });
with_addressing_mode!(adc, absolutey, AddrMode::AbsoluteY { force_cycle: false });
with_addressing_mode!(adc, indirectx, AddrMode::IndirectX);
with_addressing_mode!(adc, indirecty, AddrMode::IndirectY);

fn and<B: Bus>(cpu: &mut Cpu<B>, _: &mut B, arg: u8) {
    cpu.a &= arg;
    cpu.update_flags(
        SrUpdate {
            ..SrUpdate::num_flags(cpu.a)
        }
        .result(),
    );
}

with_addressing_mode!(and, immediate, AddrMode::Immediate);
with_addressing_mode!(and, zeropage, AddrMode::ZeroPage);
with_addressing_mode!(and, indexedzx, AddrMode::IndexedZX);
with_addressing_mode!(and, absolute, AddrMode::Absolute);
with_addressing_mode!(and, absolutex, AddrMode::AbsoluteX { force_cycle: false });
with_addressing_mode!(and, absolutey, AddrMode::AbsoluteY { force_cycle: false });
with_addressing_mode!(and, indirectx, AddrMode::IndirectX);
with_addressing_mode!(and, indirecty, AddrMode::IndirectY);

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

with_addressing_mode_addr!(asl, zeropage, AddrMode::ZeroPage);
with_addressing_mode_addr!(asl, indexedzx, AddrMode::IndexedZX);
with_addressing_mode_addr!(asl, absolute, AddrMode::Absolute);
with_addressing_mode_addr!(asl, absolutex, AddrMode::AbsoluteX { force_cycle: true });

type CheckBranch<B> = fn(cpu: &Cpu<B>) -> bool;

// Might be buggy, but passes the tom harte tests
fn conditional_branch<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B, check_branch: CheckBranch<B>) {
    let offset = cpu.read_from_pc(bus) as i8;
    if check_branch(cpu) {
        cpu.prefetch(bus);
        let [mut low, high] = cpu.pc.to_le_bytes();
        low = low.wrapping_add_signed(offset);
        let mid_pc = u16::from_le_bytes([low, high]);
        let new_pc = cpu.pc.wrapping_add_signed(offset.into());

        if cpu.pc & 0xff00 != new_pc & 0xff00 {
            bus.read(mid_pc);
        }

        cpu.pc = new_pc;
    }
}

fn bcc<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    conditional_branch(cpu, bus, |cpu| !cpu.get_flag(Sr::C));
}

fn bcs<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    conditional_branch(cpu, bus, |cpu| cpu.get_flag(Sr::C));
}

fn beq<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    conditional_branch(cpu, bus, |cpu| cpu.get_flag(Sr::Z));
}

fn bmi<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    conditional_branch(cpu, bus, |cpu| cpu.get_flag(Sr::N));
}

fn bne<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    conditional_branch(cpu, bus, |cpu| !cpu.get_flag(Sr::Z));
}

fn bpl<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    conditional_branch(cpu, bus, |cpu| !cpu.get_flag(Sr::N));
}

fn bvc<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    conditional_branch(cpu, bus, |cpu| !cpu.get_flag(Sr::V));
}

fn bvs<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    conditional_branch(cpu, bus, |cpu| cpu.get_flag(Sr::V));
}

fn bit<B: Bus>(cpu: &mut Cpu<B>, _: &mut B, arg: u8) {
    cpu.sr = cpu.sr & 0x3f | arg & 0xc0;
    cpu.set_flag(Sr::Z, cpu.a & arg == 0);
}

with_addressing_mode!(bit, zeropage, AddrMode::ZeroPage);
with_addressing_mode!(bit, absolute, AddrMode::Absolute);

// Not gonna deal with this one yet
fn brk() {}

fn clear<B: Bus>(cpu: &mut Cpu<B>, flag: Sr) {
    cpu.set_flag(flag, false);
}

fn clc<B: Bus>(cpu: &mut Cpu<B>, _: &mut B) {
    clear(cpu, Sr::C);
}

fn cli<B: Bus>(cpu: &mut Cpu<B>, _: &mut B) {
    clear(cpu, Sr::I);
}

fn clv<B: Bus>(cpu: &mut Cpu<B>, _: &mut B) {
    clear(cpu, Sr::V);
}

fn cmp<B: Bus>(cpu: &mut Cpu<B>, _: &mut B, arg: u8) {
    let (res, c) = cpu.a.overflowing_sub(arg);
    cpu.update_flags(
        SrUpdate {
            c: Some(!c),
            ..SrUpdate::num_flags(res)
        }
        .result(),
    )
}

with_addressing_mode!(cmp, immediate, AddrMode::Immediate);
with_addressing_mode!(cmp, zeropage, AddrMode::ZeroPage);
with_addressing_mode!(cmp, indexedzx, AddrMode::IndexedZX);
with_addressing_mode!(cmp, absolute, AddrMode::Absolute);
with_addressing_mode!(cmp, absolutex, AddrMode::AbsoluteX { force_cycle: false });
with_addressing_mode!(cmp, absolutey, AddrMode::AbsoluteY { force_cycle: false });
with_addressing_mode!(cmp, indirectx, AddrMode::IndirectX);
with_addressing_mode!(cmp, indirecty, AddrMode::IndirectY);

fn cpx<B: Bus>(cpu: &mut Cpu<B>, _: &mut B, arg: u8) {
    let (res, c) = cpu.x.overflowing_sub(arg);
    cpu.update_flags(
        SrUpdate {
            c: Some(!c),
            ..SrUpdate::num_flags(res)
        }
        .result(),
    )
}

with_addressing_mode!(cpx, immediate, AddrMode::Immediate);
with_addressing_mode!(cpx, zeropage, AddrMode::ZeroPage);
with_addressing_mode!(cpx, absolute, AddrMode::Absolute);

fn cpy<B: Bus>(cpu: &mut Cpu<B>, _: &mut B, arg: u8) {
    let (res, c) = cpu.y.overflowing_sub(arg);
    cpu.update_flags(
        SrUpdate {
            c: Some(!c),
            ..SrUpdate::num_flags(res)
        }
        .result(),
    )
}

with_addressing_mode!(cpy, immediate, AddrMode::Immediate);
with_addressing_mode!(cpy, zeropage, AddrMode::ZeroPage);
with_addressing_mode!(cpy, absolute, AddrMode::Absolute);

fn dec<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B, addr: Addr) {
    let val = bus.read(addr);
    bus.write(addr, val);
    let res = val.wrapping_sub(1);
    bus.write(addr, res);

    cpu.update_flags(SrUpdate::num_flags(res).result());
}

with_addressing_mode_addr!(dec, zeropage, AddrMode::ZeroPage);
with_addressing_mode_addr!(dec, indexedzx, AddrMode::IndexedZX);
with_addressing_mode_addr!(dec, absolute, AddrMode::Absolute);
with_addressing_mode_addr!(dec, absolutex, AddrMode::AbsoluteX { force_cycle: true });

fn dex<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.x = cpu.x.wrapping_sub(1);
    cpu.update_flags(SrUpdate::num_flags(cpu.x).result());
    cpu.prefetch(bus);
}

fn dey<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.y = cpu.y.wrapping_sub(1);
    cpu.update_flags(SrUpdate::num_flags(cpu.y).result());
    cpu.prefetch(bus);
}

fn eor<B: Bus>(cpu: &mut Cpu<B>, _: &mut B, arg: u8) {
    cpu.a = cpu.a ^ arg;
    cpu.update_flags(SrUpdate::num_flags(cpu.a).result());
}

with_addressing_mode!(eor, immediate, AddrMode::Immediate);
with_addressing_mode!(eor, zeropage, AddrMode::ZeroPage);
with_addressing_mode!(eor, indexedzx, AddrMode::IndexedZX);
with_addressing_mode!(eor, absolute, AddrMode::Absolute);
with_addressing_mode!(eor, absolutex, AddrMode::AbsoluteX { force_cycle: false });
with_addressing_mode!(eor, absolutey, AddrMode::AbsoluteY { force_cycle: false });
with_addressing_mode!(eor, indirectx, AddrMode::IndirectX);
with_addressing_mode!(eor, indirecty, AddrMode::IndirectY);

fn inc<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B, addr: Addr) {
    let val = bus.read(addr);
    bus.write(addr, val);
    let res = val.wrapping_add(1);
    bus.write(addr, res);

    cpu.update_flags(SrUpdate::num_flags(res).result());
}

with_addressing_mode_addr!(inc, zeropage, AddrMode::ZeroPage);
with_addressing_mode_addr!(inc, indexedzx, AddrMode::IndexedZX);
with_addressing_mode_addr!(inc, absolute, AddrMode::Absolute);
with_addressing_mode_addr!(inc, absolutex, AddrMode::AbsoluteX { force_cycle: true });

fn inx<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.x = cpu.x.wrapping_add(1);
    cpu.update_flags(SrUpdate::num_flags(cpu.x).result());
    cpu.prefetch(bus);
}

fn iny<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.y = cpu.y.wrapping_add(1);
    cpu.update_flags(SrUpdate::num_flags(cpu.y).result());
    cpu.prefetch(bus);
}

fn jmp<B: Bus>(cpu: &mut Cpu<B>, _: &mut B, addr: Addr) {
    cpu.pc = addr;
}

with_addressing_mode_addr!(jmp, absolute, AddrMode::Absolute);
with_addressing_mode_addr!(jmp, indirect, AddrMode::Indirect);

fn jsr<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    let low = cpu.read_from_pc(bus);
    cpu.stack_peek(bus);
    cpu.stack_push(bus, (cpu.pc >> 8).try_into().unwrap());
    cpu.stack_push(bus, (cpu.pc & 0xff).try_into().unwrap());
    let high = cpu.read_from_pc(bus);
    cpu.pc = u16::from_le_bytes([low, high]);
}

fn lda<B: Bus>(cpu: &mut Cpu<B>, _: &mut B, arg: u8) {
    cpu.a = arg;
    cpu.update_flags(SrUpdate::num_flags(cpu.a).result());
}

with_addressing_mode!(lda, immediate, AddrMode::Immediate);
with_addressing_mode!(lda, zeropage, AddrMode::ZeroPage);
with_addressing_mode!(lda, indexedzx, AddrMode::IndexedZX);
with_addressing_mode!(lda, absolute, AddrMode::Absolute);
with_addressing_mode!(lda, absolutex, AddrMode::AbsoluteX { force_cycle: false });
with_addressing_mode!(lda, absolutey, AddrMode::AbsoluteY { force_cycle: false });
with_addressing_mode!(lda, indirectx, AddrMode::IndirectX);
with_addressing_mode!(lda, indirecty, AddrMode::IndirectY);

fn ldx<B: Bus>(cpu: &mut Cpu<B>, _: &mut B, arg: u8) {
    cpu.x = arg;
    cpu.update_flags(SrUpdate::num_flags(cpu.x).result());
}

with_addressing_mode!(ldx, immediate, AddrMode::Immediate);
with_addressing_mode!(ldx, zeropage, AddrMode::ZeroPage);
with_addressing_mode!(ldx, indexedzy, AddrMode::IndexedZY);
with_addressing_mode!(ldx, absolute, AddrMode::Absolute);
with_addressing_mode!(ldx, absolutey, AddrMode::AbsoluteY { force_cycle: false });

fn ldy<B: Bus>(cpu: &mut Cpu<B>, _: &mut B, arg: u8) {
    cpu.y = arg;
    cpu.update_flags(SrUpdate::num_flags(cpu.y).result());
}

with_addressing_mode!(ldy, immediate, AddrMode::Immediate);
with_addressing_mode!(ldy, zeropage, AddrMode::ZeroPage);
with_addressing_mode!(ldy, indexedzx, AddrMode::IndexedZX);
with_addressing_mode!(ldy, absolute, AddrMode::Absolute);
with_addressing_mode!(ldy, absolutex, AddrMode::AbsoluteX { force_cycle: false });

fn lsr<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B, addr: Addr) {
    let arg = bus.read(addr);
    let cout = arg & 1 == 1;
    let res = arg >> 1;
    bus.write(addr, arg);
    bus.write(addr, res);
    cpu.update_flags(
        SrUpdate {
            n: Some(false),
            z: Some(res == 0),
            c: Some(cout),
            ..SrUpdate::default()
        }
        .result(),
    );
}

fn lsr_acc<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.prefetch(bus);
    let cout = cpu.a & 1 == 1;
    cpu.a = cpu.a >> 1;
    cpu.update_flags(
        SrUpdate {
            n: Some(false),
            z: Some(cpu.a == 0),
            c: Some(cout),
            ..SrUpdate::default()
        }
        .result(),
    );
}

with_addressing_mode_addr!(lsr, zeropage, AddrMode::ZeroPage);
with_addressing_mode_addr!(lsr, indexedzx, AddrMode::IndexedZX);
with_addressing_mode_addr!(lsr, absolute, AddrMode::Absolute);
with_addressing_mode_addr!(lsr, absolutex, AddrMode::AbsoluteX { force_cycle: true });

fn ora<B: Bus>(cpu: &mut Cpu<B>, _: &mut B, arg: u8) {
    cpu.a = cpu.a | arg;
    cpu.update_flags(SrUpdate::num_flags(cpu.a).result());
}

with_addressing_mode!(ora, immediate, AddrMode::Immediate);
with_addressing_mode!(ora, zeropage, AddrMode::ZeroPage);
with_addressing_mode!(ora, indexedzx, AddrMode::IndexedZX);
with_addressing_mode!(ora, absolute, AddrMode::Absolute);
with_addressing_mode!(ora, absolutex, AddrMode::AbsoluteX { force_cycle: false });
with_addressing_mode!(ora, absolutey, AddrMode::AbsoluteY { force_cycle: false });
with_addressing_mode!(ora, indirectx, AddrMode::IndirectX);
with_addressing_mode!(ora, indirecty, AddrMode::IndirectY);

fn pha<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.prefetch(bus);
    cpu.stack_push(bus, cpu.a);
}
