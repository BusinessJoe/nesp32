use super::bus::Addr;

const MAX_CART_NAME_LENGTH: usize = 128;

pub trait Cart {
    fn name(&self) -> [u8; MAX_CART_NAME_LENGTH];
    fn read(&self, addr: Addr) -> u8;
    fn write(&mut self, addr: Addr, val: u8);
}

/* Cart structs and impls */

#[derive(Debug)]
pub struct NoMapperCart {
    pub name: [u8; MAX_CART_NAME_LENGTH],
}

impl NoMapperCart {
    // Create a new NoMapperCart with the given name.
    // Panics if the given name is longer than MAX_CART_NAME_LENGTH bytes.
    pub fn new(name_slice: &[u8]) -> Self {
        let mut name = [0; MAX_CART_NAME_LENGTH];
        name[0..name_slice.len()].copy_from_slice(name_slice);
        Self { name }
    }
}

impl Cart for NoMapperCart {
    fn name(&self) -> [u8; MAX_CART_NAME_LENGTH] {
        self.name
    }

    fn read(&self, addr: Addr) -> u8 {
        0
    }

    fn write(&mut self, addr: Addr, val: u8) {}
}
