pub struct Timer {
    start_time: f64,
}

impl Timer {
    pub fn new(start_time: f64) -> Timer {
        Timer {
            start_time,
        }
    }

    pub fn reset(&mut self, time: f64) {
        self.start_time = time;
    }

    /// in seconds
    pub fn elapsed(&self, time: f64) -> f64 {
        time - self.start_time
    }
}
