use crate::{cart::header::INesHeader, Addr};

use super::{Cart, DecodeError};

/* Cart structs and impls */
pub enum INesMapper {
    NROM(NROM),
}

impl INesMapper {
    pub fn try_decode(header: &INesHeader, data: &[u8]) -> Result<Self, DecodeError> {
        Ok(match header.mapper_num {
            0 => INesMapper::NROM(NROM::new(header, data)?),
            _ => return Err(DecodeError),
        })
    }
}

impl Cart for INesMapper {
    fn read(&mut self, addr: Addr) -> u8 {
        match self {
            INesMapper::NROM(c) => c.read(addr),
        }
    }

    fn write(&mut self, addr: Addr, val: u8) {
        match self {
            INesMapper::NROM(c) => c.write(addr, val),
        }
    }
}

#[derive(Debug)]
pub struct NROM {
    pub prg_ram: [u8; 0x2000],
    pub prg_rom: [u8; 0x8000],
    /// PRG ROM is either 16 KiB or 32 KiB, determined by this value.
    pub prg_rom_size: usize,
    pub chr_rom: [u8; 0x2000],
}

impl NROM {
    /// Creates a new NROM cartridge.
    ///
    /// # Arguments
    ///
    /// * `header` - A reference to an INesHeader struct.
    /// * `data` - A slice containing the .nes file data after the header.
    pub fn new(header: &INesHeader, mut data: &[u8]) -> Result<Self, DecodeError> {
        let prg_rom_size = match header.prg_rom_size {
            1 => 0x4000,
            2 => 0x8000,
            _ => return Err(DecodeError),
        };
        let chr_rom_size = match header.chr_rom_size {
            1 => 0x2000,
            _ => return Err(DecodeError),
        };

        if header.has_trainer {
            todo!("Do something with trainer data");
            //data = &data[512 ..];
        }

        let mut prg_rom = [0u8; 0x8000];
        if data.len() < prg_rom_size {
            return Err(DecodeError);
        }
        prg_rom[0..prg_rom_size].copy_from_slice(&data[0..prg_rom_size]);
        data = &data[prg_rom_size..];

        let chr_rom: [u8; 0x2000] = data[0..chr_rom_size].try_into().map_err(|_| DecodeError)?;

        Ok(Self {
            prg_ram: [0; 0x2000],
            prg_rom,
            prg_rom_size,
            chr_rom,
        })
    }
}

impl Cart for NROM {
    fn read(&mut self, address: Addr) -> u8 {
        let addr = usize::from(address);
        match address {
            0x4020..=0x5fff => 0,
            0x6000..=0x7fff => self.prg_ram[addr - 0x6000],
            0x8000..=0xbfff => self.prg_rom[addr - 0x8000],
            0xc000..=0xffff => {
                if self.prg_rom_size == 0x4000 {
                    self.prg_rom[addr - 0xc000]
                } else {
                    self.prg_rom[addr - 0x8000]
                }
            }
            // Unexpected address
            _ => panic!(),
        }
    }

    fn write(&mut self, address: Addr, val: u8) {
        let addr = usize::from(address);
        match address {
            0x4020..=0x5fff => {},
            0x6000..=0x7fff => self.prg_ram[addr - 0x6000] = val,
            0x8000..=0xbfff => {}
            0xc000..=0xffff => {}
            // Unexpected address
            _ => panic!(),
        }
    }
}
