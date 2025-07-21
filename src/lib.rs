use std::usize;
use std::fs;

use rand::prelude::Rng;

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
    pub graphics: [u8; 64 * 32],
    regs: [u8; 16],
    pc: u16,
    delay_timer: u8,
    sound_timer: u8,
    index: u16,
    sp: usize,
    stack : [u16; 16],
    keys: [u8; 16],
}

impl CHIP8 {
    pub fn new() -> CHIP8 {
        let mut chip = CHIP8 { 
            ram: [0; 0x1000],
            graphics: [0; 64 * 32],
            stack: [0; 16],
            keys: [0; 16],
            regs: [0; 16],

            opcode: 0,
            delay_timer: 0,
            sound_timer: 0,
            index: 0,
            sp: 0,
            pc: 0x200,
        };
        for (idx, el) in FONTSET.iter().enumerate() {
            chip.ram[idx] = *el;
        }
        chip
    }

    #[inline]
    pub fn press_key(&mut self, idx: usize) {
        self.keys[idx] = 1;
    }

    #[inline]
    pub fn release_key(&mut self, idx: usize) {
        self.keys[idx] = 0;
    }

    pub fn load_rom(&mut self, file_path: &str) {
        let contents = fs::read(file_path);
        match contents {
            Ok(buff) => {
                let len = buff.len();
                for i in 0..len {
                    self.ram[0x200 + i] = buff[i];
                }
            },
            _ => {
                panic!("Invalid ROM path provided");
            },
        }
    }

    pub fn increment_pc(&mut self) {
        self.pc += 2;
    }

