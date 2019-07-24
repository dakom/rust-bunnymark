use super::bunny::{Bunny};

pub struct State {
    pub fps: u32,
    pub bunnies: Vec<Bunny>,
    pub width: u32,
    pub height: u32
}

impl State {
    pub fn new() -> Self {
        Self { fps: 0, bunnies: Vec::new(), width: 0, height: 0}
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}
