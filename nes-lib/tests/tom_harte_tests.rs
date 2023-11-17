// Tests are from https://github.com/TomHarte/ProcessorTests/tree/main/nes6502
use nes_lib;

use pretty_assertions::assert_eq;
use seq_macro::seq;
use serde::Deserialize;
use std::{fmt, fs, panic};

#[derive(Debug, Deserialize, Clone)]
struct Nes6502TestCase {
    name: String,
    initial: CpuBusState,
    r#final: CpuBusState,
    cycles: Vec<(u16, u8, String)>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
struct CpuBusState {
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    ram: Vec<(u16, u8)>,
}

impl fmt::Display for Nes6502TestCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name: {}", self.name)
    }
}

struct MockBus {
    mem: Vec<u8>,
    events: Vec<(u16, u8, String)>,
}

impl MockBus {
    fn new() -> Self {
        Self {
            mem: vec![0; 0x10000],
            events: vec![],
        }
    }
}

impl MockBus {
    fn no_event_read(&self, addr: nes_lib::Addr) -> u8 {
        self.mem[usize::from(addr)]
    }

    fn no_event_write(&mut self, addr: nes_lib::Addr, value: u8) {
        self.mem[usize::from(addr)] = value;
    }
}

impl nes_lib::Bus for MockBus {
    fn read(&mut self, addr: nes_lib::Addr) -> u8 {
        let val = self.no_event_read(addr);
        self.events.push((addr, val, "read".to_string()));
        val
    }

    fn write(&mut self, addr: nes_lib::Addr, value: u8) {
        self.events.push((addr, value, "write".to_string()));
        self.no_event_write(addr, value);
    }
}

fn run_nes_6502_test_case(case: &Nes6502TestCase, index: usize, num_cases: usize) {
    let mut bus = MockBus::new();
    let mut cpu = nes_lib::Cpu::new();
    initialize_nes_state(&mut cpu, &mut bus, &case.initial);

    cpu.tick(&mut bus);
    dbg!(cpu.a);

    let result = {
        dbg!(cpu.a);
        let case = case.clone();
        panic::catch_unwind(move || {
            dbg!(cpu.a);
            assert_nes_state(&cpu, &bus, &case.r#final);
            assert_bus_events(&bus, &case.cycles);
        })
    };

    if let Err(err) = result {
        eprintln!("Case {} of {} failed:\n{:#?}", index+1, num_cases, case);
        panic::resume_unwind(err)
    }
}

fn initialize_nes_state(cpu: &mut nes_lib::Cpu<MockBus>, bus: &mut MockBus, initial: &CpuBusState) {
    cpu.pc = initial.pc;
    cpu.a = initial.a;
    cpu.x = initial.x;
    cpu.y = initial.y;
    cpu.sr = initial.p;
    cpu.sp = initial.s;

    for (addr, val) in initial.ram.iter() {
        bus.no_event_write(*addr, *val)
    }
}

fn assert_nes_state(cpu: &nes_lib::Cpu<MockBus>, bus: &MockBus, state: &CpuBusState) {
    let mut current_state = CpuBusState {
        pc: cpu.pc,
        a: cpu.a,
        x: cpu.x,
        y: cpu.y,
        p: cpu.sr,
        s: cpu.sp,
        ram: vec![],
    };

    for (addr, _) in state.ram.iter() {
        current_state.ram.push((*addr, bus.no_event_read(*addr)));
    }

    assert_eq!(&current_state, state);
}

fn assert_bus_events(bus: &MockBus, cycles: &[(u16, u8, String)]) {
    assert_eq!(bus.events, cycles);
}

// Generate test fns for all 256 opcodes
seq!(N in 0..=255 {
    #[test]
    fn nes_6502_tests_~N() {
        // Jam opcodes
        if [0x02, 0x12, 0x22, 0x32, 0x42, 0x52, 0x62, 0x72, 0x92, 0xB2, 0xD2, 0xF2].contains(&N) {
            return;
        }

        let path = format!("tests/nes6502/v1/{:02x}.json", N);
        let data = fs::read_to_string(path).expect("Unable to read file");
        let test_cases: Vec<Nes6502TestCase> = serde_json::from_str(&data).expect("Unable to parse data");

        for (i, case) in test_cases.iter().enumerate() {
            run_nes_6502_test_case(case, i, test_cases.len());
        }
    }
});
