use crate::machine::{display::Display, keyboard::Keyboard, memory::Memory, Machine, FONTSET_ADDR};
use rand::Rng;
use std::{borrow::BorrowMut, rc::Rc};

use super::CPU;

pub fn eval<D, K>(machine: &mut Machine<D, K>, opcode: u16)
where
    D: Display,
    K: Keyboard,
{
    let cpu = machine.cpu.borrow_mut();
    let memory = machine.memory.borrow_mut();
    let display = machine.display.clone();
    let keyboard = machine.keyboard.clone();

    // notation -> hxyl
    let h: u8 = (opcode >> 12) as u8;
    let x: usize = ((opcode >> 8) & 0x000F) as usize;
    let y: usize = ((opcode >> 4) & 0x000F) as usize;
    let l: u8 = (opcode & 0x000F) as u8;

    // nnn variable. x,y,l is not used when nnn is used
    let nnn: usize = (opcode & 0x0FFF) as usize;

    // kk variable. y & l is not used when nnn is used
    let kk: u8 = (opcode & 0x00FF) as u8;

    match (h, x, y, l) {
        // 00E0 - CLS
        (0, 0, 0xe, 0) => display.clear(),

        // 00EE - RET. Return from a subroutine.
        // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer
        (0, 0, 0xe, 0xe) => {
            if cpu.sp == 0 {
                panic!("stack underflow");
            }

            cpu.sp -= 1;
            cpu.pc = cpu.stack[cpu.sp] as usize;
        }

        // 1nnn - JP addr. Jump to location nnn
        (1, _, _, _) => cpu.pc = nnn,

        // 2nnn - CALL addr. Call subroutine at nnn.
        // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
        (2, _, _, _) => {
            if cpu.sp > cpu.stack.len() {
                panic!("stack overflow");
            }

            cpu.stack[cpu.sp] = cpu.pc as u16;
            cpu.sp += 1;
            cpu.pc = nnn;
        }

        // 3xkk - SE Vx, byte. Skip next instruction if Vx = kk.
        // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
        (3, _, _, _) => {
            if cpu.v[x] == kk {
                cpu.pc += 2;
            }
        }

        //4xkk - SNE Vx, byte. Skip next instruction if Vx != kk.
        // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
        (4, _, _, _) => {
            if cpu.v[x] != kk {
                cpu.pc += 2;
            }
        }

        // 5xy0 - SE Vx, Vy. Skip next instruction if Vx = Vy.
        // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
        (5, _, _, _) => {
            if cpu.v[x] == cpu.v[y] {
                cpu.pc += 2;
            }
        }

        // 6xkk - LD Vx, byte. Set Vx = kk
        (6, _, _, _) => cpu.v[x] = kk,

        // 7xkk - ADD Vx, byte. Set Vx = Vx + kk
        (7, _, _, _) => {
            let (val, _) = cpu.v[x].overflowing_add(kk);
            cpu.v[x] = val;
        }

        // arithmetic instructions
        (8, _, _, _) => arithmetic(cpu, x, y, l),

        // 9xy0 - SNE Vx, Vy. Skip next instruction if Vx != Vy.
        // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
        (9, _, _, _) => {
            if cpu.v[x] != cpu.v[y] {
                cpu.pc += 2;
            }
        }

        // Annn - LD I, addr. Set register I = nnn
        (0xa, _, _, _) => cpu.i = nnn as u16,

        // Bnnn - JP V0, addr. Jump to location nnn + V0.
        // The program counter is set to nnn plus the value of V0.
        (0xb, _, _, _) => cpu.pc = nnn + cpu.v[0] as usize,

        // Cxkk - RND Vx, byte. Set Vx = random byte AND kk.
        // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx.
        (0xc, _, _, _) => {
            let mut rng = rand::thread_rng();
            let number: u8 = rng.gen_range(0..255);
            cpu.v[x] = number & kk;
        }

        // Dxyn - DRW Vx, Vy, nibble. Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
        (0xd, _, _, _) => {
            cpu.v[0xf] = 0;
            let sprites = memory.get_range(cpu.i as usize, l as usize);
            if display.update_pixels(
                cpu.v_reg_get(x) as usize,
                cpu.v_reg_get(y) as usize,
                sprites,
            ) {
                cpu.v[0xf] = 1;
            }
        }

        (0xe, _, _, _) => keyboards(cpu, keyboard, x, y, l),

        (0xf, _, _, _) => interpreter_stuff(cpu, memory, keyboard, x, y, l),

        // invalid opcode
        (_, _, _, _) => panic!("unknown instruction"),
    }
}

