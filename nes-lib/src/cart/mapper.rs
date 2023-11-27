use crate::{cart::header::INesHeader, Addr};

use super::{Cart, DecodeError, DeferredRead, DeferredWrite};

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
    fn read(&mut self, addr: Addr) -> Result<u8, DeferredRead> {
        match self {
            INesMapper::NROM(c) => c.read(addr),
        }
    }

    fn write(&mut self, addr: Addr, val: u8) -> Option<DeferredWrite> {
        match self {
            INesMapper::NROM(c) => c.write(addr, val),
        }
    }

    fn chr(&self) -> Option<[u8; 0x2000]> {
        match self {
            INesMapper::NROM(c) => c.chr()
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

    pub mirror_bit: usize,
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

        let mirror_bit: usize = match header.mirroring {
            super::header::Mirroring::H => 10,
            super::header::Mirroring::V => 11,
        };

        Ok(Self {
            prg_ram: [0; 0x2000],
            prg_rom,
            prg_rom_size,
            chr_rom,
            mirror_bit,
        })
    }
}

impl Cart for NROM {
    fn read(&mut self, address: Addr) -> Result<u8, DeferredRead>  {
        let addr = usize::from(address);
        Ok(match address {
            0x0000..=0x1fff => self.chr_rom[addr],
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
        })
    }

    fn write(&mut self, address: Addr, val: u8) -> Option<DeferredWrite> {
        let addr = usize::from(address);
        match address {
            0x0000..=0x1fff => {},
            0x2000..=0x3eff => {
                let mirrored: usize = (address % 0x1000).into();
                // The address into a single nametable.
                let sub_addr = mirrored % 0x400;

                // True iff nametable B is being accessed.
                let use_b = (mirrored >> self.mirror_bit) & 1 == 1;

                let final_addr: usize = if !use_b {
                    sub_addr
                } else {
                    sub_addr + 0x400
                };

                return Some(DeferredWrite::VRAM(final_addr, val));
            }
            0x4020..=0x5fff => {},
            0x6000..=0x7fff => self.prg_ram[addr - 0x6000] = val,
            0x8000..=0xbfff => {}
            0xc000..=0xffff => {}
            // Unexpected address
            _ => panic!(),
        }

        None
    }

    fn chr(&self) -> Option<[u8; 0x2000]> {
        Some(self.chr_rom.clone())
    }
}
