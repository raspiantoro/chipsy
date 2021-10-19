use super::CPU;

pub fn execute(cpu: &mut CPU, opcode: u16) {
    // notation -> hxyl
    let h: u8 = (opcode >> 12) as u8;
    let l: u8 = (opcode & 0x000F) as u8;
    let x: u8 = ((opcode >> 8) & 0x000F) as u8;
    let y: u8 = ((opcode >> 4) & 0x000F) as u8;

    match (h, x, y, l) {
        (8, _, _, _) => arithmetic(cpu, x, y, l),
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
