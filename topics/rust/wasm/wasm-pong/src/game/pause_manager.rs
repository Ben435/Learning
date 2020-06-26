use js_sys::Date;

const DEFAULT_DEBOUNCE_TIME_MILLIS: f64 = 200.0;

pub struct PauseManager {
    is_paused: bool,
    last_change_utc_millis: f64,
    debounce_time_millis: f64, // millis
}

impl PauseManager {
    pub fn new(init_paused: bool) -> PauseManager {
        PauseManager{
            is_paused: init_paused,
            last_change_utc_millis: Date::now(),
            debounce_time_millis: DEFAULT_DEBOUNCE_TIME_MILLIS,
        }
    }

    pub fn get_paused(&self) -> bool {
        self.is_paused
    }
    
    pub fn set_paused(&mut self, new_paused: bool) {
        self.is_paused = new_paused;
    }

    pub fn key_toggle_pause(&mut self) {
        let now = Date::now();

        if (now - self.last_change_utc_millis) > self.debounce_time_millis {
            self.set_paused(!self.is_paused);
        }
        self.last_change_utc_millis = now;
    }
}
