use super::{WINDOW_HEIGHT, WINDOW_WIDHT};

pub struct Screen {
    pixels: [[u8; WINDOW_WIDHT as usize]; WINDOW_HEIGHT as usize],
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            pixels: [[0; WINDOW_WIDHT as usize]; WINDOW_HEIGHT as usize],
        }
    }

    pub fn clear(&mut self) {
        self.pixels = [[0; WINDOW_WIDHT as usize]; WINDOW_HEIGHT as usize]
    }

    pub fn get_pixels(&self) -> [[u8; WINDOW_WIDHT as usize]; WINDOW_HEIGHT as usize] {
        self.pixels
    }

    pub fn set_pixels(&mut self, x: usize, y: usize, sprites: Vec<u8>) -> bool {
        let mut collision = false;
        for (row, val) in sprites.iter().enumerate() {
            let byte = *val >> 4;
            for col in 0..4 {
                let pixel = (byte & (0b1000 >> col)) >> 3 - col;
                let mut xored = 0;

                if self.pixels[y + row][x + col] == 1 && pixel == 1 {
                    collision = true;
                    xored = 1;
                }

                self.pixels[y + row][x + col] = pixel ^ xored;
            }
        }

        return collision;
    }
}
