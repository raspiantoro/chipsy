use self::{
    canvas::Canvas,
    error::{SdlError, WindowError},
    keyboard::Keyboard,
};
use audio::{AudioPlayback, SquareWave};
use chipsy::machine::display::Pixels;
use sdl2::audio::AudioSpecDesired;

pub mod canvas;
mod error;
pub mod keyboard;
pub mod audio;

#[derive(Default)]
pub struct RGB(pub u8, pub u8, pub u8);

pub struct Context {
    canvas: Canvas,
    keyboard: keyboard::Keyboard,
    audio: AudioPlayback<SquareWave>
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

        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(2),  // mono
            samples: None       // default sample size
        };

        let canvas = Canvas::new(pixels, window, canvas_bg, canvas_fg, scale)?;

        let keyboard = Keyboard::new(&sdl_context)?;

        let audio = AudioPlayback::new(audio_subsystem, &desired_spec, |spec| {
            SquareWave::new(440.0 / spec.freq as f32, 0.0, 0.25)
        });

        Ok(Context {
            canvas: canvas,
            keyboard,
            audio
        })
    }

    pub fn extract_borrow(&mut self) -> (&mut Canvas, &Keyboard, &AudioPlayback<SquareWave>){
        (&mut self.canvas, &self.keyboard, &self.audio)
    }
}
