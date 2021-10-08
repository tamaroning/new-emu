use ines::Rom;

const RAM: u16 = 0x0000;
const RAM_MIRROR_END: u16 = 0x1fff;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRROR_END: u16 = 0x3fff;
const PRG_ROM: u16 = 0x8000;
const PRG_ROM_END: u16 = 0xFFFF;

//  _______________ $10000  _______________
// | PRG-ROM       |       |               |
// | Upper Bank    |       |               |
// |_ _ _ _ _ _ _ _| $C000 | PRG-ROM       |
// | PRG-ROM       |       |               |
// | Lower Bank    |       |               |
// |_______________| $8000 |_______________|
// | SRAM          |       | SRAM          |
// |_______________| $6000 |_______________|
// | Expansion ROM |       | Expansion ROM |
// |_______________| $4020 |_______________|
// | I/O Registers |       |               |
// |_ _ _ _ _ _ _ _| $4000 |               |
// | Mirrors       |       | I/O Registers |
// | $2000-$2007   |       |               |
// |_ _ _ _ _ _ _ _| $2008 |               |
// | I/O Registers |       |               |
// |_______________| $2000 |_______________|
// | Mirrors       |       |               |
// | $0000-$07FF   |       |               |
// |_ _ _ _ _ _ _ _| $0800 |               |
// | RAM           |       | RAM           |
// |_ _ _ _ _ _ _ _| $0200 |               |
// | Stack         |       |               |
// |_ _ _ _ _ _ _ _| $0100 |               |
// | Zero Page     |       |               |
// |_______________| $0000 |_______________|

pub struct Bus {
    // 0x800 = 2048
    cpu_vram: [u8; 0x800],
    rom: Rom
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Bus {
            cpu_vram: [0; 0x800],
            rom: rom,
        }
    }
}

pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_read_u16(&self, pos: u16) -> u16;
    fn mem_write(&mut self, addr: u16, data: u8);
    fn mem_write_u16(&mut self, addr: u16, data: u16);
    fn read_prg_rom(&self, addr: u16) -> u8;
}

impl Mem for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            // 0x0000 ~ 0x1fff used as RAM
            RAM ..= RAM_MIRROR_END => {
                let lower_11_bits = addr & 0b00000111_11111111;
                self.cpu_vram[lower_11_bits as usize]
            },
            // 0x2000 ~ 0x3fff used as PPU memory
            PPU_REGISTERS ..= PPU_REGISTERS_MIRROR_END => {
                let _tmp = addr & 0b00100000_00000111;
                todo!("PPU");
            },
            PRG_ROM ..= PRG_ROM_END => self.read_prg_rom(addr),
            _ => 0, //TODO: should not ignore
        }
    }
    
    fn mem_read_u16(&self, pos: u16) -> u16 {
        let low = self.mem_read(pos) as u16;
        let high = self.mem_read(pos + 1) as u16;
        (high << 8) |  low
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            // 0x0000 ~ 0x1fff used as RAM
            RAM ..= RAM_MIRROR_END => {
                let lower_11_bits = addr & 0b00000111_11111111;
                self.cpu_vram[lower_11_bits as usize] = data;
            },
            // 0x2000 ~ 0x3fff used as PPU memory
            PPU_REGISTERS ..= PPU_REGISTERS_MIRROR_END => {
                let _tmp = addr & 0b00100000_00000111;
                todo!("PPU");
            },
            // TODO: wrtitable when cargo test
            /* 
            PRG_ROM ..= PRG_ROM_END => panic!("Cannot write to cartridge ROM"),
            */
            _ => (), // TODO: should not ignore
        }
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let high = (data >> 8) as u8;
        let low = (data & 0xFF) as u8;
        self.mem_write(pos, low);
        self.mem_write(pos + 1, high);
    }

    fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= PRG_ROM;
        if self.rom.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            addr %= 0x4000;
        }
        self.rom.prg_rom[addr as usize]
    }
}
