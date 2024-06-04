use chipsy::machine::audio::Audio;
use sdl2::{audio::{AudioCallback, AudioDevice, AudioSpec, AudioSpecDesired}, AudioSubsystem};

pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl SquareWave {
    pub fn new(phase_inc: f32, phase: f32, volume: f32) -> SquareWave{
        SquareWave {
            phase_inc: phase_inc,
            phase: phase,
            volume: volume
        }
    }
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct AudioPlayback<T>
where
    T: AudioCallback
{
    device: AudioDevice<T>
}

impl<T> AudioPlayback<T> 
where
    T: AudioCallback
{
    pub fn new<F: FnOnce(AudioSpec) -> T>(audio_subsystem: AudioSubsystem, desired_spec: &AudioSpecDesired, callback: F) -> AudioPlayback<T>
    {
        let device = audio_subsystem.open_playback(None, desired_spec, callback).unwrap();

        AudioPlayback {
            device
        }
    }


    
}

impl<T> Audio for AudioPlayback<T> 
where
    T: AudioCallback
{
    fn resume(&self){
        self.device.resume();
        // thread::sleep(duration);
        // self.pause();
    }

    fn pause(&self){
        self.device.pause()
    }
}

unsafe impl<T> Send for AudioPlayback<T>
where
    T: AudioCallback
{}