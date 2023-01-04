use std::fs::File;
use std::io::Read;
use winit::event::{VirtualKeyCode, Event};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use pixels::{Pixels, SurfaceTexture};

use chip8::Chip8;

mod chip8;
mod mem;
mod cpu;



fn main() {
    // load game file
    let mut gamefile = match File::open("gameData/ibm.ch8") {
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
    //Loops



    // // TESTING THE DATA LOADING
    //  for i in 0..chip8.mem.memory.len(){
    //      print!("{:#X}", chip8.mem.memory[i]);
    //   }


    for i in 0..chip8.cpu.display.len(){
        print!("{:#X}", chip8.cpu.display[i]);
    }


    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
            .with_title("Chip 8 Emulator!")
            .build(&event_loop)
            .unwrap();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(64, 32, &window);
        Pixels::new(64, 32, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {

        if let Event::RedrawRequested(_) = event {
            draw(&chip8.cpu.display, pixels.get_frame());
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_released(VirtualKeyCode::Escape) || input.quit() {
                for i in 0..chip8.cpu.display.len(){
                    print!("{:?}", chip8.cpu.display[i]);
                }
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height)
            }

            chip8.run_instructions();


            window.request_redraw();
        }
    });

    fn draw(display: &[u8; 64*32], frame: &mut [u8]){
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let rgba = if display[i] == 0{
                [0x00, 0x00, 0x00, 0xff]
            } else {
                [0xff, 0xff, 0xff, 0xff]
            };
            pixel.copy_from_slice(&rgba);
        }
    }

}
