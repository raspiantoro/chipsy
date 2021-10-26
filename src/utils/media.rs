pub mod canvas;
pub mod event;

use sdl2::{video::Window, Sdl};

use self::{canvas::MediaCanvas, event::MediaEvent};

pub struct RGB(pub u8, pub u8, pub u8);

pub fn init(
    bg_color: RGB,
    px_color: RGB,
    width: u32,
    height: u32,
    scale: u32,
) -> (MediaCanvas, MediaEvent) {
    let context = generate_media_context();
    let window = generate_window(&context, width * scale, height * scale);
    let canvas = MediaCanvas::new(window, bg_color, px_color, scale);
    let event_pump = MediaEvent::new(&context);

    (canvas, event_pump)
}

fn generate_media_context() -> Sdl {
    sdl2::init().expect("SDL initialization failed")
}

fn generate_window(context: &Sdl, width: u32, height: u32) -> Window {
    let video_subsystem = context.video().expect("couldn't get SDL video subsystem");

    video_subsystem
        .window("rchip", width, height)
        .position_centered()
        .opengl()
        .build()
        .expect("failed to create window")
}
