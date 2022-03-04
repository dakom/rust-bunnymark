use super::bunny::Bunny;
use super::data::Area;
use super::config::N_BUNNIES_PER_TICK;
use awsm_web::prelude::UnwrapExt;

use awsm_web::webgl::{
    get_texture_size,
    WebGlTextureSource
};

pub struct State {
    pub fps: u32,
    pub bunnies: Vec<Bunny>,
    pub adding_bunnies: bool,
    pub stage_size: Area,
    pub img_size: Area,
    pub instance_positions: Vec<f32>,
    pub add_amount: usize,
}

impl State {
    pub fn new(img:&web_sys::HtmlImageElement) -> Self {

        let (img_width, img_height, _) = get_texture_size(&WebGlTextureSource::ImageElement(&img));

        let hash_amount = web_sys::window()
            .unwrap_ext()
            .location()
            .hash()
            .ok()
            .as_ref()
            .and_then(|amount| {
                amount
                    .strip_prefix("#")
                    .unwrap_or(amount)
                    .parse::<usize>()
                    .ok()
            });

        Self { 
            fps: 0, 
            bunnies: Vec::new(), 
            adding_bunnies: false,
            stage_size: Area { width: 0, height: 0},
            img_size: Area { width: img_width, height: img_height},
            instance_positions: Vec::new(),
            add_amount: hash_amount.unwrap_or(N_BUNNIES_PER_TICK)
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.stage_size = Area { width, height };
    }

    pub fn add_bunnies(&mut self) {
        let mut count = self.bunnies.len();
        let len = count + self.add_amount;
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
        let positions = self.instance_positions.as_mut_slice();
        for (mut instance_idx, bunny) in self.bunnies.iter_mut().enumerate() {
            //update bunny positions
            let (x,y) = bunny.update(self.stage_size, self.img_size);

            //Set the instance data from bunny positions
            instance_idx *= 2;
            positions[instance_idx] = x;
            positions[instance_idx+1] = y;
        }
    }
}
