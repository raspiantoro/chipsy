use sdl2::{event::EventPollIterator, EventPump, Sdl};

pub struct MediaEvent {
    pump: EventPump,
}

impl MediaEvent {
    pub fn new(context: &Sdl) -> Self {
        let pump = context.event_pump().expect("failed to get SDL event pump");

        MediaEvent { pump }
    }

    pub fn to_iter(&mut self) -> EventPollIterator {
        self.pump.poll_iter()
    }
}
