use self::{cpu::CPU, memory::Memory, screen::Screen};
use crate::{
    rom::RomBytes,
    utils::media::{self, canvas::MediaCanvas, event::MediaEvent, RGB},
};
use sdl2::{event::Event, keyboard::Keycode};
use std::{thread::sleep, time::Duration};

pub mod cpu;
pub mod memory;
pub mod screen;

const UI_SCALE: u32 = 20;
const BG_COLOR: RGB = RGB(0, 0, 0);
const PX_COLOR: RGB = RGB(51, 255, 51);
const WINDOW_WIDHT: u32 = 64;
const WINDOW_HEIGHT: u32 = 32;
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

pub struct Machine {
    cpu: CPU,
    memory: Memory,
    display: MediaCanvas,
    event: MediaEvent,
    screen: Screen,
}

impl Machine {
    pub fn new() -> Self {
        let (display, event) =
            media::init(BG_COLOR, PX_COLOR, WINDOW_WIDHT, WINDOW_HEIGHT, UI_SCALE);

        Machine {
            cpu: CPU::default(),
            memory: Memory::new(),
            display: display,
            event: event,
            screen: Screen::new(),
        }
    }

    pub fn init(&mut self, rom_data: RomBytes) {
        self.cpu.init();

        let mut counter = 0;

        while counter < FONT_SET.len() {
            self.mem_assign((FONTSET_ADDR as usize) + counter, FONT_SET[counter]);
            counter += 1;
        }

        self.load_program(rom_data);

        self.display.draw()
    }

    fn load_program(&mut self, rom_data: RomBytes) {
        for (i, byte) in rom_data.into_iter().enumerate() {
            self.mem_assign(self.cpu.get_pc() + i, byte);
        }
    }

    // temp method for simplicity
    pub fn mem_assign(&mut self, address: usize, value: u8) {
        self.memory.assign(address, value)
    }

    pub fn v_reg_set(&mut self, index: usize, value: u8) {
        self.cpu.v_reg_set(index, value);
    }

    pub fn v_reg_get(&self, index: usize) -> u8 {
        self.cpu.v_reg_get(index)
    }

    // should be delete after tests
    pub fn i_reg_set(&mut self, value: u16) {
        self.cpu.i_reg_set(value);
    }

    pub fn run_temp(&mut self) {
        let first = self.memory.get(self.cpu.get_pc());
        let two = self.memory.get(self.cpu.get_pc() + 1);
        self.cpu.inc_pc();

        let opcode: u16 = ((first as u16) << 8) | two as u16;

        if opcode == 0 {
            return;
        }

        CPU::run(self, opcode);
    }

    pub fn run(&mut self) {
        'running: loop {
            for event in self.event.to_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    _ => {}
                }
            }

            let first = self.memory.get(self.cpu.get_pc());
            let two = self.memory.get(self.cpu.get_pc() + 1);
            self.cpu.inc_pc();

            let opcode: u16 = ((first as u16) << 8) | two as u16;

            if opcode == 0 {
                return;
            }

            CPU::run(self, opcode);

            self.display.prepare();

            for (y, row) in self.screen.get_pixels().iter().enumerate() {
                for (x, col) in row.iter().enumerate() {
                    self.display.set_pixel(x as u32, y as u32, col)
                }
            }

            self.display.draw();

            sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
