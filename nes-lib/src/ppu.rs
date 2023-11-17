pub type Color = (u8, u8, u8);

pub trait Screen {
    fn put_pixel(row: u16, col: u16, c: Color);
}


