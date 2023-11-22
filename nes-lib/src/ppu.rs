const PPUCTRL: usize = 0;
const PPUMASK: usize = 1;
const PPUSTATUS: usize = 2;
const OAMADDR: usize = 3;
const OAMDATA: usize = 4;
const PPUSCROLL: usize = 5;
const PPUADDR: usize = 6;
const PPUDATA: usize = 7;
const OAMDMA: usize = 8;

pub enum PixelSource {
    Screen,
    Left,
}

pub type Color = (u8, u8, u8);

pub trait Screen {
    fn put_pixel(&mut self, row: u16, col: u16, c: Color);
}

pub struct Ppu<S: Screen> {
    regs: [u8; 0x8],
    screen: S,
    time: u128,
}

impl<S: Screen> Ppu<S> {
    pub fn new(screen: S) -> Self {
        Self {
            regs: [0; 0x8],
            screen,
            time: 0,
        }
    }

    pub fn catch_up(&mut self, time: u128) {
        let elapsed = time - self.time;
        self.time = time;

        for _ in 0 .. elapsed {
            self.tick();
        }
    }

    fn tick(&mut self) {
        self.time += 1;
    }
}
