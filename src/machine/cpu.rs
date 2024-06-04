use self::instructions::eval;
use super::{audio::Audio, display::Display, keyboard::Keyboard, Machine};

mod instructions;

#[derive(Default)]
pub struct CPU {
    v: [u8; 16],
    i: u16,
    st: u8,
    dt: u8,
    pc: usize,
    sp: usize,
    stack: [u16; 16],
}

impl CPU {
    pub fn init(&mut self) {
        self.pc = 0x200;
    }

    pub fn v_reg_set(&mut self, index: usize, value: u8) {
        self.v[index] = value;
    }

    pub fn v_reg_get(&self, index: usize) -> u8 {
        self.v[index]
    }

    // should be delete after tests
    pub fn i_reg_set(&mut self, value: u16) {
        self.i = value
    }

    pub fn get_dt(&self) -> u8 {
        self.dt
    }

    pub fn dec_dt(&mut self) {
        self.dt -= 1
    }

    pub fn get_st(&self) -> u8 {
        self.st
    }

    pub fn reset_st(&mut self) {
        self.st = 0
    }

    pub fn get_pc(&self) -> usize {
        self.pc
    }

    pub fn inc_pc(&mut self) {
        self.pc += 2;
    }

    pub fn execute<D, K, A>(machine: &mut Machine<D, K, A>, opcode: u16)
    where
        D: Display,
        K: Keyboard,
        A: Audio
    {
        eval(machine, opcode);
    }
}
