pub const KEYBOARD_SIZE: usize = 16;

#[derive(PartialEq, Clone, Copy)]
pub enum Key {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    Unknown,
}

impl From<u8> for Key {
    fn from(orig: u8) -> Self {
        let key = match orig {
            0 => Self::Key0,
            1 => Self::Key1,
            2 => Self::Key2,
            3 => Self::Key3,
            4 => Self::Key4,
            5 => Self::Key5,
            6 => Self::Key6,
            7 => Self::Key7,
            8 => Self::Key8,
            9 => Self::Key9,
            10 => Self::KeyA,
            11 => Self::KeyB,
            12 => Self::KeyC,
            13 => Self::KeyD,
            14 => Self::KeyE,
            15 => Self::KeyF,
            _ => Self::Unknown,
        };

        key
    }
}

impl Into<i8> for Key {
    fn into(self) -> i8 {
        let key_number = match self {
            Key::Key0 => 0,
            Key::Key1 => 1,
            Key::Key2 => 2,
            Key::Key3 => 3,
            Key::Key4 => 4,
            Key::Key5 => 5,
            Key::Key6 => 6,
            Key::Key7 => 7,
            Key::Key8 => 8,
            Key::Key9 => 9,
            Key::KeyA => 10,
            Key::KeyB => 11,
            Key::KeyC => 12,
            Key::KeyD => 13,
            Key::KeyE => 14,
            Key::KeyF => 15,
            Key::Unknown => -1,
        };

        key_number
    }
}

pub enum Action {
    KeyUp,
    KeyDown,
}

pub trait Keyboard {
    fn scan(&self);
    fn wait(&self) -> Key;
    fn quit(&self) -> bool;
    fn key_is_down(&self, key: Key) -> bool;
}