fn arithmetic(cpu: &mut CPU, x: usize, y: usize, l: u8) {
    match l {
        // 8xy0 - LD Vx, Vy. Set Vx = Vy.
        // Stores the value of register Vy in register Vx.
        0 => cpu.v[x] = cpu.v[y],

        // 8xy1 - OR Vx, Vy. Set Vx = Vx OR Vy.
        // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
        1 => cpu.v[x] |= cpu.v[y],

        // 8xy2 - AND Vx, Vy. Set Vx = Vx AND Vy.
        // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
        2 => cpu.v[x] &= cpu.v[y],

        // 8xy3 - XOR Vx, Vy. Set Vx = Vx XOR Vy.
        // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
        3 => cpu.v[x] ^= cpu.v[y],

        // 8xy4 - ADD Vx, Vy. Set Vx = Vx + Vy, set VF = carry
        // The values of Vx and Vy are added together.
        // If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
        // Only the lowest 8 bits of the result are kept, and stored in Vx.
        4 => {
            cpu.v[0xF] = 0;
            let (val, overflow) = cpu.v[x].overflowing_add(cpu.v[y]);
            cpu.v[x] = val;
            if overflow {
                cpu.v[0xF] = 1;
            }
        }

        // 8xy5 - SUB Vx, Vy. Set Vx = Vx - Vy, set VF = NOT borrow.
        // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
        5 => {
            cpu.v[0xF] = 0;
            let (val, borrow) = cpu.v[x].overflowing_sub(cpu.v[y]);
            cpu.v[x] = val;
            if borrow {
                cpu.v[0xF] = 1;
            }
        }

        // 8xy6 - SHR Vx {, Vy}. Set Vx = Vx SHR 1.
        // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
        6 => {
            cpu.v[0xf] = 0;
            if cpu.v[x] & 0x1 == 1 {
                cpu.v[0xf] = 1;
            }
            cpu.v[x] = cpu.v[x] >> 1;
        }

        // 8xy7 - SUBN Vx, Vy. Set Vx = Vy - Vx, set VF = NOT borrow.
        // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
        7 => {
            cpu.v[0xF] = 0;
            let (val, borrow) = cpu.v[y].overflowing_sub(cpu.v[x]);
            cpu.v[x as usize] = val;
            if borrow {
                cpu.v[0xF] = 1;
            }
        }

        // 8xyE - SHL Vx {, Vy}. Set Vx = Vx SHL 1.
        // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
        0xe => {
            cpu.v[0xf] = 0;
            if cpu.v[x] & 0x80 == 128 {
                cpu.v[0xf] = 1;
            }
            cpu.v[x] = cpu.v[x] << 1;
        }

        _ => panic!("unknown instruction"),
    }
}

fn keyboards<K: Keyboard>(cpu: &mut CPU, keyboard: Rc<K>, x: usize, y: usize, l: u8) {
    match (y, l) {
        // Ex9E - SKP Vx. Skip next instruction if key with the value of Vx is pressed.
        // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
        (9, 0xe) => {
            if keyboard.key_is_down(cpu.v[x].into()) {
                cpu.pc += 2;
            }
        }

        // ExA1 - SKNP Vx. Skip next instruction if key with the value of Vx is not pressed.
        // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
        (0xa, 1) => {
            if !keyboard.key_is_down(cpu.v[x].into()) {
                cpu.pc += 2;
            }
        }

        _ => panic!("unknown instruction"),
    }
}

fn interpreter_stuff<K: Keyboard>(
    cpu: &mut CPU,
    memory: &mut Memory,
    keyboard: Rc<K>,
    x: usize,
    y: usize,
    l: u8,
) {
    match (y, l) {
        // Fx07 - LD Vx, DT. Set Vx = delay timer value.
        // The value of DT is placed into Vx.
        (0, 7) => cpu.v[x] = cpu.dt,

        // Fx0A - LD Vx, K. Wait for a key press, store the value of the key in Vx.
        // All execution stops until a key is pressed, then the value of that key is stored in Vx.
        (0, 0xa) => {
            let key: i8 = keyboard.wait().into();
            cpu.v[x] = key as u8;
        }

        // Fx15 - LD DT, Vx. Set delay timer = Vx.
        // DT is set equal to the value of Vx.
        (1, 5) => cpu.dt = cpu.v[x],

        // Fx18 - LD ST, Vx. Set sound timer = Vx.
        // ST is set equal to the value of Vx.
        (1, 8) => cpu.st = cpu.v[x],

        // Fx1E - ADD I, Vx. Set I = I + Vx.
        // The values of I and Vx are added, and the results are stored in I.
        (1, 0xe) => cpu.i = cpu.i + cpu.v[x] as u16,

        // Fx29 - LD F, Vx. Set I = location of sprite for digit Vx.
        // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
        (2, 9) => cpu.i = (cpu.v[x] as u16 * 5) + FONTSET_ADDR,

        // Fx33 - LD B, Vx. Store BCD representation of Vx in memory locations I, I+1, and I+2.
        // The interpreter takes the decimal value of Vx,
        // and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
        (3, 3) => {
            memory.assign(cpu.i as usize, cpu.v[x] / 100);
            memory.assign((cpu.i + 1) as usize, (cpu.v[x] / 10) % 10);
            memory.assign((cpu.i + 2) as usize, (cpu.v[x] % 100) % 10);
        }

        // Fx55 - LD [I], Vx. Store registers V0 through Vx in memory starting at location I.
        // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
        (5, 5) => {
            for c in 0..=x as u16 {
                memory.assign((cpu.i + c) as usize, cpu.v[x]);
            }
        }

        // Fx65 - LD Vx, [I]. Read registers V0 through Vx from memory starting at location I.
        // The interpreter reads values from memory starting at location I into registers V0 through Vx.
        (6, 5) => {
            for c in 0..=x as u16 {
                cpu.v[c as usize] = memory.get((cpu.i + c) as usize)
            }
        }

        _ => panic!("unknown instruction"),
    }
}
