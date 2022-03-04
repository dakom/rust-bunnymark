use super::state::{State};
use super::hud::{Hud};
use super::config::{get_media_href};
use super::game_loop::{begin_loop};
use super::renderer::SceneRenderer;

use gloo_events::EventListener;
use std::rc::{Rc};
use std::cell::{RefCell};
use web_sys::{HtmlElement, HtmlCanvasElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use awsm_web::window::{get_window_size};
use awsm_web::loaders::{fetch::fetch_url, image};
use awsm_web::webgl::{
    BufferMask,
    ResizeStrategy,
    get_webgl_context_1, 
    WebGlContextOptions, 
    WebGl1Renderer
};

pub fn start() -> Result<js_sys::Promise, JsValue> {

    let window = web_sys::window().ok_or("should have a Window")?;
    let document = window.document().ok_or("should have a Document")?;
    let body = document.body().ok_or("should have a Body")?;

    let loading: HtmlElement = document.create_element("div")?.dyn_into()?;
    loading.set_class_name("loading");
    loading.set_text_content(Some("Loading..."));
    body.append_child(&loading)?;


    let future = async move {
        let bunny_img = image::load(get_media_href("bunny.png")).await?;
        let vertex = fetch_url(&get_media_href("vertex.glsl")).await?.text().await?;
        let fragment = fetch_url(&get_media_href("fragment.glsl")).await?.text().await?;


        let state = Rc::new(RefCell::new(State::new(&bunny_img)));

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

        let mut scene_renderer = SceneRenderer::new(Rc::clone(&renderer), &vertex, &fragment, &bunny_img)?;

        let on_resize = {
            let window = window.clone();
            let renderer = Rc::clone(&renderer);
            let state = Rc::clone(&state);
            move |_: &web_sys::Event| {
                let (width, height) = get_window_size(&window).unwrap();
                renderer.borrow_mut().resize(ResizeStrategy::All(width, height));
                state.borrow_mut().resize(width, height);
            }
        };

        on_resize(&web_sys::Event::new("").unwrap());

        EventListener::new(&window, "resize", on_resize).forget();

        {
            let renderer = renderer.borrow_mut();

            renderer.gl.clear_color(0.3, 0.3, 0.3, 1.0);
            renderer.clear(&[
                BufferMask::ColorBufferBit,
                BufferMask::DepthBufferBit,
            ]);
        }


        state.borrow_mut().add_bunnies();
        scene_renderer.update_uvs(&state.borrow()).unwrap();

        begin_loop(&window, &renderer.borrow().canvas, scene_renderer, state, hud)?;

        Ok(JsValue::null())
    };

    Ok(future_to_promise(future))
}



