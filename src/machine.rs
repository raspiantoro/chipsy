use audio::Audio;

use self::{cpu::CPU, display::Display, keyboard::Keyboard, memory::Memory};
use crate::{rom::RomBytes, timer::TIMER};
use std::{thread::sleep, time::Duration};

pub mod cpu;
pub mod display;
pub mod keyboard;
pub mod memory;
pub mod audio;

const FONTSET_ADDR: u16 = 0x50;
const FONT_SET: [u8; 80] = [
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

pub struct Machine<'a, D, K, A>
where
    D: Display,
    K: Keyboard,
    A: Audio
{
    cpu: CPU,
    memory: Memory,
    display: &'a mut D,
    keyboard: &'a K,
    audio: &'a A
}

impl<'a, D, K, A> Machine<'a, D, K, A>
where
    D: Display,
    K: Keyboard,
    A: Audio
{
    pub fn new(display: &'a mut D, keyboard: &'a K, audio: &'a A) -> Self {
        Machine {
            cpu: CPU::default(),
            memory: Memory::new(),
            display: display,
            keyboard: keyboard,
            audio: audio
        }
    }

    pub fn init(&mut self, rom_data: RomBytes) {
        self.cpu.init();

        let mut counter = 0;

        while counter < FONT_SET.len() {
            self.memory.assign((FONTSET_ADDR as usize) + counter, FONT_SET[counter]);
            counter += 1;
        }

        self.load_program(rom_data);

        self.display.render();
    }

    fn load_program(&mut self, rom_data: RomBytes) {
        for (i, byte) in rom_data.into_iter().enumerate() {
            self.memory.assign(self.cpu.get_pc() + i, byte);
        }
    }

    pub fn v_reg_set(&mut self, index: usize, value: u8) {
        self.cpu.v_reg_set(index, value);
    }

    pub fn v_reg_get(&self, index: usize) -> u8 {
        self.cpu.v_reg_get(index)
    }

    pub fn run(&mut self) {
        loop {
            self.keyboard.scan();

            if self.keyboard.quit() {
                return;
            }

            self.display.render();

            if self.cpu.get_dt() > 0 {
                sleep(Duration::new(0, 1_000_000_000u32 / 600));
                self.cpu.dec_dt();
            }

            let mut timer = TIMER.lock().unwrap();

            match timer.state() {
                crate::timer::TimerState::Running(remaining) => {
                    timer.tick();

                    if remaining == 0 {
                        self.audio.pause();
                        timer.stop();
                    }
                },
                crate::timer::TimerState::Idle => (),
            }

            if self.cpu.get_st() > 0 {
                self.audio.resume();
                timer.start(Duration::new(0, 1_000_000_000u32 / self.cpu.get_st() as u32));
                self.cpu.reset_st();
            }

            let first = self.memory.get(self.cpu.get_pc());
            let second = self.memory.get(self.cpu.get_pc() + 1);
            self.cpu.inc_pc();

            let opcode: u16 = ((first as u16) << 8) | second as u16;

            if opcode == 0 {
                return;
            }

            CPU::execute(self, opcode);
        }
    }
}
