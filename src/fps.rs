use web_sys::{Performance};

pub struct FpsCounter {
    performance:Performance,
    begin_time: f64,
    prev_time: f64,
    frames: usize,
    pub current: f64
}

impl FpsCounter {
    pub fn new(performance:Performance) -> Self {
        let begin_time = performance.now();
        Self {
            performance,
            begin_time,
            prev_time: begin_time,
            frames: 0,
            current: 0.0
        }
    }

    pub fn begin(&mut self) {
        self.begin_time = self.performance.now();
    }

    pub fn end(&mut self) {
        self.frames += 1;
        let time = self.performance.now();

        if time >= (self.prev_time + 1000.0) {
            self.current = ((self.frames * 1000) as f64) / (time - self.prev_time);
            self.prev_time = time;
            self.frames = 0; 
        }
    }

}


