use rand::Rng;


use crate::{chip8, mem::{self, Mem}};


pub struct Cpu {
    regs: [u8; 16],
    index: u16,
    pc: u16,
    stack: Vec<u16>,
    sp: u8,
    display: [u8; 64 * 32]
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { 
            regs: [0; 16],
            index: 0,
            stack: Vec::new(),
            sp: 0,
            pc: chip8::PROGRAM_START,
            display: [0; 64 * 32],
        }
    }

    pub fn run_instructions(&mut self, mem: &mut Mem){
        
        // TEST INSTRUCTION LOADING
        // for instruction in mem.memory.iter() {
        //     print!("{:#X}", instruction);
        // }

        let hi = mem.read_data(self.pc) as u16;
        let lo= mem.read_data(self.pc + 1) as u16;

        let instruction = (hi << 8) | lo;

        let x = ((instruction & 0x0F00) << 4) >> 8;
        let y = ((instruction & 0x00F0) << 8) >> 4;

        let nnn = (instruction & 0x0FFF) << 4;
        let nn = (instruction & 0x00FF) << 8;
        let n = (instruction & 0x000F) << 12;

        let kind = (instruction & 0xF000) >> 12;

        println!("{:#X}", instruction);
        println!("{:#X}", kind);

        match kind {
            0x0 => {
                match nnn {
                    0x0E0 => {
                        self.display = [0; 64*32];
                        }


                    _ => print!("default")
                }
            }

            0x1 => {
                self.pc = nnn;
                print!("TEST");
            }



            _ => print!("default")

        }

    }
}