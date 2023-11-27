use crate::{Bus, Cpu};

#[derive(Clone, Copy)]
pub enum Sr {
    C,
    Z,
    I,
    D,
    B,
    V,
    N,
}

impl Sr {
    fn index(&self) -> usize {
        match *self {
            Sr::C => 0,
            Sr::Z => 1,
            Sr::I => 2,
            Sr::D => 3,
            Sr::B => 4,
            Sr::V => 6,
            Sr::N => 7,
        }
    }
}

pub struct SrUpdate {
    pub c: Option<bool>,
    pub z: Option<bool>,
    pub i: Option<bool>,
    pub d: Option<bool>,
    pub b: Option<bool>,
    pub v: Option<bool>,
    pub n: Option<bool>,
}

pub struct SrUpdateResult {
    mask: u8,
    val: u8,
}

impl SrUpdate {
    pub const fn default() -> Self {
        Self {
            c: None,
            z: None,
            i: None,
            d: None,
            b: None,
            v: None,
            n: None,
        }
    }

    /// Sets N and Z flags.
    pub const fn num_flags(res: u8) -> Self {
        Self {
            z: Some(res == 0),
            n: Some(res >> 7 == 1),
            ..Self::default()
        }
    }

    // Mask contains 1 iff corresponding flag is None.
    // Val contains 1 iff corresponding flag is Some(true).
    // Should be used like cpu.s = cpu.s & mask | val.
    pub const fn result(&self) -> SrUpdateResult {
        let mut mask = 0;
        let mut val = 0;
        let flags = [self.c, self.z, self.i, self.d, self.b, None, self.v, self.n];
        let mut i = 0;
        while i < flags.len() {
            if flags[i].is_none() {
                mask |= 1 << i;
            }
            if let Some(true) = flags[i] {
                val |= 1 << i;
            }
            i += 1;
        }
        SrUpdateResult { mask, val }
    }
}

impl<B: Bus> Cpu<B> {
    pub fn update_flags(&mut self, sru: SrUpdateResult) {
        self.sr = self.sr & sru.mask | sru.val;
    }

    pub fn get_flag(&self, flag: Sr) -> bool {
        (self.sr >> flag.index()) & 1 == 1
    }

    pub fn set_flag(&mut self, flag: Sr, state: bool) {
        let mut status = self.sr & !(1 << flag.index());
        if state {
            status |= 1 << flag.index();
        }
        self.sr = status;
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::status_register::{Sr, SrUpdate};
    use crate::{Bus, Cpu};

    struct MockBus;
    impl Bus for MockBus {
        fn read(&mut self, _: crate::Addr) -> u8 {
            0
        }

        fn write(&mut self, _: crate::Addr, _: u8) {}

        fn catch_up(&mut self, _: u128) {
            unimplemented!()
        }

        fn debug_chr(&self) -> Option<[u8; 0x2000]> {
            None
        }
    }

    #[test]
    fn test_set_carry() {
        let mut cpu: Cpu<MockBus> = Cpu::new();
        for i in 0..=255 {
            cpu.sr = i;
            cpu.update_flags(
                SrUpdate {
                    c: Some(true),
                    ..SrUpdate::default()
                }
                .result(),
            );
            assert_eq!(cpu.sr, i | 1);
            assert_eq!(cpu.get_flag(Sr::C), true);
        }
    }

    #[test]
    fn test_reset_carry() {
        let mut cpu: Cpu<MockBus> = Cpu::new();
        for i in 0..=255 {
            cpu.sr = i;
            cpu.update_flags(
                SrUpdate {
                    c: Some(false),
                    ..SrUpdate::default()
                }
                .result(),
            );
            assert_eq!(cpu.sr, i & 0b11111110);
            assert_eq!(cpu.get_flag(Sr::C), false);
        }
    }

    #[test]
    fn test_set_negative() {
        let mut cpu: Cpu<MockBus> = Cpu::new();
        for i in 0..=255 {
            cpu.sr = i;
            cpu.update_flags(
                SrUpdate {
                    n: Some(true),
                    ..SrUpdate::default()
                }
                .result(),
            );
            assert_eq!(cpu.sr, i | (1 << 7));
            assert_eq!(cpu.get_flag(Sr::N), true);
        }
    }

    #[test]
    fn test_reset_negative() {
        let mut cpu: Cpu<MockBus> = Cpu::new();
        for i in 0..=255 {
            cpu.sr = i;
            cpu.update_flags(
                SrUpdate {
                    n: Some(false),
                    ..SrUpdate::default()
                }
                .result(),
            );
            assert_eq!(cpu.sr, i & 0b01111111);
            assert_eq!(cpu.get_flag(Sr::N), false);
        }
    }
}
