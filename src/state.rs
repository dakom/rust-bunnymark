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
            stage_size: Area { width: 0f32, height: 0f32},
            img_size: Area { width: img_width as f32, height: img_height as f32},
            instance_positions: Vec::new(),
            add_amount: hash_amount.unwrap_or(N_BUNNIES_PER_TICK)
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.stage_size = Area { width: width as f32, height: height as f32 };
    }

    pub fn add_bunnies(&mut self) {
        let stage_size = self.stage_size;
        let img_size = self.img_size;


        for i in 0..self.add_amount {
            let (bunny, (x,y)) = Bunny::new(i, stage_size, img_size);
            self.bunnies.push(bunny);
            self.instance_positions.push(x);
            self.instance_positions.push(y);
        }

    }

    pub fn update(&mut self) {
        let mut positions = self.instance_positions.as_mut_slice();
        for (instance_idx, bunny) in self.bunnies.iter_mut().enumerate() {
            //update bunny positions
            let (mut pos_x, mut pos_y) = get_point_unchecked(&mut positions, instance_idx * 2);
            bunny.update(self.stage_size, self.img_size, &mut pos_x, &mut pos_y);

        }
    }
}

fn get_point_unchecked(s: &mut [f32], offset: usize) -> (&mut f32, &mut f32) {
    let ptr = s.as_mut_ptr();

    unsafe {
        (&mut *ptr.add(offset), &mut *ptr.add(offset + 1))
    }
}
