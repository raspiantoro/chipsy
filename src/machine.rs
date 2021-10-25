use self::{cpu::CPU, memory::Memory};

pub mod cpu;
pub mod memory;

static mut CLEAR_SCREEN: u8 = 0;
static FONTSET_ADDR: u16 = 0x50;
static FONT_SET: [u8; 80] = [
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
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            cpu: CPU::default(),
            memory: Memory::new(),
        }
    }

    pub fn init(&mut self) {
        self.cpu.init();

        let mut counter = 0;

        while counter < FONT_SET.len() {
            self.mem_assign((FONTSET_ADDR as usize) + counter, FONT_SET[counter]);
            counter += 1;
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

    pub fn run(&mut self) {
        loop {
            let first = self.memory.get(self.cpu.get_pc());
            let two = self.memory.get(self.cpu.get_pc() + 1);
            self.cpu.inc_pc();

            let opcode: u16 = ((first as u16) << 8) | two as u16;

            if opcode == 0 {
                return;
            }

            CPU::run(self, opcode);

            if unsafe { CLEAR_SCREEN == 1 } {
                // add clear screen operations later
                unsafe { CLEAR_SCREEN = 0 }
            }
        }
    }
}
