use std::fs::File;
use std::io::Read;
use crate::chip8::Chip8;
extern crate glium;

mod ram;
mod cpu;
mod chip8;
mod display;
mod keyboard;
mod bus;

fn main() {
    let mut file = File::open("Data/INVADERS").unwrap();

    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    let mut events_loop = glium::glutin::EventsLoop::new();

    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(glium::glutin::dpi::LogicalSize::new(64.0, 32.0))
        .with_title("CHIP 8");
    
    let context = glium::glutin::ContextBuilder::new();

    let display = glium::Display::new(window, context, &events_loop).unwrap();


    loop {
        chip8.run_instruction();
    }
}
