pub mod media;

use std::{thread::sleep, time::Duration};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Texture, TextureCreator},
};

static UI_SCALE: u32 = 30;

pub struct Config {
    width: u32,
    height: u32,
}

pub struct UI {
    config: Config,
}

impl UI {
    pub fn new(width: u32, height: u32) -> Self {
        let config = Config { width, height };
        UI { config }
    }

    pub fn show(&self) {
        let sdl_context = sdl2::init().expect("SDL initialization failed");
        let video_subsystem = sdl_context
            .video()
            .expect("couldn't get SDL video subsystem");

        let window = video_subsystem
            .window("rchip", self.config.width, self.config.height)
            .position_centered()
            .opengl()
            .build()
            .expect("failed to create window");

        let mut canvas = window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .expect("failed to convert window into canvas");

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context
            .event_pump()
            .expect("failed to get SDL event pump");

        let texture_creator: TextureCreator<_> = canvas.texture_creator();
        let texture_size: u32 = 32;
        let mut square_texture: Texture = texture_creator
            .create_texture_target(None, texture_size, texture_size)
            .expect("failed to create a texture");

        let _ = canvas.with_texture_canvas(&mut square_texture, |texture| {
            texture.set_draw_color(Color::RGB(0, 255, 0));
            texture.clear();
        });

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    _ => {}
                }
            }

            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.clear();

            canvas.set_draw_color(Color::RGB(0, 0, 0));

            let _ = canvas.fill_rect(Rect::new(
                10 * UI_SCALE as i32,
                10 * UI_SCALE as i32,
                UI_SCALE,
                UI_SCALE,
            ));
            let _ = canvas.fill_rect(Rect::new(
                11 * UI_SCALE as i32,
                10 * UI_SCALE as i32,
                UI_SCALE,
                UI_SCALE,
            ));
            let _ = canvas.fill_rect(Rect::new(
                12 * UI_SCALE as i32,
                10 * UI_SCALE as i32,
                UI_SCALE,
                UI_SCALE,
            ));
            let _ = canvas.fill_rect(Rect::new(
                13 * UI_SCALE as i32,
                10 * UI_SCALE as i32,
                UI_SCALE,
                UI_SCALE,
            ));
            let _ = canvas.fill_rect(Rect::new(
                14 * UI_SCALE as i32,
                10 * UI_SCALE as i32,
                UI_SCALE,
                UI_SCALE,
            ));
            let _ = canvas.fill_rect(Rect::new(
                12 * UI_SCALE as i32,
                11 * UI_SCALE as i32,
                UI_SCALE,
                UI_SCALE,
            ));
            let _ = canvas.fill_rect(Rect::new(
                12 * UI_SCALE as i32,
                12 * UI_SCALE as i32,
                UI_SCALE,
                UI_SCALE,
            ));
            let _ = canvas.fill_rect(Rect::new(
                12 * UI_SCALE as i32,
                13 * UI_SCALE as i32,
                UI_SCALE,
                UI_SCALE,
            ));
            let _ = canvas.fill_rect(Rect::new(
                12 * UI_SCALE as i32,
                14 * UI_SCALE as i32,
                UI_SCALE,
                UI_SCALE,
            ));
            let _ = canvas.fill_rect(Rect::new(
                12 * UI_SCALE as i32,
                15 * UI_SCALE as i32,
                UI_SCALE,
                UI_SCALE,
            ));

            // canvas
            //     .copy(
            //         &square_texture,
            //         None,
            //         Rect::new(0, 0, texture_size, texture_size),
            //     )
            //     .expect("couldn't copy texture into window");

            canvas.present();

            sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
