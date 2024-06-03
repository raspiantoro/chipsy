use std::cell::RefCell;

use super::{error::SdlError, RGB};
use chipsy::machine::display::{Display, Pixels};
use sdl2::{pixels::Color, rect::Rect, render, video::Window};

// #[derive(Clone)]
pub struct Canvas {
    canvas: RefCell<render::Canvas<Window>>,
    bg_color: RGB,
    fg_color: RGB,
    scale: u32,
    pixels: Pixels,
}

impl Display for Canvas {
    fn get_pixels(&mut self) -> &mut Pixels {
        &mut self.pixels
    }

    fn render(&self) {
        let pixels = &self.pixels;
        let mut canvas = self.canvas.borrow_mut();

        canvas.set_draw_color(Color::RGB(
            self.bg_color.0,
            self.bg_color.1,
            self.bg_color.2,
        ));

        canvas.clear();

        canvas.set_draw_color(Color::RGB(
            self.fg_color.0,
            self.fg_color.1,
            self.fg_color.2,
        ));

        for (y, row) in pixels.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col == 1 {
                    let _ = canvas.fill_rect(Rect::new(
                        (x as u32 * self.scale) as i32,
                        (y as u32 * self.scale) as i32,
                        self.scale,
                        self.scale,
                    ));
                }
            }
        }

        canvas.present();
    }
}

impl Canvas {
    pub fn new(
        pixels: Pixels,
        window: Window,
        bg_color: RGB,
        fg_color: RGB,
        scale: u32,
    ) -> Result<Self, SdlError> {
        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(SdlError::CanvasError)?;

        canvas.set_draw_color(Color::RGB(bg_color.0, bg_color.1, bg_color.2));
        canvas.clear();

        Ok(Self {
            canvas: RefCell::new(canvas),
            bg_color,
            fg_color,
            scale,
            pixels: pixels,
        })
    }
}
