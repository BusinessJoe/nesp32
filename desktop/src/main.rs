use nes_lib::{self, cart::{header::FileHeader, mapper::INesMapper, DecodeError}, NesBus};

struct DesktopScreen {

}

impl DesktopScreen {
    fn new() -> Self {
        Self {}
    }
}

impl nes_lib::Screen for DesktopScreen {
    fn put_pixel(&mut self, row: u16, col: u16, c: nes_lib::Color) {
        println!("r: {} c: {} rgb: {:?}", row, col, c);
    }
}

fn main() -> Result<(), DecodeError> {
    let rom_bytes = include_bytes!("../nestest.nes");
    let header = FileHeader::try_decode(&rom_bytes[..16])?;
    let cart = match header {
        FileHeader::INes(header) => INesMapper::try_decode(&header, &rom_bytes[16..])?,
        FileHeader::Nes2(_) => return Err(DecodeError),
    };

    let screen = DesktopScreen::new();
    let ppu = nes_lib::Ppu::new(screen);

    let mut bus = NesBus::new(cart, ppu);
    bus.on_write(|addr, val| {
        println!("write addr: {:x} val: {:x}", addr, val);
    });

    let mut nes = nes_lib::Nes::new(bus);
    nes.cpu.pc = 0xc000;

    println!("PC starting at {:X}", nes.cpu.pc);

    loop {
        nes.tick();
    }
}
