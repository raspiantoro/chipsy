use chipsy::{machine::Machine, rom::RomBytes};

#[test]
fn test_run() {
    let rom_data = RomBytes::new();
    let mut machine = Machine::new();
    machine.init(rom_data);

    // try to added register V5 and V2
    // the result should be stored in register V5
    // final result: V5 = 0xF = 15 and VF = 0
    machine.v_reg_set(5, 10);
    machine.v_reg_set(2, 5);

    // opcode 8xy4 => x and y is V registers index
    machine.mem_assign(0x200, 0x85);
    machine.mem_assign(0x201, 0x24);

    // stop machine execution
    machine.mem_assign(0x202, 0x0);
    machine.mem_assign(0x203, 0x0);

    machine.run();

    // println!("addition result: {}", machine.v_reg_get(5));
    // println!("VF: {}", machine.v_reg_get(0xF));

    assert_eq!(machine.v_reg_get(5), 0xF);
    assert_eq!(machine.v_reg_get(0xF), 0);

    // try to added register V1 and V8
    // the result should be stored in register V1
    // this addition will be overflow,
    // V1 should kept the lowest 8 bit only of the result
    // and VF = carry should be 1.
    // 100 + 200 = 300 = 0x012C (not enough space for u8)
    // final result: V1 = 0x2C = 44 and VF = 1
    machine.v_reg_set(1, 100);
    machine.v_reg_set(8, 200);

    // opcode 8xy4 => x and y is V registers index
    machine.mem_assign(0x204, 0x81);
    machine.mem_assign(0x205, 0x84);

    // stop machine execution
    machine.mem_assign(0x206, 0x0);
    machine.mem_assign(0x207, 0x0);

    machine.run();

    // println!("overflow addition result: {}", machine.v_reg_get(1));
    // println!("VF: {}", machine.v_reg_get(0xF));

    assert_eq!(machine.v_reg_get(1), 0x2C);
    assert_eq!(machine.v_reg_get(0xF), 0x1)
}
