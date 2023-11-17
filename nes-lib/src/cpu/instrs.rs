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

            // PHP
            0x08 => php,

            // PLA
            0x68 => pla,

            // PLP
            0x28 => plp,

            // ROL
            0x2A => rol_acc,
            0x26 => rol_zeropage,
            0x36 => rol_indexedzx,
            0x2E => rol_absolute,
            0x3E => rol_absolutex,

            // ROR
            0x6A => ror_acc,
            0x66 => ror_zeropage,
            0x76 => ror_indexedzx,
            0x6E => ror_absolute,
            0x7E => ror_absolutex,

            // RTI
            0x40 => rti,

            // RTS
            0x60 => rts,

            // SBC
            0xE9 => sbc_immediate,
            0xE5 => sbc_zeropage,
            0xF5 => sbc_indexedzx,
            0xED => sbc_absolute,
            0xFD => sbc_absolutex,
            0xF9 => sbc_absolutey,
            0xE1 => sbc_indirectx,
            0xF1 => sbc_indirecty,

            // Sets
            0x38 => sec,
            0xF8 => todo_op!("Set decimal flag"),
            0x78 => sei,

            // STA
            0x85 => sta_zeropage,
            0x95 => sta_indexedzx,
            0x8D => sta_absolute,
            0x9D => sta_absolutex,
            0x99 => sta_absolutey,
            0x81 => sta_indirectx,
            0x91 => sta_indirecty,

            // STX
            0x86 => stx_zeropage,
            0x96 => stx_indexedzy,
            0x8E => stx_absolute,

            // STY
            0x84 => sty_zeropage,
            0x94 => sty_indexedzx,
            0x8C => sty_absolute,

            // Transfers
            0xAA => tax,
            0xA8 => tay,
            0xBA => tsx,
            0x8A => txa,
            0x9A => txs,
            0x98 => tya,

            /* Here be monsters */

            // Illegal NOPs
            0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => nop_implied,
            0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 => nop_immediate,
            0x04 | 0x44 | 0x64 => nop_zeropage,
            0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 => nop_indexedzx,
            0x0C => nop_absolute,
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => nop_absolutex,

            // JAM
            0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xB2 | 0xD2 | 0xF2 => jam,

            // ALR
            0x4B => alr,

            // ANC
            0x0B | 0x2B => anc,

            // ANE
            0x8B => todo_op!("Cursed ANE/XAA opcode"),
            
            // ARR
            0x6B => arr,

            // DCP
            0xC7 => dcp_zeropage,
            0xD7 => dcp_indexedzx,
            0xCF => dcp_absolute,
            0xDF => dcp_absolutex,
            0xDB => dcp_absolutey,
            0xC3 => dcp_indirectx,
            0xD3 => dcp_indirecty,

            // SLO
            0x07 => slo_zeropage,
            0x17 => slo_indexedzx,
            0x0F => slo_absolute,
            0x1F => slo_absolutex,
            0x1B => slo_absolutey,
            0x03 => slo_indirectx,
            0x13 => slo_indirecty,

            // ISC
            0xE7 => isc_zeropage,
            0xF7 => isc_indexedzx,
            0xEF => isc_absolute,
            0xFF => isc_absolutex,
            0xFB => isc_absolutey,
            0xE3 => isc_indirectx,
            0xF3 => isc_indirecty,

            _ => panic_fp,
        };
        i += 1;
    }
    lut
}

// Utility function for overflow flag
fn add_overflows(a1: u8, a2: u8, res: u8) -> bool {
    (a1 & a2) >> 7 == 1 && (a1 ^ res) >> 7 == 1
}

// Utility function for overflow flag
fn sub_overflows(a1: u8, a2: u8, res: u8) -> bool {
    (a1 ^ a2) >> 7 == 1 && (a1 ^ res) >> 7 == 1
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

fn php<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.prefetch(bus);
    cpu.stack_push(bus, cpu.sr | 0b0011_0000);
}

