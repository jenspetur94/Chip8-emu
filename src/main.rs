extern crate sdl2;
use std::fs::File;
use std::io::Read;
use chip8::Chip8;
use keypad::Keypad;
use audio::Audio;
use std::thread;
use std::time::Duration;

mod chip8;
mod display;
mod keypad;
mod font;
mod audio;

const DISPLAY_WIDTH : usize = 64;
const DISPLAY_HEIGHT : usize = 32;

fn main() {
    let sleep_duration = Duration::from_millis(2);
    let mut file = File::open("Data/VERS").unwrap();

    let mut data = [0u8; 3584];


    let bytes_read = if let Ok(bytes_read) = file.read(&mut data) {
        bytes_read
    } else {
        0
    };
    let sdl_context = sdl2::init().unwrap();


    let mut chip8 = Chip8::new(&sdl_context);
    let mut keypad = Keypad::new(&sdl_context);
    let audio = Audio::new(&sdl_context);
    chip8.load_rom(&data);
    

    while  let Ok(chip8_keys) = keypad.poll() {
        let output = chip8.tick(chip8_keys);

        if output.beep {
            audio.start_beep();
        } else {
            audio.stop_beep();
        }
        thread::sleep(sleep_duration);
    }
}
