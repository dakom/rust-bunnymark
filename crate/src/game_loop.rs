use super::state::{State};
use super::hud::{Hud};
use super::bunny::{Bunny};
use super::data::{Area};
use super::renderer::SceneRenderer;
use super::fps::{FpsCounter};

use std::rc::{Rc};
use std::cell::{RefCell};
use gloo_events::{EventListener};
use log::{info};
use web_sys::{Window, Document, EventTarget, Event};

use awsm::tick::{RafLoop};
use awsm::webgl::{
    get_webgl_context_1, 
    WebGlContextOptions, 
    ClearBufferMask,
    WebGlCommon,
    WebGl1Renderer
};

pub fn begin_loop(window:&Window, document:&Document, canvas:&EventTarget, mut renderer:SceneRenderer, state:Rc<RefCell<State>>, hud:Hud) -> Result<(), awsm::errors::Error> {

    //input callbacks
    let on_mouse_down = {
        let state = Rc::clone(&state);
        move || {
            info!("mouse down");
            let mut state = state.borrow_mut();
            state.adding_bunnies = true;
        }
    };

    let on_mouse_release = {
        let state = Rc::clone(&state);
        move || {
            info!("mouse release");
            let mut state = state.borrow_mut();
            state.adding_bunnies = false;
        }
    };

    //awsm provides a more advanced loop with a fixed timestep
    //but the goal here is to match https://github.com/pixijs/bunny-mark
    let tick = RafLoop::start({
        let state = Rc::clone(&state);
        let mut fps_counter = FpsCounter::new(window.performance().unwrap());

        move |_| {
            fps_counter.begin();
            let mut state = state.borrow_mut();

            if state.adding_bunnies {
                state.add_bunnies();
            }

            state.update();

            renderer.render(&state).unwrap();

            fps_counter.end();
            state.fps = fps_counter.current.ceil() as u32;
            hud.update(&state);
        }
    })?;


    //end of the line! gotta keep these things in memory...
    //listening to touch and mouse: https://stackoverflow.com/a/31210694/784519
    EventListener::new(&canvas, "touchstart", {
        let on_mouse_down = on_mouse_down.clone();
        move |e:&Event| {
            e.prevent_default();
            on_mouse_down();
        }
    }).forget();

    EventListener::new(&canvas, "mousedown", {
        let on_mouse_down = on_mouse_down.clone();
        move |e:&Event| {
            e.prevent_default();
            on_mouse_down();
        }
    }).forget();

    EventListener::new(&canvas, "touchend", {
        let on_mouse_release = on_mouse_release.clone();
        move |e:&Event| {
            e.prevent_default();
            on_mouse_release();
        }
    }).forget();

    EventListener::new(&canvas, "mouseup", {
        let on_mouse_release = on_mouse_release.clone();
        move |e:&Event| {
            e.prevent_default();
            on_mouse_release();
        }
    }).forget();

    std::mem::forget(Box::new(tick));

    Ok(())
}
