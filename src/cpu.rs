use rand::Rng;


use crate::{chip8, mem::{self, Mem}};


pub struct Cpu {
    regs: [u8; 16],
    index: u16,
    pc: u16,
    stack: Vec<u16>,
    sp: u8
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { 
            regs: [0; 16],
            index: 0,
            stack: Vec::new(),
            sp: 0,
            pc: chip8::PROGRAM_START,
        }
    }

    pub fn run_instructions(&mut self, mem: &mut Mem){
        
        // TEST INSTRUCTION LOADING
        // for instruction in mem.memory.iter() {
        //     print!("{:#X}", instruction);
        // }

        let hi = mem.read_data(self.pc) as u16;
        let lo= mem.read_data(self.pc + 1) as u16;

        let instruction = (hi << 8 as u8) | lo;
    }
}