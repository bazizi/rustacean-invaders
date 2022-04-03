mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn log(str : &str) {
    web_sys::console::log_1(&JsValue::from_str(str));
}

#[wasm_bindgen]
pub fn main()
{
    let canvas = document().get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let canvas_context = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    
    log(&format!("Canvas size: {}x{}", canvas.width(), canvas.height()).to_string());
    canvas_context.fill_rect(0., 0., 500., 500.);   
}