fn pla<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.prefetch(bus);
    cpu.a = cpu.stack_pop(bus);
    cpu.update_flags(SrUpdate::num_flags(cpu.a).result());
}

fn plp<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.prefetch(bus);
    cpu.sr = cpu.stack_pop(bus) & 0b1100_1111;
}

fn rol<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B, addr: Addr) {
    let arg = bus.read(addr);
    let cin = cpu.get_flag(Sr::C);
    let cout = arg >> 7 == 1;
    let mut res = arg.wrapping_shl(1);
    if cin {
        res += 1
    }
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

fn rol_acc<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.prefetch(bus);
    let cin = cpu.get_flag(Sr::C);
    let cout = cpu.a >> 7 == 1;
    cpu.a = cpu.a.wrapping_shl(1);
    if cin {
        cpu.a += 1;
    }
    cpu.update_flags(
        SrUpdate {
            c: Some(cout),
            ..SrUpdate::num_flags(cpu.a)
        }
        .result(),
    );
}

with_addressing_mode_addr!(rol, zeropage, AddrMode::ZeroPage);
with_addressing_mode_addr!(rol, indexedzx, AddrMode::IndexedZX);
with_addressing_mode_addr!(rol, absolute, AddrMode::Absolute);
with_addressing_mode_addr!(rol, absolutex, AddrMode::AbsoluteX { force_cycle: true });

fn ror<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B, addr: Addr) {
    let arg = bus.read(addr);
    let cin = cpu.get_flag(Sr::C);
    let cout = arg & 1 == 1;
    let mut res = arg >> 1;
    if cin {
        res |= 1 << 7;
    }
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

fn ror_acc<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.prefetch(bus);
    let cin = cpu.get_flag(Sr::C);
    let cout = cpu.a & 1 == 1;
    cpu.a = cpu.a >> 1;
    if cin {
        cpu.a |= 1 << 7;
    }
    cpu.update_flags(
        SrUpdate {
            c: Some(cout),
            ..SrUpdate::num_flags(cpu.a)
        }
        .result(),
    );
}

with_addressing_mode_addr!(ror, zeropage, AddrMode::ZeroPage);
with_addressing_mode_addr!(ror, indexedzx, AddrMode::IndexedZX);
with_addressing_mode_addr!(ror, absolute, AddrMode::Absolute);
with_addressing_mode_addr!(ror, absolutex, AddrMode::AbsoluteX { force_cycle: true });

fn rti<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.prefetch(bus);
    cpu.stack_peek(bus);
    cpu.sr = cpu.stack_pop(bus) & 0b1100_1111 | 0b0010_0000;
    let low = cpu.stack_pop(bus);
    let high = cpu.stack_pop(bus);
    cpu.pc = u16::from_le_bytes([low, high]);
}

fn rts<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.prefetch(bus);
    cpu.stack_peek(bus);
    let low = cpu.stack_pop(bus);
    let high = cpu.stack_pop(bus);
    cpu.pc = u16::from_le_bytes([low, high]);
    cpu.prefetch(bus);
    cpu.pc = cpu.pc.wrapping_add(1);
}

fn sbc<B: Bus>(cpu: &mut Cpu<B>, _: &mut B, arg: u8) {
    let prev = cpu.a;
    let cin = if cpu.get_flag(Sr::C) { 0 } else { 1 };

    let (tmp, c1) = prev.overflowing_sub(arg);
    let (res, c2) = tmp.overflowing_sub(cin);
    let v = sub_overflows(prev, arg, res);

    cpu.a = res;

    cpu.update_flags(
        SrUpdate {
            c: Some(!(c1 | c2)),
            v: Some(v),
            ..SrUpdate::num_flags(res)
        }
        .result(),
    );
}

