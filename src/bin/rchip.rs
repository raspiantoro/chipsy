use rchip::machine::Machine;

fn main() {
    let arr: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    let mut machine = Machine::new();
    machine.init();

    machine.v_reg_set(1, 40);
    machine.v_reg_set(2, 20);
    machine.i_reg_set(0x050);

    machine.mem_assign(0x200, 0xD1);
    machine.mem_assign(0x201, 0x25);

    // // infinite loop
    machine.mem_assign(0x202, 0x12);
    machine.mem_assign(0x203, 0x00);

    // stop process
    // machine.mem_assign(0x202, 0x00);
    // machine.mem_assign(0x203, 0x00);

    machine.run();
}
