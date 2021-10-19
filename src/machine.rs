use self::{cpu::CPU, memory::Memory};

pub mod cpu;
pub mod memory;

pub struct Machine {
    cpu: CPU,
    memory: Memory,
}

impl Machine {
    pub fn New() -> Self {
        Machine {
            cpu: CPU::default(),
            memory: Memory::new(),
        }
    }

    pub fn init(&mut self) {
        self.cpu.init();
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

    // temp method
    pub fn run(&mut self) {
        loop {
            let first = self.memory.get(self.cpu.get_pc());
            let two = self.memory.get(self.cpu.get_pc() + 1);
            self.cpu.inc_pc();

            let opcode: u16 = ((first as u16) << 8) | two as u16;

            if opcode == 0 {
                return;
            }

            self.cpu.run(opcode);
        }
    }
}
