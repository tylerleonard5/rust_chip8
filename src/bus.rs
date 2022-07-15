use crate::mem::Mem;

pub struct Bus {
    pub mem: Mem
}

impl Bus {

    // Initialize memory instance in bus
    pub fn new() -> Bus{
        Bus {
            mem: Mem::new()
        }
    }

    // Writes data to memory
    pub fn write_data_toMem(&mut self, address: u16, data: u8){
        self.mem.write_bytes_mem(address, data);
    }
}