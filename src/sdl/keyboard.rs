use std::cell::RefCell;

use super::error::SdlError;
use chipsy::machine::{self, keyboard};
use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};

type MachineKey = machine::keyboard::Key;

#[derive(Clone, Copy, PartialEq)]
enum KeyState {
    Up,
    Down,
}

pub struct Key {
    machine_key: MachineKey,
    keycode: Keycode,
    state: RefCell<KeyState>,
}

impl Key {
    fn new(keycode: Keycode, machine_key: MachineKey) -> Self {
        Key {
            machine_key,
            keycode,
            state: RefCell::new(KeyState::Up),
        }
    }
}

pub struct Keyboard {
    keys: [Key; keyboard::KEYBOARD_SIZE],
    event_pump: RefCell<EventPump>,
    is_quit: RefCell<bool>,
}

impl Keyboard {
    pub fn new(context: &Sdl) -> Result<Self, SdlError> {
        let keys = [
            Key::new(Keycode::Num1, MachineKey::Key1),
            Key::new(Keycode::Num2, MachineKey::Key2),
            Key::new(Keycode::Num3, MachineKey::Key3),
            Key::new(Keycode::Num4, MachineKey::KeyC),
            Key::new(Keycode::Q, MachineKey::Key4),
            Key::new(Keycode::W, MachineKey::Key5),
            Key::new(Keycode::E, MachineKey::Key6),
            Key::new(Keycode::R, MachineKey::KeyD),
            Key::new(Keycode::A, MachineKey::Key7),
            Key::new(Keycode::S, MachineKey::Key8),
            Key::new(Keycode::D, MachineKey::Key9),
            Key::new(Keycode::F, MachineKey::KeyE),
            Key::new(Keycode::Z, MachineKey::KeyA),
            Key::new(Keycode::X, MachineKey::Key0),
            Key::new(Keycode::C, MachineKey::KeyB),
            Key::new(Keycode::V, MachineKey::KeyF),
        ];

        let event_pump = context.event_pump().map_err(SdlError::KeyboardError)?;
        let event_pump = RefCell::new(event_pump);

        Ok(Self {
            keys,
            event_pump,
            is_quit: RefCell::new(false),
        })
    }

    fn update_state(&self, keycode: Keycode, state: KeyState) {
        for key in self.keys.iter() {
            if key.keycode == keycode {
                *key.state.borrow_mut() = state;
            }
        }
    }

    pub fn get_machine_key(&self, keycode: Keycode) -> MachineKey {
        for key in self.keys.iter() {
            if key.keycode == keycode {
                return key.machine_key;
            }
        }

        return MachineKey::Unknown;
    }
}

impl keyboard::Keyboard for Keyboard {
    fn scan(&self) {
        let mut event_pump = self.event_pump.borrow_mut();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    *self.is_quit.borrow_mut() = true;
                }

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => self.update_state(keycode, KeyState::Down),

                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => self.update_state(keycode, KeyState::Up),

                _ => return,
            }
        }
    }

    fn wait(&self) -> MachineKey {
        let mut event_pump = self.event_pump.borrow_mut();
        let mut machine_key: MachineKey = MachineKey::Unknown;

        for event in event_pump.wait_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    machine_key = self.get_machine_key(keycode);
                    if machine_key != MachineKey::Unknown {
                        break;
                    } else {
                        continue;
                    }
                }
                _ => continue,
            }
        }

        machine_key
    }

    fn quit(&self) -> bool {
        if *self.is_quit.borrow() {
            true
        } else {
            false
        }
    }

    fn key_is_down(&self, key: keyboard::Key) -> bool {
        for k in self.keys.iter() {
            if k.machine_key == key {
                return *k.state.borrow() == KeyState::Down;
            }
        }

        return false;
    }
}
