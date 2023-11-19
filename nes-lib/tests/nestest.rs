use nes_lib::{
    cart::{header::FileHeader, mapper::INesMapper, DecodeError},
    Bus, Nes, NesBus, Ppu, Screen,
};

use regex::Regex;
use pretty_assertions::assert_eq;

struct MockScreen;

impl MockScreen {
    pub fn new() -> Self {
        Self
    }
}

impl Screen for MockScreen {
    fn put_pixel(&mut self, _: u16, _: u16, _: nes_lib::Color) {}
}

#[cfg_attr(not(feature = "debug"), ignore)]
#[test]
fn nestest() -> Result<(), DecodeError> {
    let rom_bytes = include_bytes!("res/nestest.nes");
    let log_str = include_str!("res/nestest.log");

    let screen = MockScreen::new();
    let ppu = Ppu::new(screen);

    let header = FileHeader::try_decode(&rom_bytes[..16])?;
    let cart = match header {
        FileHeader::INes(header) => INesMapper::try_decode(&header, &rom_bytes[16..])?,
        FileHeader::Nes2(_) => return Err(DecodeError),
    };

    let mut bus = NesBus::new(cart, ppu);
    bus.on_write(|addr, val| {
        if addr == 0x02 || addr == 0x03 {
            println!("write");
            dbg!(addr);
            dbg!(val);
        }
    });
    let mut nes = Nes::new(bus);

    nes.cpu.pc = 0xc000;

    for log_line in log_str.lines() {
        println!("{}", log_line);
        let expected_log_event = parse_log_line(log_line);
        let current_log_event = LogEvent {
            pc: nes.cpu.pc,
            a: nes.cpu.a,
            x: nes.cpu.x,
            y: nes.cpu.y,
            sp: nes.cpu.sp,
            sr: nes.cpu.sr,
        };
        assert_eq!(current_log_event, expected_log_event);
        nes.tick();
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct LogEvent {
    pc: u16,
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    sr: u8,
}

fn parse_log_line(line: &str) -> LogEvent {
    let re = Regex::new(r"^(?<pc>[A-F0-9]{4})  (?<op1>[A-F0-9]{2}) (?<op2>[ A-F0-9]{2}) (?<op3>[ A-F0-9]{2})  (.{30})  A:(?<a>[A-F0-9]{2}) X:(?<x>[A-F0-9]{2}) Y:(?<y>[A-F0-9]{2}) P:(?<sr>[A-F0-9]{2}) SP:(?<sp>[A-F0-9]{2}) PPU:(.{7}) CYC:(\d+)$").unwrap();
    let caps = re.captures(line).unwrap();

    LogEvent {
        pc: u16::from_str_radix(&caps["pc"], 16).unwrap(),
        a: u8::from_str_radix(&caps["a"], 16).unwrap(),
        x: u8::from_str_radix(&caps["x"], 16).unwrap(),
        y: u8::from_str_radix(&caps["y"], 16).unwrap(),
        sp: u8::from_str_radix(&caps["sp"], 16).unwrap(),
        sr: u8::from_str_radix(&caps["sr"], 16).unwrap(),
    }
}
