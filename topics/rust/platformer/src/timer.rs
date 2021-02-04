use std::time::{Duration,Instant};

pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}
