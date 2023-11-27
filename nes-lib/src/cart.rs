pub mod header;
pub mod mapper;

use super::bus::Addr;

/// Indicates an error decoding a .nes file
#[derive(Debug, Clone)]
pub struct DecodeError;

#[derive(Debug)]
pub enum DeferredRead {
    /// The address should be in the range of 0 .. 0x800
    VRAM(usize),
}

#[derive(Debug)]
pub enum DeferredWrite {
    /// The address should be in the range of 0 .. 0x800
    VRAM(usize, u8)
}

// TODO: maybe remove?
pub trait Cart {
    fn read(&mut self, address: Addr) -> Result<u8, DeferredRead>;
    fn write(&mut self, address: Addr, val: u8) -> Option<DeferredWrite>;
    fn chr(&self) -> Option<[u8; 0x2000]>;
}
