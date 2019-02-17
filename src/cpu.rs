use crate::ram::Ram;
use std::fmt;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu{
    vx: [u8; 16],
    pc: u16,
    i : u16,
    prev_pc: u16,
}

impl Cpu{
    pub fn new() -> Cpu{
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
            prev_pc: 0,
        }
    }

    pub fn run_instruction(&mut self, ram: &mut Ram){
        let hi = ram.read_byte(self.pc) as u16;
        let lo = ram.read_byte(self.pc + 1) as u16;
        let instruction = hi << 8 | lo;
        println!("Instruction read: {:#X}  hi: {:#X} lo: {:#X}  isntruction: ", instruction, hi, lo);

        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x0FF) as u8;
        let n = (instruction & 0x00F) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;
        println!("nnn={:#X}, nn={:#X}, n={:#X}, x={:#X}, y={:#X}", nnn, nn, n, x, y);

        if self.prev_pc == self.pc {
            panic!("Increment PC !!!");
        }
        self.prev_pc = self.pc;

        match (instruction & 0xF000) >> 12 {
            0x1 => {
                //goto nnn;
                self.pc = nnn;
            },
            0x6 => {
                //vx = nn;
                self.write_reg_vx(x, nn);
                self.pc += 2;
            },
            0x7 => {
                self.add_reg_vx(x, nn);
                self.pc += 2;
            },
            0xA => {
                //I = nnn
                self.i = nnn;
                self.pc += 2;
            }
            0xD => {
                self.debug_draw_sprite(ram, x, y, n);
                self.pc += 2;
            },
            0xF => {
                match (instruction & 0x00FF) {
                    0x1E => {
                        let vx = self.read_reg_vx(x);
                        self.i += vx as u16;
                    },
                    _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instruction)
                }
                self.pc += 2;
            }
            _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instruction)
        }
    }

    fn write_reg_vx(&mut self, index: u8, value: u8){
        self.vx[index as usize] = value;
    }

    fn add_reg_vx(&mut self, index: u8, value: u8){
        self.vx[index as usize] += value;
    }

    fn read_reg_vx(&mut self, index: u8) -> u8 {
        self.vx[index as usize]
    }

    fn debug_draw_sprite(&self, ram: &mut Ram, x: u8, y: u8, height: u8){
        println!("drawin sprite at ({}, {})", x, y);
        for y in 0..height {
            let mut b = ram.read_byte(self.i + y as u16);
            for _ in 0..8 {
                match (b & 0b1000_0000) >> 7 {
                    0 => print!(" "),
                    1 => print!("#"),
                    _ =>  unreachable!()
                }
                b = b << 1;
            }
            print!("\n");

        }
        print!("\n");

    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pc: {:#X}\n", self.pc);
        write!(f, "vx: ");
        for item in self.vx.iter() {
            write!(f, "{:#X} ", *item);
        }
        write!(f,"\n");
        write!(f, "I:{:#X}\n", self.i)
    }
}