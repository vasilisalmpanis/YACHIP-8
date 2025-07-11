
struct CHIP8 {
    ram: RAM,
    pc: u16
}

impl CHIP8 {
    fn new() -> CHIP8 {
        CHIP8 { 
            ram: RAM::new(),
            pc: 0
        }
    }

    fn load_rom(&mut self, rom: &[u8]) {
        self.ram.load_rom(rom);
    }
}

pub struct RAM {
    memory: [u8; 0x1000],
}

impl RAM {
    fn new() -> RAM {
        RAM { memory: [0; 0x1000] }
    }

    fn read(&self, address: u16) -> u8 {
        if address >= 0x1000 {
            panic!("Invalid address");
        }
        self.memory[address as usize]
    }

    fn load_rom(&mut self, rom: &[u8]) {
        self.memory[0..rom.len()].copy_from_slice(rom);  
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
        assert_eq!(&vec[0..100], &result.ram.memory[0..100]);
    }
}
