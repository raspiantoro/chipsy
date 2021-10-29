use std::char::from_digit;

use sdl2::keyboard::Keycode;

use super::KEYBOARD_SIZE;

pub struct Keyboard {
    keys: [Key; KEYBOARD_SIZE],
}

impl Keyboard {
    pub fn new() -> Self {
        let keys = [
            Key::new(Keycode::Num1, Keycode::Num1),
            Key::new(Keycode::Num2, Keycode::Num2),
            Key::new(Keycode::Num3, Keycode::Num3),
            Key::new(Keycode::Num4, Keycode::C),
            Key::new(Keycode::Q, Keycode::Num4),
            Key::new(Keycode::W, Keycode::Num5),
            Key::new(Keycode::E, Keycode::Num6),
            Key::new(Keycode::R, Keycode::D),
            Key::new(Keycode::A, Keycode::Num7),
            Key::new(Keycode::S, Keycode::Num8),
            Key::new(Keycode::D, Keycode::Num9),
            Key::new(Keycode::F, Keycode::E),
            Key::new(Keycode::Z, Keycode::A),
            Key::new(Keycode::X, Keycode::Num0),
            Key::new(Keycode::C, Keycode::B),
            Key::new(Keycode::V, Keycode::F),
        ];

        Keyboard { keys }
    }

    pub fn update_down_state(&mut self, keycode: Keycode, down: bool) {
        for key in self.keys.iter_mut() {
            if key.map_key == keycode {
                key.down = down;
            }
        }
    }

    pub fn key_is_down(&self, chip_key: i32) -> bool {
        let ascii_key = match from_digit(chip_key as u32, 10) {
            Some(c) => c,
            None => return false,
        };

        match Keycode::from_i32(ascii_key as i32) {
            Some(some_key) => {
                for key in self.keys.iter() {
                    if key.chip_key == some_key {
                        return key.down;
                    }
                }
                return false;
            }
            None => return false,
        }
    }

    pub fn get_chip_key(&self, map_key: Keycode) -> i8 {
        for key in self.keys.iter() {
            if key.map_key == map_key {
                return key.chip_key as i8;
            }
        }

        return -1;
    }
}

struct Key {
    map_key: Keycode,
    chip_key: Keycode,
    down: bool,
}

impl Key {
    fn new(map_key: Keycode, chip_key: Keycode) -> Self {
        Key {
            map_key,
            chip_key,
            down: false,
        }
    }
}
