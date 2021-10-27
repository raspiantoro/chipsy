use super::RGB;
use sdl2::{pixels::Color, rect::Rect, render, video::Window};

pub struct MediaCanvas {
    canvas: render::Canvas<Window>,
    bg_color: RGB,
    px_color: RGB,
    scale: u32,
}

impl MediaCanvas {
    pub fn new(window: Window, bg_color: RGB, px_color: RGB, scale: u32) -> Self {
        let mut canvas = window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .expect("failed to convert window into canvas");

        canvas.set_draw_color(Color::RGB(bg_color.0, bg_color.1, bg_color.2));
        canvas.clear();

        Self {
            canvas,
            bg_color,
            px_color,
            scale,
        }
    }

    pub fn prepare(&mut self) {
        self.canvas.set_draw_color(Color::RGB(
            self.bg_color.0,
            self.bg_color.1,
            self.bg_color.2,
        ));

        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(
            self.px_color.0,
            self.px_color.1,
            self.px_color.2,
        ));
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: &u8) {
        // if *value == 1 {
        //     self.canvas.set_draw_color(Color::RGB(
        //         self.px_color.0,
        //         self.px_color.1,
        //         self.px_color.2,
        //     ));
        // } else {
        //     self.canvas.set_draw_color(Color::RGB(
        //         self.bg_color.0,
        //         self.bg_color.1,
        //         self.bg_color.2,
        //     ));
        // }

        if *value == 1 {
            let _ = self.canvas.fill_rect(Rect::new(
                (x * self.scale) as i32,
                (y * self.scale) as i32,
                self.scale,
                self.scale,
            ));
        }
    }

    pub fn draw(&mut self) {
        self.canvas.present()
    }
}