with_addressing_mode!(sbc, immediate, AddrMode::Immediate);
with_addressing_mode!(sbc, zeropage, AddrMode::ZeroPage);
with_addressing_mode!(sbc, indexedzx, AddrMode::IndexedZX);
with_addressing_mode!(sbc, absolute, AddrMode::Absolute);
with_addressing_mode!(sbc, absolutex, AddrMode::AbsoluteX { force_cycle: false });
with_addressing_mode!(sbc, absolutey, AddrMode::AbsoluteY { force_cycle: false });
with_addressing_mode!(sbc, indirectx, AddrMode::IndirectX);
with_addressing_mode!(sbc, indirecty, AddrMode::IndirectY);

fn set<B: Bus>(cpu: &mut Cpu<B>, flag: Sr) {
    cpu.set_flag(flag, true);
}

fn sec<B: Bus>(cpu: &mut Cpu<B>, _: &mut B) {
    set(cpu, Sr::C);
}

fn sei<B: Bus>(cpu: &mut Cpu<B>, _: &mut B) {
    set(cpu, Sr::I);
}

fn sta<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B, addr: Addr) {
    bus.write(addr, cpu.a);
}

with_addressing_mode_addr!(sta, zeropage, AddrMode::ZeroPage);
with_addressing_mode_addr!(sta, indexedzx, AddrMode::IndexedZX);
with_addressing_mode_addr!(sta, absolute, AddrMode::Absolute);
with_addressing_mode_addr!(sta, absolutex, AddrMode::AbsoluteX { force_cycle: true });
with_addressing_mode_addr!(sta, absolutey, AddrMode::AbsoluteY { force_cycle: true });
with_addressing_mode_addr!(sta, indirectx, AddrMode::IndirectX);
with_addressing_mode_addr!(sta, indirecty, AddrMode::IndirectY);

fn stx<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B, addr: Addr) {
    bus.write(addr, cpu.x);
}

with_addressing_mode_addr!(stx, zeropage, AddrMode::ZeroPage);
with_addressing_mode_addr!(stx, indexedzy, AddrMode::IndexedZY);
with_addressing_mode_addr!(stx, absolute, AddrMode::Absolute);

fn sty<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B, addr: Addr) {
    bus.write(addr, cpu.y);
}

with_addressing_mode_addr!(sty, zeropage, AddrMode::ZeroPage);
with_addressing_mode_addr!(sty, indexedzx, AddrMode::IndexedZX);
with_addressing_mode_addr!(sty, absolute, AddrMode::Absolute);

fn tax<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.x = cpu.a;
    cpu.update_flags(SrUpdate::num_flags(cpu.x).result());
    cpu.prefetch(bus);
}

fn tay<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.y = cpu.a;
    cpu.update_flags(SrUpdate::num_flags(cpu.y).result());
    cpu.prefetch(bus);
}

fn tsx<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.x = cpu.sp;
    cpu.update_flags(SrUpdate::num_flags(cpu.x).result());
    cpu.prefetch(bus);
}

fn txa<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.a = cpu.x;
    cpu.update_flags(SrUpdate::num_flags(cpu.a).result());
    cpu.prefetch(bus);
}

fn txs<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.sp = cpu.x;
    cpu.update_flags(SrUpdate::num_flags(cpu.sp).result());
    cpu.prefetch(bus);
}

fn tya<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    cpu.a = cpu.y;
    cpu.update_flags(SrUpdate::num_flags(cpu.a).result());
    cpu.prefetch(bus);
}

/* Here be monsters */

fn jam<B: Bus>(cpu: &mut Cpu<B>, _: &mut B) {
    cpu.jam(); 
}

fn alr<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    let arg = cpu.read_arg(bus, AddrMode::Immediate);
    // AND
    cpu.a &= arg;

    // LSR
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

fn anc<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    let arg = cpu.read_arg(bus, AddrMode::Immediate);
    // AND
    cpu.a &= arg;

    let cout = cpu.a >> 7 == 1;
    cpu.update_flags(
        SrUpdate {
            c: Some(cout),
            ..SrUpdate::num_flags(cpu.a)
        }
        .result(),
    );
}

