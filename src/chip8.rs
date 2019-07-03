use rand;
use rand::Rng;

use crate::display::Display;
use crate ::font::FONT_SET;

use crate::DISPLAY_HEIGHT;
use crate::DISPLAY_WIDTH;

const STACK_SIZE : usize = 16;
const PROGRAM_START : usize = 0x200;

pub struct Output {
    pub beep: bool,
}

pub struct Chip8 {
    vram: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    pub memory : [u8; 4096],

    v : [u8; 16],
    i : usize,
    pc : usize,
    prev_pc: usize,
    pub stack : [usize; STACK_SIZE],
    sp : usize,
    display : Display,
    keypad: [bool; 16],
    keypad_waiting: bool,
    keypad_register: usize,
    delay_timer: u8,
    sound_timer: u8,

}

impl Chip8 {
    pub fn new(sdl_context: &sdl2::Sdl) -> Chip8 {
        let mut memory = [0u8; 4096];
        for i in 0..FONT_SET.len() {
            memory[i] = FONT_SET[i];
        }
        Chip8 {
            vram: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            memory: memory,
            v: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            prev_pc: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
            display: Display::new(&sdl_context),
            keypad: [false; 16],
            keypad_waiting: false,
            keypad_register: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn tick(&mut self, keypad: [bool; 16]) -> Output {
        self.keypad = keypad;

        if self.keypad_waiting {
            for i in 0..keypad.len(){
                if keypad[i] {
                    self.keypad_waiting = false;
                    self.v[self.keypad_register] = i as u8;
                    break
                }

            }
        }
        else {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }
            self.run_opcode();
        }
        Output {
            beep: self.sound_timer > 0,
        }
    }

    fn run_opcode(&mut self){
        let hi = self.memory[self.pc] as u16;
        let lo = self.memory[self.pc + 1] as u16;

        let opcode = hi << 8 | lo;

        println!("opcode read: {:#X} hi: {:#X} lo: {:#X}", opcode, hi, lo);

        if self.prev_pc == self.pc {
            //panic!("increment PC!!!");
        }

        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x0FF) as u8;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = ((opcode & 0x000F)) as usize;
        self.prev_pc = self.pc;

        match (opcode & 0xF000) >> 12 {
            0x0 => {
                match kk {
                    0xE0 => {
                        for y in 0..DISPLAY_HEIGHT {
                            for x in 0..DISPLAY_WIDTH {
                                self.vram[y][x] = 0;
                            }
                        }
                        self.display.draw(&self.vram);
                        self.pc += 2;
                    }
                    0xEE => {
                        self.sp -= 1 ;
                        self.pc = self.stack[self.sp];
                    }
                    
                    _ => panic!("Unrecognized opcode {:#X}:{:#X}", self.pc, opcode) 
                }
            }
            0x1 => {
                self.pc = nnn;
            }
            0x2 => {
                self.stack[self.sp] = self.pc + 2;
                self.sp += 1;
                self.pc = nnn;
            }
            0x3 => {
                if self.v[x] == kk {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x4 => {
                if self.v[x] != kk{
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x5 => {
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x6 => {
                self.v[x] = kk;
                self.pc += 2;
            }
            0x7 => {
                let vx =self.v[x] as u16;
                let val = kk as u16;
                let result = vx + val;
                self.v[x] = result as u8;
                self.pc += 2;
            }
            0x8 => {
                match n {
                    0x0 => {
                        self.v[x] = self.v[y];
                        self.pc += 2;
                    }
                    0x1 => {
                        self.v[x] |= self.v[y];
                        self.pc += 2;
                    }
                    0x2 => {
                        self.v[x] &= self.v[y];
                        self.pc += 2;
                    }
                    0x3 => {
                        self.v[x] ^= self.v[y];
                        self.pc += 2;
                    }
                    0x4 => {
                        let vx = self.v[x] as u16;
                        let vy = self.v[y] as u16;
                        let result = vx + vy;
                        self.v[x] = result as u8;
                        self.v[0x0F] = if result > 0xFF { 1 } else { 0 };
                        self.pc += 2;

                    }
                    0x5 => {
                        self.v[0x0F] = if self.v[x] > self.v[y] {1} else {0};
                        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                        self.pc += 2;
                    }
                    0x6 => {
                        self.v[0x0F] = self.v[x] & 1;
                        self.v[x] >>= 1;
                        self.pc += 2;
                    }
                    0x7 => {
                        self.v[0x0F] = if self.v[y] > self.v[x] {1} else {0};
                        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                        self.pc += 2;
                    }
                    0xE => {
                        self.v[0x0F] = (self.v[x] & 0b10000000) >> 7;
                        self.v[x] <<=1;
                        self.pc += 2;
                    }
                    _ => panic!("Unrecognized opcode {:#X}:{:#X}", self.pc, opcode) 
                }
            }
            0x9 => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0xA => {
                self.i = nnn;
                self. pc += 2;
            }
            0xB => {
                self.pc = nnn + self.v[0] as usize;
            }
            0xC => {
                let mut rng = rand::thread_rng();
                self.v[x] = rng.gen::<u8>() & kk;
                self.pc += 2;
            }
            0xD => {
                self.v[0x0F] = 0;
                for byte in 0..n {
                    let y = (self.v[y] as usize + byte) % DISPLAY_HEIGHT;
                    for bit in 0..8 {
                        let x = (self.v[x] as usize + bit) % DISPLAY_WIDTH;
                        let color = (self.memory[self.i as usize + byte] >> (7 - bit)) & 1;
                        self.v[0x0F] |= color & self.vram[y][x];
                        self.vram[y][x] ^= color;
                    }

                }
                self.display.draw(&self.vram);
                self.pc += 2;
            }
            0xE => {
                match kk {
                    0x9E => {
                        if self.keypad[self.v[x] as usize] {
                            self.pc += 2;
                        }
                        self.pc += 2;
                    }
                    0xA1 => {
                        if !self.keypad[self.v[x] as usize] {
                            self.pc += 2;
                        }
                        self.pc += 2;
                    }

                    _ => panic!("Unrecognized opcode {:#X}:{:#X}", self.pc, opcode) 
                }
            }
            0xF => {
                match kk {
                    0x07 => {
                        self.v[x] = self.delay_timer;
                        self.pc += 2;
                    }
                    0x0A => {
                        self.keypad_waiting = true;
                        self.keypad_register = x;
                        self.pc += 2;
                    }
                    0x15 => {
                        self.delay_timer = self.v[x];
                        self.pc += 2;
                    }
                    0x18 => {
                        self.sound_timer = self.v[x];
                        self.pc += 2;
                    }
                    0x1E => {
                        self.i += self.v[x] as usize;
                        self.pc += 2; 
                    }
                    0x29 => {
                        self.i = self.v[x] as usize * 5;
                        self.pc += 2;
                    }
                    0x33 => {
                        self.memory[self.i] = self.v[x] / 100;
                        self.memory[self.i + 1] = (self.v[x] % 100) / 10;
                        self.memory[self.i + 2] = self.v[x] % 10;
                        self.pc += 2;
                    }
                    0x55 => {
                        for i in 0..x + 1 {
                            self.memory[self.i + i] = self.v[i];
                        }
                        self.pc += 2;
                    }
                    0x65 => {
                        for i in 0..x +1 {
                            self.v[i] = self.memory[self.i + i];
                        }
                        self.pc += 2;
                        
                    }
                    _ => panic!("Unrecognized opcode {:#X}:{:#X}", self.pc, opcode) 
                }
            }
            _ => panic!("Unrecognized opcode {:#X}:{:#X}", self.pc, opcode)
        }
    }


    pub fn load_rom(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.memory[0x200 + i] = byte;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
#[path = "./processor_test.rs"]
mod processor_test;