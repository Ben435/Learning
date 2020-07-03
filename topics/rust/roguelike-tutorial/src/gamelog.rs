pub struct GameLog {
    entries: Vec<String>
}

impl GameLog {

    pub fn new(init_messages: &[String]) -> GameLog {
        GameLog {
            entries: Vec::from(init_messages),
        }
    }

    pub fn info(&mut self, message: String) {
        self.entries.push(message);
    }

    pub fn entries(&self) -> &Vec<String> {
        &self.entries
    }
}
