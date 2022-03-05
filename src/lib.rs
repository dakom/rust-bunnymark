mod bunny;
mod data;
mod hud;
mod game_start;
mod game_loop;
mod state;
mod config;
mod renderer;
mod fps;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

// enable logging and panic hook only during debug builds
cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook", debug_assertions))] {
        fn setup() {
            wasm_logger::init(wasm_logger::Config::default());
            console_error_panic_hook::set_once();
            log::info!("rust logging enabled!!!");
        }
    } else {
        fn setup() {
            log::info!("rust logging disabled!"); //<-- won't be seen
        }
    }
}

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run() -> Result<js_sys::Promise, JsValue> {
    setup();

    log::info!("logging enabled!");
    game_start::start()
}
