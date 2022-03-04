use super::state::State;
use super::hud::Hud;
use super::renderer::SceneRenderer;
use super::fps::FpsCounter;

use std::rc::Rc;
use std::cell::RefCell;
use gloo_events::EventListener;
use web_sys::{Window, EventTarget, Event};
use awsm_web::tick::Raf;

pub fn begin_loop(window:&Window, canvas:&EventTarget, mut renderer:SceneRenderer, state:Rc<RefCell<State>>, hud:Hud) -> Result<(), awsm_web::errors::Error> {


    //awsm provides a more advanced loop with a fixed timestep
    //but the goal here is to match https://github.com/pixijs/bunny-mark
    let tick = Raf::new({
        let state = Rc::clone(&state);
        let mut fps_counter = FpsCounter::new(window.performance().unwrap());

        move |_| {
            fps_counter.begin();
            let mut state = state.borrow_mut();

            if state.adding_bunnies {
                state.add_bunnies();
                renderer.update_uvs(&state).unwrap();
            }

            state.update_physics();
            renderer.update_vertices(&state).unwrap();

            renderer.render(&state).unwrap();

            fps_counter.end();
            state.fps = fps_counter.current.ceil() as u32;
            hud.update(&state);
        }
    });

    //end of the line! gotta keep these things in memory...
    EventListener::new(&canvas, "pointerdown", {
        let state = Rc::clone(&state);
        move |_e:&Event| {
            let mut state = state.borrow_mut();
            state.adding_bunnies = true;
        }
    }).forget();

    EventListener::new(&canvas, "pointerup", {
        let state = Rc::clone(&state);
        move |_e:&Event| {
            let mut state = state.borrow_mut();
            state.adding_bunnies = false;
        }
    }).forget();

    std::mem::forget(Box::new(tick));

    Ok(())
}

