// Tests are from https://github.com/TomHarte/ProcessorTests/tree/main/nes6502
use nes_lib;

use std::{fmt, fs};
use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
struct Nes6502TestCase {
    name: String,
    initial: NesState,
    r#final: NesState,
    cycles: Vec<(u16, u8, String)>,
}

#[derive(Deserialize)]
struct NesState {
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
    mem: Vec<u8>
}

impl MockBus {
    fn new() -> Self {
        Self {
            mem: vec![0; 0x10000]
        }
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
fn nes_6502_tests() {
    let results = (0 ..= 0xFF).flat_map(|i| {
        let path = format!("tests/nes6502/v1/{:02X}.json", i);
        let data = fs::read_to_string(path).expect("Unable to read file");
        let test_cases: Vec<Nes6502TestCase> = serde_json::from_str(&data).expect("Unable to parse data");

        test_cases.into_iter().map(run_nes_6502_test_case)
    });

    let mut num_passed = 0;
    let mut num_run = 0;

    for res in results {
        num_run += 1;
        match res {
            Ok(_) => num_passed += 1,
            Err((case_name, msg)) => {
                eprintln!("test {} failed: {}", case_name, msg);
            }
        }
    }

    eprintln!("Passed {}/{}", num_passed, num_run);

    assert_eq!(num_passed, num_run);
}

fn run_nes_6502_test_case(case: Nes6502TestCase) -> Result<(), (String, String)> {
    let bus = MockBus::new();
    let mut nes = nes_lib::Nes::new(bus);

    Err((case.name, "Unimplemented".to_string()))
}
