use nes_lib;

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

fn main() {
    let rom = include_bytes!("../nestest.nes");
    let cart = nes_lib::cart::NoMapperCart::new(b"Cart Name", rom);

    let screen = DesktopScreen::new();
    let ppu = nes_lib::Ppu::new(screen);

    let bus = nes_lib::NesBus::new(cart, ppu);
    let mut nes = nes_lib::Nes::new(bus);

    loop {
        nes.tick();
    }
}
