use std::{thread::sleep, time::Duration};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

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
            .build()
            .expect("failed to convert window into canvas");

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context
            .event_pump()
            .expect("failed to get SDL event pump");

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

            sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
