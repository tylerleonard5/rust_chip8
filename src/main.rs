use std::fs::File;
use std::io::Read;

fn main() {
    let mut gamefile = match File::open("gameData/INVADERS") {
        Ok(file) => file,
        Err(_) => panic!("Couldn't open file"),
    };

    let mut game_data = Vec::new();

    gamefile.read_to_end(&mut game_data).expect("Couldn't read");

}
