use std::fs::File;
use std::io::Read;

use chip8::Chip8;

mod chip8;
mod mem;
mod bus;
mod cpu;

fn main() {
    // load game file
    let mut gamefile = match File::open("gameData/INVADERS") {
        Ok(file) => file,
        Err(_) => panic!("Couldn't open file"),
    };

    // Store file data in array of bytes
    let mut game_data = Vec::new();

    gamefile.read_to_end(&mut game_data).expect("Couldn't read");


    // Initialize a chip8 instance
    let mut chip8 = Chip8::new();

    // load data into chip8 instance
    chip8.load_data(&game_data);

    // // TESTING THE DATA LOADING
    // for i in 0..chip8.bus.mem.memory.len(){
    //     print!("{:#X}", chip8.bus.mem.memory[i]);
    // }

}
