use rchip::{machine::Machine, rom};

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    rchip.exe ROM_FILE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    rchip ROM_FILE
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);

    let rom_data = rom::read(fname);
    // let rom_data = rom::dummy_rom_data();

    let mut machine = Machine::new();
    machine.init(rom_data);

    machine.run();

    // let a: u8 = 156;
    // // let b: u8 = 4;

    // // let (val, borrow) = a.overflowing_sub(b);
    // // a = val;

    // // let val: u8 = a << 1;

    // println!(
    //     "hundreds: {}, tens: {}, ones: {}",
    //     a / 100,
    //     (a / 10) % 10,
    //     (a % 100) % 10
    // );

    // for i in 0..=10 as u8 {
    //     println!("Test: {}", 3 + i)
    // }
}
