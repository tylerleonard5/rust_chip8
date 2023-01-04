use rand::Rng;


use crate::{chip8, mem::{Mem}};


pub struct Cpu {
    regs: [u8; 16],
    index: u16,
    pc: u16,
    stack: Vec<u16>,
    sp: u8,
    pub display: [u8; 64 * 32]
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

        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;

        let nnn = instruction & 0x0FFF;
        let nn = instruction & 0x00FF;
        let n = instruction & 0x000F;

        let kind = (instruction & 0xF000) >> 12;

        // println!("{:#X}", instruction);
        // // println!("{:#X}", kind);
        // // println!("{:#X}", hi);
        // // println!("{:#X}", lo);
        // // println!("{:#X}", x);
        // // // println!("{:#X}", y);
        // // // println!("{:#X}", nnn);
        // // // println!("{:#X}", nn);
        // //  println!("{:#X}", n);
        // println!("{:#X}", self.pc);


        match kind {
            0x0 => {
                match nnn {
                    0x0E0 => {
                        println!("HERE");
                        self.display = [0; 64*32];

                        // draw display onto the screen

                        self.pc += 2;
                        }


                    _ => print!("default")
                }
            }

            0x1 => {
                self.pc = nnn;
            }
            0x6 => {
                self.regs[(x) as usize] = nn as u8;
                self.pc += 2;
            }

            0x7 => {
                let ins = self.regs[(x) as usize] + (nn as u8);
                self.regs[(x) as usize] = ins;
                self.pc += 2;
            }

            0xA => {
                self.index = nnn;
                self.pc += 2;
            }

            //IMPLEMENT DISPLAY
            0xD => {
                let mut x_cor = self.regs[(x) as usize] % 64;
                let mut y_cor = self.regs[(y) as usize] % 32;
                println!("{}", x_cor);
                println!("{}", y_cor);
                println!("{}", self.display.len());
                println!("{}", (((y_cor as u16)*(64))) + x_cor as u16);
                self.regs[15] = 0;

                for i in 0..n { // each i is the row of sprite data
                    x_cor = self.regs[(x) as usize] % 64;
                    let mut data = mem.read_data(self.index + i); // data from index

                    for _i in 0..8 {
                        let curr_bit = data >> 7;

                        if curr_bit == 1 && self.display[((((y_cor as u16)*(64))) + x_cor as u16) as usize] == 1 {
                            self.display[((((y_cor as u16)*(64))) + x_cor as u16) as usize] = 0;
                            self.regs[15] = 1;
                        }else if curr_bit == 1 && self.display[((((y_cor as u16)*(64))) + x_cor as u16) as usize] == 0 {
                            self.display[((((y_cor as u16)*(64))) + x_cor as u16) as usize] = 1;
                        }
                        
                        if x_cor > 63 {
                            break;
                        }
                        x_cor += 1;

                        data = data << 1;

                    }
                    y_cor += 1;

                    if y_cor > 31 {
                        break;
                    }
                }

                // draw display on screen
                
                self.pc += 2;
            }
            _ => print!("default")

        }

    }
}