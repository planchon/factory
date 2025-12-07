pub struct Entity {
    is_alive: bool,
}

impl Entity {
    pub fn new() -> Self {
        Self { is_alive: true }
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn kill(&mut self) {
        self.is_alive = false;
    }

    pub fn reset(&mut self) {
        self.is_alive = true;
    }
}
