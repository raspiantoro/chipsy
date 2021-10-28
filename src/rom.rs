use std::{
    fs::OpenOptions,
    io::{BufReader, Read},
};

pub type RomBytes = Vec<u8>;

pub fn read(filename: &str) -> RomBytes {
    let path = std::path::Path::new(filename);
    let f = OpenOptions::new().read(true).open(path).unwrap();
    let mut reader = BufReader::new(f);

    let mut rom_data = RomBytes::new();

    reader.read_to_end(&mut rom_data).unwrap();

    rom_data
}

// for debuging
pub fn print_opcodes(rom_data: RomBytes) {
    let mut count = 1;
    for data in rom_data.iter() {
        print!("{:02x}", data);
        if count == 2 {
            println!();
            count = 1;
            continue;
        }
        count += 1;
    }
}

// for testing
pub fn dummy_rom_data() -> RomBytes {
    let mut rom_data = RomBytes::new();

    rom_data.push(0x00);
    rom_data.push(0xe0); // CLS
    rom_data.push(0x61);
    rom_data.push(0x14); // LD v1, 0x14
    rom_data.push(0x62);
    rom_data.push(0x0a); // LD v2, 0x0a
    rom_data.push(0xa0);
    rom_data.push(0x50); // LD i, 0x050
    rom_data.push(0x00);
    rom_data.push(0xe0); // CLS
    rom_data.push(0xD1);
    rom_data.push(0x25); // DRW v1, v2, 0x5
    rom_data.push(0x12);
    rom_data.push(0x08); // JP 0x208

    rom_data
}
