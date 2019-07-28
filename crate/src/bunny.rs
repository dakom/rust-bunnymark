use super::data::{Point, Area};
use rand::prelude::*;
use log::{info};

const START_GRAVITY:f64 = 0.75;

//mimicking https://github.com/pixijs/bunny-mark/blob/master/src/Bunny.js
#[derive(Debug)]
pub struct Bunny {
    pub gravity: f64,
    pub pos: Point,
    pub speed: Point,
}
impl Bunny {
    pub fn new(count:usize, stage_size: Area, img_size: Area) -> Self {
        let mut speed = Point::new_random();

        speed.x *= 10.0;
        speed.y = (speed.y * 10.0) - 5.0;

        //alternate between corners
        let pos_x = match (count % 2) {
            0 => 0.0f64,
            _ => (stage_size.width - img_size.width) as f64
        };

        let pos_y = (stage_size.height - img_size.height) as f64;

        let pos = Point { x: pos_x, y: pos_y };


        Self {
            gravity: START_GRAVITY,
            pos,
            speed,
        }
    }

    //movement is made to match https://github.com/pixijs/bunny-mark/blob/master/src/Bunny.js
    pub fn update(&mut self, stage_size: Area, img_size:Area) {
        self.pos.x += self.speed.x;
        self.pos.y -= self.speed.y;
    
        self.speed.y += self.gravity;

        let bounds_right = (stage_size.width - img_size.width) as f64;
        if self.pos.x > bounds_right {
            self.speed.x *= -1.0;
            self.pos.x = bounds_right;
        } else if self.pos.x < 0.0 {
            self.speed.x *= -1.0;
            self.pos.x = 0.0 
        }

        let bounds_top = (stage_size.height - img_size.height) as f64;
    
        if self.pos.y < 0.0 {
            self.speed.y *= -0.85;
            self.pos.y = 0.0;
            let rand_bool:bool = thread_rng().gen();
            if rand_bool  {
                let rand_float:f64 = thread_rng().gen();
                self.speed.y  -= rand_float * 6.0;
            }
        } else if self.pos.y > bounds_top {
            self.speed.y = 0.0;
            self.pos.y = bounds_top;
        }
    }
}