    pub fn cycle(&mut self) {
        let byte1 = self.ram[self.pc as usize] as u16;
        let byte2 = self.ram[self.pc as usize + 1] as u16;
        self.opcode = (byte1 << 8) | byte2;

        let ins: u8 = (byte1 as u8) >> 4;
        // println!("opcode {:#x}", self.opcode);
        match ins {
            0x0 => {
                match self.opcode {
                    0x00E0 => self.graphics = [0; 64 * 32],
                    0x00EE => { // Return
                        self.sp -= 1;
                        self.pc = self.stack[self.sp];
                    }
                    _ => panic!("Undefined opcode {}\n", self.opcode)
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
                let reg2: u8 = (byte2 as u8) >> 4;
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
            0x7 => {
                let reg: u8 = byte1 as u8 & 0xF;
                let value = self.regs[reg as usize].wrapping_add(byte2 as u8);
                self.regs[reg as usize] = value;
                self.increment_pc();
            },
            0x8 => {
                let x = ((byte1 as u8) & 0x0F) as usize;
                let y = ((byte2 as u8) >> 4) as usize;
                let m = (byte2 & 0x0F) as u8;
                match m {
                0x0 => {self.regs[x as usize] = self.regs[y as usize]},
                0x1 => {self.regs[x as usize] |= self.regs[y as usize]},
                0x2 => {self.regs[x as usize] &= self.regs[y as usize]},
                0x3 => {self.regs[x as usize] ^= self.regs[y as usize]},

                0x4 => {
                    let mut sum: u16 = self.regs[x] as u16;
                    sum = sum.wrapping_add(self.regs[y] as u16);
                    self.regs[0xF] = if sum > 255 {1} else {0};
                    self.regs[x] = sum as u8;
                },
                0x5 => {
                    self.regs[0xF] = if self.regs[x] > self.regs[y] {1} else {0};
                    self.regs[x] = self.regs[x].wrapping_sub(self.regs[y]);
                },
                0x6 => {
                    self.regs[0xF] = self.regs[x] & 1; // LSB
                    self.regs[x] >>= 1;
                },
                0x7 => {
                    self.regs[0xF] = if self.regs[y] > self.regs[x] {1} else {0};
                    self.regs[x] = self.regs[y].wrapping_sub(self.regs[x]);
                },
                0xe => {
                    self.regs[0xF] = if self.regs[x] & 0x80 != 0 {1} else {0}; // MSB
                    self.regs[x] <<= 1;
                },
                _=> {},
                }
                self.increment_pc();
            },
            0x9 => {
                let reg1: u8 = (byte1 as u8) & 0x0F;
                let reg2: u8 = (byte2 as u8) >> 4;
                if self.regs[reg1 as usize] != self.regs[reg2 as usize] {
                    self.increment_pc();
                }
                self.increment_pc();
            }
            0xA => {
                self.index = self.opcode & 0x0FFF;
                self.increment_pc();
            }
            0xB => {
                self.pc = (self.opcode & 0x0FFF) + (self.regs[0] as u16);
                self.increment_pc();
            }
            0xC => {
                let mut rng = rand::rng();
                let byte: u8 = rng.random::<u8>();
                let x = ((byte1 as u8) & 0x0F) as usize;
                let result = byte & (byte2 as u8);
                self.regs[x] = result;
                self.increment_pc();
            }
            0xD => {
                self.regs[0xF] = 0;
                let xx = ((byte1 as u8) & 0x0F) as usize;
                let yy = ((byte2 as u8) >> 4) as usize;
                let nn = ((byte2 & 0x0F) as u8) as usize;

                let regx = self.regs[xx] as usize;
                let regy = self.regs[yy] as usize;
                let mut y: usize = 0;
                while y < nn {
                    let pixel = self.ram[(self.index as usize) + y];
                    y += 1;
                    let mut x = 0;
                    while x < 8 {
                        let msb = 0x80;
                        if pixel & (msb >> x) != 0 {
                            let t_x = (regx + x) % 64;
                            let t_y = (regy + y) % 32;
                            let index = t_x + t_y * 64;
                            self.graphics[index] ^= 1;
                            if self.graphics[index] == 0 {
                                self.regs[0xF] = 1;
                            }
                        }
                        x += 1;
                    }
                }
                self.increment_pc();
            },
            0xE => {
                let x = ((byte1 as u8) & 0x0F) as usize;
                let kk = byte2 as usize;
                if kk == 0x9E {
                    if self.keys[self.regs[x] as usize] == 1 {
                        self.increment_pc();
                    }
                } else if kk == 0xA1 {
                    if self.keys[self.regs[x] as usize] != 1 {
                        self.increment_pc();
                    }
                }
                self.increment_pc();
            },
            0xF => {
                let mode = byte2;
                let x = ((byte1 as u8) & 0x0F) as usize;
                match mode {
                    0x07 => {
                        self.regs[x] = self.delay_timer;
                    },
                    0x0A => {
                        let mut key_pressed: bool = false;
                        for n in 0..16 {
                            if self.keys[n] != 0 {
                                self.regs[x] = n as u8;
                                key_pressed = true;
                                break;
                            }
                        }
                        if !key_pressed {
                            return ;
                        }
                    },
                    0x15 => {
                        self.delay_timer = self.regs[x];
                    },
                    0x18 => {
                        self.sound_timer = self.regs[x];
                    },
                    0x1E => {
                        self.index = self.index.wrapping_add(self.regs[x] as u16);
                    },
                    0x29 => {
                        if self.regs[x] < 16 {
                            self.index = (self.regs[x] as u16) * 0x05;
                        }
                    },
                    0x33 => {
                        self.ram[self.index as usize] = self.regs[x] / 100;
                        self.ram[self.index as usize + 1] = (self.regs[x] / 10) % 10;
                        self.ram[self.index as usize + 2] = self.regs[x] % 10;
                    },
                    0x55 => {
                        for i in 0..=x {
                            self.ram[(self.index as usize) + i] = self.regs[i];
                        }
                    },
                    0x65 => {
                        for i in 0..x {
                            self.regs[i] = self.ram[(self.index as usize) + i];
                        }
                    },
                    _ => {},
                }
                self.increment_pc();
            },
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
        result.load_rom("roms/games/Cave.ch8");
        for _ in 0..100 {
            result.cycle();
        }
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
