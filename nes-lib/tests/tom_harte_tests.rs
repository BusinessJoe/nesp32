// Tests are from https://github.com/TomHarte/ProcessorTests/tree/main/nes6502
#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]

use nes_lib;

use pretty_assertions::assert_eq;
use serde::Deserialize;
use std::{fmt, panic};

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

    fn catch_up(&mut self, _: u128) {
        unimplemented!()
    }

    fn debug_chr(&self) -> Option<[u8; 0x2000]> {
        None
    }
}

fn run_nes_6502_state_test_case(case: &Nes6502TestCase, index: usize, num_cases: usize) {
    let mut bus = MockBus::new();
    let mut cpu = nes_lib::Cpu::new();
    initialize_nes_state(&mut cpu, &mut bus, &case.initial);

    cpu.tick(&mut bus);

    let result = {
        let case = case.clone();
        panic::catch_unwind(move || {
            assert_nes_state(&cpu, &bus, &case.r#final);
        })
    };

    if let Err(err) = result {
        eprintln!("Case {} of {} failed:\n{:#?}", index + 1, num_cases, case);
        panic::resume_unwind(err)
    }
}

fn run_nes_6502_read_write_test_case(case: &Nes6502TestCase, index: usize, num_cases: usize) {
    let mut bus = MockBus::new();
    let mut cpu = nes_lib::Cpu::new();
    initialize_nes_state(&mut cpu, &mut bus, &case.initial);

    cpu.tick(&mut bus);

    let result = {
        let case = case.clone();
        panic::catch_unwind(move || {
            assert_bus_events(&bus, &case.cycles);
        })
    };

    if let Err(err) = result {
        eprintln!("Case {} of {} failed:\n{:#?}", index + 1, num_cases, case);
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

fn ignore_test(path: &std::path::Path) -> bool {
    let filename = path.iter().last().unwrap().to_str().unwrap();
    let hex_str = filename.split('.').into_iter().next().unwrap();
    let hex = u8::from_str_radix(hex_str, 16).unwrap();

    // Jam opcodes
    if [0x02, 0x12, 0x22, 0x32, 0x42, 0x52, 0x62, 0x72, 0x92, 0xB2, 0xD2, 0xF2].contains(&hex) {
        return true;
    }

    // Unstable opcodes
    if [0xAB, 0x8B, 0x9F, 0x93, 0x9E, 0x9C, 0x9B].contains(&hex) {
        return true;
    }

    return false;
}

#[datatest::files("tests/nes6502/v1", {
    input in r"^(.*)\.json" if !ignore_test,
})]
fn nes_6502_opcode_state_test(input: &str) {
    let test_cases: Vec<Nes6502TestCase> = serde_json::from_str(input).expect("Unable to parse data");

    for (i, case) in test_cases.iter().enumerate() {
        run_nes_6502_state_test_case(case, i, test_cases.len());
    }
}

#[datatest::files("tests/nes6502/v1", {
    input in r"^(.*)\.json" if !ignore_test,
})]
fn nes_6502_opcode_read_write_test(input: &str) {
    let test_cases: Vec<Nes6502TestCase> = serde_json::from_str(input).expect("Unable to parse data");

    for (i, case) in test_cases.iter().enumerate() {
        run_nes_6502_read_write_test_case(case, i, test_cases.len());
    }
}
