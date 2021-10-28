use sdl2::keyboard::Keycode;

use super::KEYBOARD_SIZE;

pub struct Keyboard {
    keys: [Key; KEYBOARD_SIZE],
}

impl Keyboard {
    pub fn new() -> Self {
        let keys = [
            Key::new(Keycode::Num1),
            Key::new(Keycode::Num2),
            Key::new(Keycode::Num3),
            Key::new(Keycode::Num4),
            Key::new(Keycode::Q),
            Key::new(Keycode::W),
            Key::new(Keycode::E),
            Key::new(Keycode::R),
            Key::new(Keycode::A),
            Key::new(Keycode::S),
            Key::new(Keycode::D),
            Key::new(Keycode::F),
            Key::new(Keycode::Z),
            Key::new(Keycode::X),
            Key::new(Keycode::C),
            Key::new(Keycode::V),
        ];

        Keyboard { keys }
    }

    pub fn update_down_state(&mut self, keycode: Keycode, down: bool) {
        for key in self.keys.iter_mut() {
            if key.keycode == keycode {
                key.down = down;
            }
        }
    }

    pub fn key_is_down(&self, index: usize) -> bool {
        self.keys[index].down
    }

    pub fn get_key_index(&self, keycode: Keycode) -> i8 {
        for (i, key) in self.keys.iter().enumerate() {
            if key.keycode == keycode {
                return i as i8;
            }
        }

        return -1;
    }
}

struct Key {
    keycode: Keycode,
    down: bool,
}

impl Key {
    fn new(keycode: Keycode) -> Self {
        Key {
            keycode,
            down: false,
        }
    }

    // pub fn get_keycode(&self) -> Keycode {
    //     self.keycode
    // }

    // pub fn update_state(&mut self, down: bool) {
    //     self.down = down;
    // }
}
