use std::borrow::{Borrow, BorrowMut};

use crate::machine::{self, Machine};

use super::CPU;

pub fn execute(machine: &mut Machine, opcode: u16) {
    let cpu = machine.cpu.borrow_mut();
    let memory = machine.memory.borrow();
    let screen = machine.screen.borrow_mut();

    // notation -> hxyl
    let h: u8 = (opcode >> 12) as u8;
    let l: u8 = (opcode & 0x000F) as u8;
    let x: u8 = ((opcode >> 8) & 0x000F) as u8;
    let y: u8 = ((opcode >> 4) & 0x000F) as u8;

    // nnn variable. x,y,l is not used when nnn is used
    let nnn: usize = (opcode & 0x0FFF) as usize;

    // kk variable. y & l is not used when nnn is used
    let kk: u8 = (opcode & 0x00FF) as u8;

    match (h, x, y, l) {
        // 00E0 - CLS
        (0, 0, 0xe, 0) => screen.clear(),

        // 1nnn - JP addr. Jump to location nnn
        (1, _, _, _) => cpu.pc = nnn,

        // 6xkk - LD Vx, byte. Set Vx = kk
        (6, _, _, _) => cpu.v[x as usize] = kk,

        // 7xkk - ADD Vx, byte. Set Vx = Vx + kk
        (7, _, _, _) => cpu.v[x as usize] += kk,

        // Annn - LD I, addr. Set register I = nnn
        (0xa, _, _, _) => cpu.i = nnn as u16,

        // Dxyn - DRW Vx, Vy, nibble. Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
        (0xd, _, _, _) => {
            cpu.v[0xf] = 0;
            let sprites = memory.get_range(cpu.i as usize, l as usize);
            if screen.set_pixels(
                cpu.v_reg_get(x as usize) as usize,
                cpu.v_reg_get(y as usize) as usize,
                sprites,
            ) {
                cpu.v[0xf] = 1;
            }
        }

        // arithmetic instructions
        (8, _, _, _) => arithmetic(cpu, x, y, l),

        // invalid opcode
        (_, _, _, _) => panic!("unknown instruction"),
    }
}

fn arithmetic(cpu: &mut CPU, x: u8, y: u8, l: u8) {
    match l {
        // 8xy4 - ADD Vx, Vy. Set Vx = Vx + Vy, set VF = carry
        4 => {
            let (val, overflow) = cpu.v[x as usize].overflowing_add(cpu.v[y as usize]);
            cpu.v[x as usize] = val;
            if overflow {
                cpu.v[0xF] = 1;
            }
        }
        _ => panic!("unknown instruction"),
    }
}
