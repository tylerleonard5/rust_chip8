use std::fs::File;
use std::io::Read;

use chip8::Chip8;

mod chip8;
mod ram;

fn main() {
    let mut gamefile = match File::open("gameData/INVADERS") {
        Ok(file) => file,
        Err(_) => panic!("Couldn't open file"),
    };

    let mut game_data = Vec::new();

    gamefile.read_to_end(&mut game_data).expect("Couldn't read");

    let mut chip8 = Chip8::new();
    chip8.load_rom(&game_data);
}
