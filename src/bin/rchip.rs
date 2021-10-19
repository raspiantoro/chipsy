use rchip::machine::Machine;

fn main(){
    let mut machine = Machine::New();
    machine.init();

    // dummy program
    machine.v_reg_set(1, 100);
    machine.v_reg_set(8, 200);

    // opcode 8xy4
    machine.mem_assign(0x200, 0x81);
    machine.mem_assign(0x201, 0x84);
    //

    machine.run();


    println!("{}", machine.v_reg_get(1));
    println!("{}", machine.v_reg_get(0xF));
}
