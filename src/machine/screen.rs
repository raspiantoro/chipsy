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

        for (row, byte) in sprites.iter().enumerate() {
            for col in 0..8 {
                let (x_pos, y_pos) = (
                    (x + col) % WINDOW_WIDHT as usize,
                    (y + row) % WINDOW_HEIGHT as usize,
                );

                let pixel = (byte & (0b_1000_0000 >> col)) >> 7 - col;

                if pixel == 0 {
                    continue;
                }

                let mut xored = 0;

                if self.pixels[y_pos][x_pos] == 1 && pixel == 1 {
                    collision = true;
                    xored = 1;
                }

                self.pixels[y_pos][x_pos] = pixel ^ xored;
            }
        }

        return collision;
    }
}
