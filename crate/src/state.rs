use super::bunny::{Bunny};
use super::data::{Area};
use super::config::{N_BUNNIES_PER_TICK};

use awsm::webgl::{
    get_texture_size,
    WebGlTextureSource
};

pub struct State {
    pub fps: u32,
    pub bunnies: Vec<Bunny>,
    pub adding_bunnies: bool,
    pub stage_size: Area,
    pub img_size: Area,
    pub instance_positions: Vec<f32>
}

impl State {
    pub fn new(img:&web_sys::HtmlImageElement) -> Self {

        let (img_width, img_height, _) = get_texture_size(&WebGlTextureSource::ImageElement(&img));

        Self { 
            fps: 0, 
            bunnies: Vec::new(), 
            adding_bunnies: false,
            stage_size: Area { width: 0, height: 0},
            img_size: Area { width: img_width, height: img_height},
            instance_positions: Vec::new(),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.stage_size = Area { width, height };
    }

    pub fn add_bunnies(&mut self) {
        let mut count = self.bunnies.len();
        let len = count + N_BUNNIES_PER_TICK;
        let stage_size = self.stage_size;
        let img_size = self.img_size;

        self.bunnies.resize_with(len, | | {
            let bunny = Bunny::new(count, stage_size, img_size);
            count += 1;
            bunny
        }); 

        self.instance_positions.resize(len * 2, 0.0);
    }

    pub fn update(&mut self) {
        for (instance_idx, mut bunny) in self.bunnies.iter_mut().enumerate() {
            //update bunny positions
            bunny.update(self.stage_size, self.img_size);

            let instance_idx = instance_idx * 2;
            //Set the instance data from bunny positions
            self.instance_positions[instance_idx] = bunny.pos.x as f32;
            self.instance_positions[instance_idx+1] = bunny.pos.y as f32;
        }
    }
}
