use sdl2::audio::AudioCallback;

pub struct TriangleWave {
    phase_inc: f32,
    volume: f32
}

impl TriangleWave {
    pub fn new(phase_inc: f32, volume: f32) -> TriangleWave{
        TriangleWave {
            phase_inc: phase_inc,
            volume: volume
        }
    }
}

impl AudioCallback for TriangleWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a triangle wave
        let mut increased = true;
        for x in out.iter_mut() {
            if self.volume >= 0.5 {
                increased = false;
            } else if self.volume <= -0.5 {
                increased = true;
            }

            if increased {
                self.volume += self.phase_inc + (1.0/10000.0)
            } else {
                self.volume -= self.phase_inc + (1.0/10000.0)
            };

            *x = self.volume;
        }
    }
}