fn arr<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
    let arg = cpu.read_arg(bus, AddrMode::Immediate);
    // AND
    cpu.a &= arg;

    // ROR
    let cin = cpu.get_flag(Sr::C);
    let cout = cpu.a & 1 == 1;
    cpu.a = cpu.a >> 1;
    if cin {
        cpu.a |= 1 << 7;
    }
    cpu.update_flags(
        SrUpdate {
            c: Some(cout),
            ..SrUpdate::num_flags(cpu.a)
        }
        .result(),
    );
}

fn dcp<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B, addr: Addr) {
    let val = bus.read(addr);
    bus.write(addr, val);
    let res = val.wrapping_sub(1);
    bus.write(addr, res);

    let (res, c) = cpu.a.overflowing_sub(res);
    cpu.update_flags(
        SrUpdate {
            c: Some(!c),
            ..SrUpdate::num_flags(res)
        }
        .result(),
    )
}

with_addressing_mode_addr!(dcp, zeropage, AddrMode::ZeroPage);
with_addressing_mode_addr!(dcp, indexedzx, AddrMode::IndexedZX);
with_addressing_mode_addr!(dcp, absolute, AddrMode::Absolute);
with_addressing_mode_addr!(dcp, absolutex, AddrMode::AbsoluteX { force_cycle: true });
with_addressing_mode_addr!(dcp, absolutey, AddrMode::AbsoluteY { force_cycle: true });
with_addressing_mode_addr!(dcp, indirectx, AddrMode::IndirectX);
with_addressing_mode_addr!(dcp, indirecty, AddrMode::IndirectY);

fn isc<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B, addr: Addr) {
    let val = bus.read(addr);
    bus.write(addr, val);
    let res = val.wrapping_add(1);
    bus.write(addr, res);

    let prev = cpu.a;
    let cin = if cpu.get_flag(Sr::C) { 0 } else { 1 };

    let (tmp, c1) = prev.overflowing_sub(res);
    let (res2, c2) = tmp.overflowing_sub(cin);
    let v = sub_overflows(prev, res, res2);

    cpu.a = res2;

    cpu.update_flags(
        SrUpdate {
            c: Some(!(c1 | c2)),
            v: Some(v),
            ..SrUpdate::num_flags(res2)
        }
        .result(),
    );
}

with_addressing_mode_addr!(isc, zeropage, AddrMode::ZeroPage);
with_addressing_mode_addr!(isc, indexedzx, AddrMode::IndexedZX);
with_addressing_mode_addr!(isc, absolute, AddrMode::Absolute);
with_addressing_mode_addr!(isc, absolutex, AddrMode::AbsoluteX { force_cycle: true });
with_addressing_mode_addr!(isc, absolutey, AddrMode::AbsoluteY { force_cycle: true });
with_addressing_mode_addr!(isc, indirectx, AddrMode::IndirectX);
with_addressing_mode_addr!(isc, indirecty, AddrMode::IndirectY);

fn slo<B: Bus>(cpu: &mut Cpu<B>, bus: &mut B, addr: Addr) {
    let arg = bus.read(addr);
    let cout = arg >> 7 == 1;
    let res = arg << 1;
    bus.write(addr, arg);
    bus.write(addr, res);

    cpu.a = cpu.a | res;
    cpu.update_flags(
        SrUpdate {
            c: Some(cout),
            ..SrUpdate::num_flags(cpu.a)
        }
        .result(),
    );
}

with_addressing_mode_addr!(slo, zeropage, AddrMode::ZeroPage);
with_addressing_mode_addr!(slo, indexedzx, AddrMode::IndexedZX);
with_addressing_mode_addr!(slo, absolute, AddrMode::Absolute);
with_addressing_mode_addr!(slo, absolutex, AddrMode::AbsoluteX { force_cycle: true });
with_addressing_mode_addr!(slo, absolutey, AddrMode::AbsoluteY { force_cycle: true });
with_addressing_mode_addr!(slo, indirectx, AddrMode::IndirectX);
with_addressing_mode_addr!(slo, indirecty, AddrMode::IndirectY);
