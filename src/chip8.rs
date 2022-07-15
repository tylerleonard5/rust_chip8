use crate::bus::Bus;
pub struct Chip8 {
    pub bus: Bus
}

impl Chip8 {

    // Initialize Chip8
    pub fn new() -> Chip8{
        Chip8 { bus: Bus::new() }
    }

    // Takes data and writes it to the bus starting at the 0x200 address
    pub fn load_data(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.bus.write_data((0x200 + i) as u16, data[i]);
        }
    }
}