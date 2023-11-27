#![no_std]
#![feature(const_mut_refs)]

mod bus;
pub mod cart;
mod cpu;
mod nes;
mod ppu;

pub use bus::{Addr, Bus, NesBus};
pub use cpu::Cpu;
pub use nes::Nes;
pub use ppu::{Color, Ppu, Screen, PixelSource};

#[cfg(feature = "debug")]
pub use nes::{EventSource, EventCb};
