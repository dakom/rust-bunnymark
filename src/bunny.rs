use super::data::{Point, Area};

const START_GRAVITY:f32= 0.75;

//mimicking https://github.com/pixijs/bunny-mark/blob/master/src/Bunny.js
#[derive(Debug)]
pub struct Bunny {
    pub gravity: f32,
    pub speed: Point,
}
impl Bunny {
    pub fn new(count:usize, stage_size: Area, img_size: Area) -> (Self, (f32, f32)) {
        let mut speed = Point::new_random();

        speed.x *= 10.0;
        speed.y = (speed.y * 10.0) - 5.0;

        //alternate between corners
        let pos_x = match count % 2 {
            0 => 0.0f32,
            _ => (stage_size.width - img_size.width) as f32
        };

        let pos_y = (stage_size.height - img_size.height) as f32;


        let bunny = Self {
            gravity: START_GRAVITY,
            speed,
        };

        (bunny, (pos_x, pos_y))
    }

    //movement is made to match https://github.com/pixijs/bunny-mark/blob/master/src/Bunny.js
    pub fn update(&mut self, stage_size: Area, img_size:Area, pos_x: &mut f32, pos_y: &mut f32) {

        *pos_x += self.speed.x;
        *pos_y -= self.speed.y;
    
        self.speed.y += self.gravity;

        let bounds_right = (stage_size.width - img_size.width) as f32;
        if *pos_x > bounds_right {
            self.speed.x *= -1.0;
            *pos_x = bounds_right;
        } else if *pos_x < 0.0 {
            self.speed.x *= -1.0;
            *pos_x = 0.0 
        }

        let bounds_top = (stage_size.height - img_size.height) as f32;
    
        if *pos_y < 0.0 {
            self.speed.y *= -0.85;
            *pos_y = 0.0;
            if js_sys::Math::random() > 0.5 {
                self.speed.y -= (js_sys::Math::random() as f32) * 6.0;
            }
        } else if *pos_y > bounds_top {
            self.speed.y = 0.0;
            *pos_y = bounds_top;
        }
    }
}
