use rand::Rng;


use crate::chip8;


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

    
}