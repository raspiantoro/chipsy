use chipsy::{machine::Machine, rom};

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
}
