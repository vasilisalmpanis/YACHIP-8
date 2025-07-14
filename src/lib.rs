use rand::prelude::*;

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

struct CHIP8 {
    ram: [u8; 0x1000],
    opcode: u16,
    graphics: [u8; 64 * 32],
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
    fn new() -> CHIP8 {
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

    fn load_rom(&mut self, rom: &[u8]) {
        self.ram[0..rom.len()].copy_from_slice(rom);  
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
}
