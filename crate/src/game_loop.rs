use super::state::{State};
use super::hud::{Hud};

use std::rc::{Rc};
use std::cell::{RefCell};

use awsm::tick::{MainLoop, MainLoopOptions};

use awsm::webgl::{
    get_webgl_context_1, 
    WebGlContextOptions, 
    ClearBufferMask,
    WebGlCommon,
    WebGl1Renderer
};

pub fn begin_loop(renderer:Rc<RefCell<WebGl1Renderer>>, state:Rc<RefCell<State>>, hud:Hud) -> Result<(), awsm::errors::Error> {


    //callbacks
    let begin = move |time, delta| {
        //check for input
    };

    let update = move |delta| {
        //physics
    };

    let draw = move |interpolation| {
        //not doing
    };
    let end = {
        let state = Rc::clone(&state);
        move |fps:f64, abort:bool| {
            let mut state = state.borrow_mut();
            state.fps = fps.ceil() as u32;
            hud.update(&state);
        }
    };

    let main_loop = MainLoop::start(MainLoopOptions::default(), begin, update, draw, end)?;

    std::mem::forget(Box::new(main_loop));

    Ok(())
}
