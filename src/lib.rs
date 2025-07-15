use std::usize;

// use rand::prelude::*;

const FONTSET_SIZE: usize = 80;

pub const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct CHIP8 {
    ram: [u8; 0x1000],
    opcode: u16,
    graphics: [u8; 64 * 32],
    regs: [u8; 16],
    pc: u16,
    // delay_timer: u8,
    // sound_timer: u8,
    // index: u16,
    sp: usize,
    stack : [u16; 16],
    // keys: [u8; 16],
}

impl CHIP8 {
    pub fn new() -> CHIP8 {
        let mut chip = CHIP8 { 
            ram: [0; 0x1000],
            graphics: [0; 64 * 32],
            stack: [0; 16],
            // keys: [0; 16],
            regs: [0; 16],

            opcode: 0,
            // delay_timer: 0,
            // sound_timer: 0,
            // index: 0,
            sp: 0,
            pc: 0x200,
        };
        for (idx, el) in FONTSET.iter().enumerate() {
            chip.ram[idx] = *el;
        }
        chip
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.ram[0..rom.len()].copy_from_slice(rom);  
    }

    pub fn increment_pc(&mut self) {
        self.pc += 2;
    }

    pub fn cycle(&mut self) {
        let byte1 = self.ram[self.pc as usize] as u16;
        let byte2 = self.ram[self.pc as usize + 1] as u16;
        self.opcode = (byte1 << 8) | byte2;

        let ins: u8 = (byte1 as u8) >> 4;
        match ins {
            0x0 => {
                match self.opcode {
                    0x00E0 => self.graphics = [0; 64 * 32],
                    0x00EE => { // Return
                        self.sp -= 1;
                        self.pc = self.stack[self.sp];
                    }
                    _ => panic!("Undefined opcode\n")
                }
                self.increment_pc();
            },
            0x1 => self.pc = self.opcode & 0xFFF,
            0x2 => { // Call instruction
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                self.pc = self.opcode & 0x0FFF;
            }
            0x3 => {
                let reg: u8 = (byte1 as u8) & 0x0F;
                if self.regs[reg as usize] == (byte2 as u8) {
                    self.increment_pc();
                }
                self.increment_pc();
            }
            0x4 => {
                let reg: u8 = (byte1 as u8) & 0x0F;
                if self.regs[reg as usize] != (byte2 as u8) {
                    self.increment_pc();
                }
                self.increment_pc();
            }
            0x5 => {
                let reg1: u8 = (byte1 as u8) & 0x0F;
                let reg2: u8 = ((byte2 as u8) & 0x0F) >> 4;
                if self.regs[reg1 as usize] == self.regs[reg2 as usize] {
                    self.increment_pc();
                }
                self.increment_pc();
            }
            0x6 => {
                let value: u8 = byte2 as u8;
                let reg: u8 = (byte1 & 0x0F) as u8;
                self.regs[reg as usize] = value;
                self.increment_pc();
            }
            _ => {
                panic!("Undefined instruction\n");
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_fake_rom() {
        let mut result = CHIP8::new();
        let vec: [u8; 100] = [1; 100];
        result.load_rom(&vec[0..100]);
    assert_eq!(&vec[0..100], &result.ram[0..100]);
    }

    #[test]
    fn load_font_set() {
        let result = CHIP8::new();
        for (idx, _) in FONTSET.iter().enumerate() {
            assert_eq!(result.ram[idx], FONTSET[idx]);
        }
    }

    #[test]
    fn clear_screen() {
        let mut result = CHIP8::new();
        result.ram[result.pc as usize] = 0x00;
        result.ram[result.pc as usize + 1] = 0xE0;
        result.cycle();
    }
}
