use std::{sync::Mutex, time::Duration};

use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
pub enum TimerState {
    Running(u128),
    Idle
}

#[derive(Debug, Clone, Copy)]
pub struct Timer {
    count_down: u128,
    state: TimerState
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            count_down: 0,
            state: TimerState::Idle
        }
    }

    pub fn start(&mut self, duration: Duration) {
        self.count_down = duration.as_millis();
        self.state = TimerState::Running(self.count_down);
    }

    pub fn stop(&mut self) {
        self.count_down = 0;
        self.state = TimerState::Idle;
    }

    pub fn tick(&mut self){
        if self.count_down > 0 {
            self.count_down -= 1;
        }
        
        self.state = TimerState::Running(self.count_down);
    }

    pub fn state(self) -> TimerState{
        self.state
    }

}

lazy_static!{
    pub static ref TIMER: Mutex<Timer> = Mutex::new(Timer::new());
}