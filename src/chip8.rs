use crate::mem::Mem;
const PROGRAM_START: u16 = 0x200;
pub struct Chip8 {
    pub mem: Mem
}

impl Chip8 {

    // Initialize Chip8
    pub fn new() -> Chip8{
        Chip8 { 
            mem: Mem::new()

        
        }
    }

    // Takes data and writes it to the bus starting at the 0x200 address
    pub fn load_data(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.mem.write_data((PROGRAM_START as usize + i) as u16, data[i]);
        }
    }
}