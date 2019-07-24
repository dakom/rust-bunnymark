use super::state::{State};
use super::hud::{Hud};

use web_sys::{Window, Document, HtmlElement};
use wasm_bindgen::prelude::{JsValue};
use wasm_bindgen::JsCast;


pub fn init(window:Window, document:Document, body:HtmlElement) -> Result<(), JsValue> {
    let mut state = State::new();

    let hud = Hud::new(&document, &body)?;
    hud.update(&state);

    Ok(())
}




