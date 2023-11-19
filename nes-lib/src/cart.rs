pub mod header;
pub mod mapper;

use super::bus::Addr;

/// Indicates an error decoding a .nes file
#[derive(Debug, Clone)]
pub struct DecodeError;

// TODO: maybe remove?
pub trait Cart {
    fn read(&mut self, address: Addr) -> u8;
    fn write(&mut self, address: Addr, val: u8);
}
