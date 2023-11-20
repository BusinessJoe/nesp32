use super::DecodeError;

// .nes files either come in the iNES or NES 2.0 format
pub enum FileHeader {
    INes(INesHeader),
    Nes2(Nes2Header),
}

pub struct INesHeader {
    /// Size of PRG ROM in 16 KB units.
    pub prg_rom_size: usize,
    /// Size of CHR ROM in 8 KB units.
    pub chr_rom_size: usize,

    pub mapper_num: u8,

    pub has_trainer: bool,
}

pub struct Nes2Header;

impl INesHeader {
    pub fn decode(header: &[u8]) -> Self {
        let prg_rom_size: usize = header[4].into();
        let chr_rom_size: usize = header[5].into();
        let mapper_num: u8 = header[6] >> 4 | header[7] & 0xf0;
        let has_trainer = header[6] & 0b100 == 0b100;
        Self {
            prg_rom_size,
            chr_rom_size,
            mapper_num,
            has_trainer,
        }
    }
}

impl Nes2Header {
    pub fn decode(_: &[u8]) -> Self {
        // Not implemented
        todo!("Nes2Header")
    }
}

impl FileHeader {
    pub fn try_decode(header: &[u8]) -> Result<Self, DecodeError> {
        if header.len() < 16 {
            // Header is too small.
            return Err(DecodeError);
        }
        if !(char::from(header[0]) == 'N'
            && char::from(header[1]) == 'E'
            && char::from(header[2]) == 'S'
            && header[3] == 0x1a)
        {
            // Header doesn't begin with NES<EOF>.
            return Err(DecodeError);
        }

        if header[7] & 0x0c != 0x08 {
            let ines = INesHeader::decode(header);
            Ok(FileHeader::INes(ines))
        } else {
            let nes2 = Nes2Header::decode(header);
            Ok(FileHeader::Nes2(nes2))
        }
    }
}
