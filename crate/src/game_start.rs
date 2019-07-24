use super::state::{State};
use super::hud::{Hud};
use super::config::{get_media_href};
use super::game_loop::{begin_loop};

use log::{info};
use gloo_events::EventListener;
use std::rc::{Rc};
use std::cell::{RefCell};
use web_sys::{Window, Document, HtmlElement, HtmlCanvasElement};
use wasm_bindgen::prelude::{JsValue};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::futures_0_3::future_to_promise;
use awsm::loaders::fetch;
use awsm::webgl::{
    get_webgl_context_1, 
    WebGlContextOptions, 
    ClearBufferMask,
    WebGlCommon,
    WebGl1Renderer
};

pub fn start() -> Result<(), JsValue> {

    let window = web_sys::window().ok_or("should have a Window")?;
    let document = window.document().ok_or("should have a Document")?;
    let body = document.body().ok_or("should have a Body")?;

    let loading: HtmlElement = document.create_element("div")?.dyn_into()?;
    loading.set_class_name("loading");
    loading.set_text_content(Some("Loading..."));
    body.append_child(&loading)?;


    let future = async move {
        let bunny_img = fetch::image(&get_media_href("bunny.png")).await?;
        let vertex = fetch::text(&get_media_href("vertex.glsl")).await?;
        let fragment = fetch::text(&get_media_href("fragment.glsl")).await?;

        let state = Rc::new(RefCell::new(State::new()));

        body.remove_child(&loading)?;
        let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
        body.append_child(&canvas)?;

        let hud = Hud::new(&document, &body)?;
        hud.update(&state.borrow());

        //not using any webgl2 features so might as well stick with v1
        let gl = get_webgl_context_1(&canvas, Some(&WebGlContextOptions {
            alpha: false,
            ..WebGlContextOptions::default()
        }))?;

        let renderer = WebGl1Renderer::new(gl).map(|r| Rc::new(RefCell::new(r)))?;

        let mut on_resize = {
            let window = window.clone();
            let renderer = Rc::clone(&renderer);
            let state = Rc::clone(&state);
            move |_: &web_sys::Event| {
                let (width, height) = awsm::window::get_size(&window).unwrap();
                renderer.borrow_mut().resize(width, height);
                state.borrow_mut().resize(width, height);
            }
        };

        on_resize(&web_sys::Event::new("").unwrap());

        EventListener::new(&window, "resize", on_resize).forget();

        {
            let renderer = renderer.borrow_mut();

            renderer.gl.clear_color(0.3, 0.3, 0.3, 1.0);
            renderer.clear(&[
                ClearBufferMask::ColorBufferBit,
                ClearBufferMask::DepthBufferBit,
            ]);
        }

        begin_loop(renderer, state, hud)?;

        Ok(JsValue::null())
    };

    future_to_promise(future);

    Ok(())
}



