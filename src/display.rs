const WIDTH: usize =  64;
const HEIGHT: usize = 32;

pub struct Display {
    screen: [[u8; WIDTH]; HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [[0; WIDTH]; HEIGHT],
        }
    }

    pub fn debug_draw_byte(&mut self, mut byte: u8, x: u8, y: u8) -> bool{
        let mut flipped = false;
        let mut coord_x = x as usize;
        let coord_y = y as usize;

        for _ in 0..8 {
            match (byte & 0b1000_0000) >> 7 {
                0 => {
                    if self.screen[coord_y][coord_x] == 1{
                        flipped = true;
                    }
                    self.screen[coord_y][coord_x] = 0;  
                }
                1 => self.screen[coord_y][coord_x] = 1,
                _ =>  unreachable!()
            }
            coord_x += 1;
            byte = byte << 1;
        }
        self.present();
        flipped
    }

    fn present(&self){
        for y in 0..HEIGHT{
            for x in 0..WIDTH{
                if self.screen[y][x] == 0 {
                    print!("_");
                } else {
                    print!("#");
                }
            }
            print!("\n");
        }
    }
}