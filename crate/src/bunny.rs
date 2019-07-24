use super::data::{Point};

pub struct Bunny {
    pos: Point,
    vel: Point
}
impl Bunny {
    pub fn new() -> Self {
        Self {
            pos: Point::new(),
            vel: Point::new()
        }
    }
}
