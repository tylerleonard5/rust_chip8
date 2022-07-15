pub struct Mem {
    pub memory: [u8; 4096],
}

impl Mem {

    // Creates new instance of memory with interpteter values set
    pub fn new() -> Mem {
        let mut temp = Mem {
            memory: [0; 4096],
        };

        let interpreter: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ];

        let mut count = 0;
        for sprite in interpreter{
            for i in sprite{
                temp.memory[0x050 + count] = i;
                count += 1;
            }
        }

        temp
    }

    // Writes bytes into memory starting at an address
    pub fn write_data(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    pub fn read_data (&mut self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}