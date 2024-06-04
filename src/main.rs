use chipsy::{machine::Machine, rom, sound_generator::triangle::TriangleWave};
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
    let mut sdl_context = Context::new(
        "chipsy",
        pixels,
        BG_COLOR,
        FG_COLOR,
        WINDOW_WIDHT,
        WINDOW_HEIGHT,
        SCALE,
        |spec| {
            TriangleWave::new(220.0 / spec.freq as f32, 0.25)
        }
    )
    .unwrap();

    let (canvas, keyboard, audio) = sdl_context.extract_borrow();

    let mut machine = Machine::new(canvas, keyboard, audio);

    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);

    let rom_data = rom::read(fname);

    machine.init(rom_data);

    machine.run();
}
