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
            }

            state.update();

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

///// Until Raf is availble in gloo...
//struct Raf {
    //state: Rc<RefCell<Option<RafState>>>,
//}

//struct RafState {
    //id: i32,
    //closure: Closure<dyn FnMut(f64)>,
//}

//impl Raf {
    //fn new<F>(mut callback: F) -> Self where F: FnMut(f64) + 'static {
        //let state: Rc<RefCell<Option<RafState>>> = Rc::new(RefCell::new(None));

        //fn schedule(callback: &Closure<dyn FnMut(f64)>) -> i32 {
            //web_sys::window()
                //.unwrap_throw()
                //.request_animation_frame(callback.as_ref().unchecked_ref())
                //.unwrap_throw()
        //}

        //let closure = {
            //let state = state.clone();

            //Closure::wrap(Box::new(move |time| {
                //{
                    //let mut state = state.borrow_mut();
                    //let state = state.as_mut().unwrap_throw();
                    //state.id = schedule(&state.closure);
                //}

                //callback(time);
            //}) as Box<dyn FnMut(f64)>)
        //};

        //*state.borrow_mut() = Some(RafState {
            //id: schedule(&closure),
            //closure
        //});

        //Self { state }
    //}
//}

//impl Drop for Raf {
    //fn drop(&mut self) {
        //// The take is necessary in order to prevent an Rc leak
        //let state = self.state.borrow_mut().take().unwrap_throw();

        //web_sys::window()
            //.unwrap_throw()
            //.cancel_animation_frame(state.id)
            //.unwrap_throw();
    //}
//}
