use super::bunny::{Bunny};

pub struct State {
    pub fps: u32,
    pub bunnies: Vec<Bunny>
}

impl State {
    pub fn new() -> Self {
        Self { fps: 0, bunnies: Vec::new() }
    }
}
