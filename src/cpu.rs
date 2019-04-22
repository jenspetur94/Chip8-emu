pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu{
    mem: [u8; 4096],
    vx: [u8; 16],
    i: u16,
    pc: u16,
    ret_stack: Vec<u16>,
}

impl Cpu{
    pub fn new() -> Cpu { 
        let mut cpu = Cpu {
            mem: [0; 4096],
            vx: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            ret_stack: Vec::<u16>::new(),       
        };
        cpu.initialize_ram();

        cpu
    }

    pub fn run_instruction(&mut self){
        let hi = self.ram_read_byte(self.pc) as u16;
        let lo = self.ram_read_byte(self.pc+1) as u16;

        let instruction = hi << 8 | lo;

        println!("Instruction read: {:#X}  hi: {:#X} lo: {:#X}", instruction, hi, lo);
    }
    
    pub fn initialize_ram(&mut self) {
        let sprites: [u8; 80] = [
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2 
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80, // F
            ];
        
        for i in 0..80 {
            self.mem[i] = sprites[i];
        }
    }

    pub fn ram_read_byte(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    pub fn ram_write_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }
}

