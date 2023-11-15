use crate::{Cpu, Bus};

use super::{Lut, AddrMode, status_register::{Sr, SrUpdate}};

macro_rules! with_addressing_modes {
    ( $func_name:ident, $( $addr_mode:ident ),* ) => {
        $(
            paste::item! {
                fn [< $func_name _ $addr_mode:lower >] <B: Bus>(cpu: &mut Cpu<B>, bus: &mut B) {
                    let arg = cpu.read_arg(bus, AddrMode::$addr_mode);
                    $func_name(cpu, bus, arg)
                }
            }
        )*
    }
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

fn nop<B: Bus>(_: &mut Cpu<B>, _: &mut B, _: u8) { }

with_addressing_modes!(nop, Immediate, ZeroPage, IndexedZX, Absolute, AbsoluteX);

fn adc<B: Bus>(cpu: &mut Cpu<B>, _: &mut B, arg: u8) {
    let prev = cpu.a;
    let cin = if cpu.get_flag(Sr::C) { 1 } else { 0 };

    let (tmp, c1) = prev.overflowing_add(arg);
    let (res, c2) = tmp.overflowing_add(cin);
    let v = add_overflows(prev, arg, res);

    cpu.a = res;

    cpu.update_flags(SrUpdate {
        c: Some(c1 | c2),
        v: Some(v),
        ..SrUpdate::num_flags(res)
    }.result());
}

with_addressing_modes!(adc, Immediate, ZeroPage, IndexedZX, Absolute, AbsoluteX, AbsoluteY, IndirectX, IndirectY);

/*
fn and<B: Bus>(cpu: &mut Cpu<B>, _: &mut B, arg: u8) {
    cpu.a &= arg;
    cpu.update_flags(SrUpdate {
        ..SrUpdate::num_flags(cpu.a)
    }.result());
}

with_addressing_modes!(and, 
*/

#[cfg(test)]
mod tests {
    use crate::{Cpu, Bus, Addr};

    use super::adc;

    struct MockBus {
        mem: [u8; 0x10000],
    }

    impl MockBus {
        fn new() -> Self {
            Self {
                mem: [0; 0x10000],
            }
        }
    }

    impl Bus for MockBus {
        fn read(&mut self, addr: Addr) -> u8 {
            self.mem[usize::from(addr)]
        }

        fn write(&mut self, addr: Addr, value: u8) {
            self.mem[usize::from(addr)] = value;
        }
    }


    #[test]
    fn test_adc_0() {
        let mut cpu: Cpu<MockBus> = Cpu::new();
        let mut bus = MockBus::new();
        cpu.a = 238;
        cpu.sr = 0b0000_0001;
        adc(&mut cpu, &mut bus, 0);
        assert_eq!(cpu.a, 239);
    }

    #[test]
    fn test_sanity() {
        let (res, c) = (238_u8).overflowing_add(1);
        assert_eq!(res, 239);
        assert_eq!(c, false);
    }
}
