use super::state::{State};
use web_sys::{HtmlElement, Document};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::{JsValue};

pub struct Hud {
    container:HtmlElement,
    num_bunnies:HtmlElement,
    fps:HtmlElement,
}

impl Hud {
    pub fn new(document:&Document, body:&HtmlElement) -> Result<Self, JsValue> {
        let container: HtmlElement = document.create_element("div")?.dyn_into()?;
        container.set_class_name("info");
        body.append_child(&container)?;


        let num_bunnies: HtmlElement = document.create_element("div")?.dyn_into()?;
        num_bunnies.set_class_name("info-num_bunnies");
        num_bunnies.set_text_content(Some(""));
        container.append_child(&num_bunnies)?;

        let fps: HtmlElement = document.create_element("div")?.dyn_into()?;
        fps.set_class_name("info-fps");
        fps.set_text_content(Some(""));
        container.append_child(&fps)?;

        Ok(Self{ container, num_bunnies, fps })
    }

    pub fn update(&self, state:&State) {
        let s = format!("bunnies: {}", state.bunnies.len());
        self.num_bunnies.set_text_content(Some(&s));
        let s = format!("fps: {}", state.fps);
        self.fps.set_text_content(Some(&s));
    }
}
