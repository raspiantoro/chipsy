const WINDOW_WIDHT: usize = 64;
const WINDOW_HEIGHT: usize = 32;

pub type Pixels = [[u8; WINDOW_WIDHT]; WINDOW_HEIGHT];

pub trait Display {
    fn get_pixels(&mut self) -> &mut Pixels;
    fn render(&self);

    fn clear(&mut self) {
        let pixels = self.get_pixels();
        *pixels = [[0; WINDOW_WIDHT]; WINDOW_HEIGHT];
    }

    fn update_pixels(&mut self, x: usize, y: usize, sprites: Vec<u8>) -> bool {
        let mut collision = false;

        let pixels = self.get_pixels();

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

                if pixels[y_pos][x_pos] == 1 && pixel == 1 {
                    collision = true;
                    xored = 1;
                }

                pixels[y_pos][x_pos] = pixel ^ xored;
            }
        }

        return collision;
    }
}
