use self::{
    canvas::Canvas,
    error::{SdlError, WindowError},
    keyboard::Keyboard,
};
use chipsy::machine::display::Pixels;
use std::rc::Rc;

pub mod canvas;
mod error;
pub mod keyboard;

#[derive(Default)]
pub struct RGB(pub u8, pub u8, pub u8);

pub struct Context {
    canvas: Rc<Canvas>,
    keyboard: Rc<keyboard::Keyboard>,
}

impl Context {
    pub fn new(
        window_title: &str,
        pixels: Pixels,
        canvas_bg: RGB,
        canvas_fg: RGB,
        canvas_width: u32,
        canvas_height: u32,
        scale: u32,
    ) -> Result<Self, SdlError> {
        let sdl_context = sdl2::init().map_err(SdlError::ContextError)?;

        let video_subsystem = sdl_context.video().map_err(WindowError::String)?;

        let window = video_subsystem
            .window(window_title, canvas_width * scale, canvas_height * scale)
            .position_centered()
            .opengl()
            .build()
            .map_err(WindowError::WindowBuildError)?;

        let canvas = Canvas::new(pixels, window, canvas_bg, canvas_fg, scale)?;

        let keyboard = Keyboard::new(&sdl_context)?;
        let keyboard = Rc::new(keyboard);

        Ok(Context {
            canvas: Rc::new(canvas),
            keyboard,
        })
    }

    pub fn get_canvas(&self) -> Rc<Canvas> {
        self.canvas.clone()
    }

    pub fn get_keyboard(&self) -> Rc<Keyboard> {
        self.keyboard.clone()
    }
}
