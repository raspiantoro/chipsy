use chipsy::{machine::Machine, rom};
use sdl::{Context, RGB};

mod sdl;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    chipsy.exe ROM_FILE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    chipsy ROM_FILE
";

const SCALE: u32 = 15;
const BG_COLOR: RGB = RGB(0, 0, 0);
const FG_COLOR: RGB = RGB(233, 68, 43);
const WINDOW_WIDHT: u32 = 64;
const WINDOW_HEIGHT: u32 = 32;

fn main() {
    let pixels = [[0; WINDOW_WIDHT as usize]; WINDOW_HEIGHT as usize];
    let sdl_context = Context::new(
        "chipsy",
        pixels,
        BG_COLOR,
        FG_COLOR,
        WINDOW_WIDHT,
        WINDOW_HEIGHT,
        SCALE,
    )
    .unwrap();

    let mut machine = Machine::new(sdl_context.get_canvas(), sdl_context.get_keyboard());

    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);

    let rom_data = rom::read(fname);
    // let rom_data = rom::dummy_rom_data();

    machine.init(rom_data);

    machine.run();
}
