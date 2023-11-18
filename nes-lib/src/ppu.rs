pub type Color = (u8, u8, u8);

pub trait Screen {
    fn put_pixel(&mut self, row: u16, col: u16, c: Color);
}

pub struct Ppu<S: Screen> {
    regs: [u8; 0x8],
    screen: S,
}

impl<S: Screen> Ppu<S> {
    pub fn new(screen: S) -> Self {
        Self {
            regs: [0; 0x8],
            screen,
        }
    }
}
