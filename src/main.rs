use std::fs::File;
use std::io::Read;
use winit::event::VirtualKeyCode;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use chip8::Chip8;

mod chip8;
mod mem;
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
    chip8.run_instructions();

    // // TESTING THE DATA LOADING
    //  for i in 0..chip8.mem.memory.len(){
    //      print!("{:#X}", chip8.mem.memory[i]);
    //   }


    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
            .with_title("Rust by Example: Winit!")
            .build(&event_loop)
            .unwrap();

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_released(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            let mouse_diff = input.mouse_diff();
            if mouse_diff != (0.0, 0.0) {
                println!("Mouse diff is: {:?}", mouse_diff);
                println!("Mouse position is: {:?}", input.mouse());
            }
        }
    });

}
