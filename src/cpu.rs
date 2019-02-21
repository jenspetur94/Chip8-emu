use std::fmt;
use crate::bus::Bus;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu{
    vx: [u8; 16],
    pc: u16,
    i : u16,
    prev_pc: u16,
    ret_stack: Vec<u16>

}

impl Cpu{
    pub fn new() -> Cpu{
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
            prev_pc: 0,
            ret_stack: Vec::<u16>::new(),
        }
    }

    pub fn run_instruction(&mut self, bus: &mut Bus){
        let hi = bus.ram_read_byte(self.pc) as u16;
        let lo = bus.ram_read_byte(self.pc + 1) as u16;
        let instruction = hi << 8 | lo;
        println!("Instruction read: {:#X}  hi: {:#X} lo: {:#X}", instruction, hi, lo);

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
            0x2 => {
                //Call subroutine at address NNN
                self.ret_stack.push(self.pc + 2);
                self.pc = nnn;
            },
            0x3 => {
                let vx = self.read_reg_vx(x);
                if vx == nn {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x6 => {
                //vx = nn;
                self.write_reg_vx(x, nn);
                self.pc += 2;
            },
            0x7 => {
                let vx = self.read_reg_vx(x);
                self.write_reg_vx(x, vx.wrapping_add(nn));
                self.pc += 2;
            },
            0x8 => {
                match n {
                    0 => {
                        //vx=vy
                        let vy = self.read_reg_vx(y);
                        self.write_reg_vx(x, vy);
                        self.pc += 2;
                    },
                    _ => panic!("Unrecognized 0x8xy* instruction {:#X}:{:#X}", self.pc, instruction)
                }
            }
            0xA => {
                //I = nnn
                self.i = nnn;
                self.pc += 2;
            }
            0xD => {
                self.debug_draw_sprite(bus, x, y, n);
                self.pc += 2;
            },
            0xE => {
                match nn {
                    0xA1 => {
                        //if(key() != vx) skip then next instruction
                        let key = self.read_reg_vx(x);
                        if bus.key_pressed(key) {
                            self.pc += 2;
                        } else {
                            self.pc += 4;
                        }
                    },
                    _ => panic!("Unrecognized 0xEx** instruction {:#X}:{:#X}", self.pc, instruction)
                }
            }
            0xF => {
                match (nn) {
                    //I += Vx
                    0x1E => {
                        let vx = self.read_reg_vx(x);
                        self.i += vx as u16;
                        self.pc += 2;
                    },
                    _ => panic!("Unrecognized 0xFx** instruction {:#X}:{:#X}", self.pc, instruction)
                }
            }
            _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instruction)
        }
    }

    fn write_reg_vx(&mut self, index: u8, value: u8){
        self.vx[index as usize] = value;
    }

    fn read_reg_vx(&mut self, index: u8) -> u8 {
        self.vx[index as usize]
    }

    fn debug_draw_sprite(&mut self, bus: &mut Bus, x: u8, y: u8, height: u8){
        println!("drawin sprite at ({}, {})", x, y);
        let mut should_set_vf = false;
        for y in 0..height {
            let b = bus.ram_read_byte(self.i + y as u16);
            if bus.debug_draw_byte( b, x, y) {
                should_set_vf = true;
            }
        }
        if should_set_vf{
            self.write_reg_vx(0xF, 1);
        } else {
            self.write_reg_vx(0xF, 0);
        }
        

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