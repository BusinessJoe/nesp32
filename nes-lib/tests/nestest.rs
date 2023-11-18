struct MockBus {
    mem: Vec<u8>,
}

impl MockBus {
    fn new_nestest() -> Self {
        let bytes = include_bytes!("roms/nestest.nes");
        let mut mem = vec![0; 0x10000];
        mem[0x8000 .. 0x8000 + bytes.len()].copy_from_slice(bytes);
        Self { mem }
    }
}

impl nes_lib::Bus for MockBus {
    fn read(&mut self, addr: nes_lib::Addr) -> u8 {
        self.mem[usize::from(addr)]
    }

    fn write(&mut self, addr: nes_lib::Addr, value: u8) {
        self.mem[usize::from(addr)] = value;
    }
}

#[test]
fn nestest() {
    let mut bus = MockBus::new_nestest();
    let mut cpu: nes_lib::Cpu<MockBus> = nes_lib::Cpu::new();
    cpu.pc = 0xc000;

    for _ in 0 .. 1000 {
        cpu.tick(&mut bus);
    }

}
