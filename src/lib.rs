mod vec2;
mod spaceship;
mod utils;

use vec2::Vec2;
use spaceship::SpaceShip;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;

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

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
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
    let image = document().get_element_by_id("spaceship")
    .unwrap()
    .dyn_into::<web_sys::HtmlImageElement>()
    .unwrap();  
    image.set_width(50);
    image.set_height(50);

    let mut space_ship = SpaceShip::new();
    let movement = Rc::new(Cell::new(Vec2{x: 0.}));

    {
        let movement = movement.clone();  
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if event.key() == "ArrowRight"
            {
                movement.set(Vec2 { x: 1. });
            }
            else if event.key() == "ArrowLeft"
            {
                movement.set(Vec2 { x: -1. });
            }
        }) as Box<dyn FnMut(_)>);
        document().add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }


    {
        let movement = movement.clone();  
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if event.key() == "ArrowRight" || event.key() == "ArrowLeft"
            {
                movement.set(Vec2 { x: 0. });
            }
        }) as Box<dyn FnMut(_)>);
        document().add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }


    // Here we want to call `requestAnimationFrame` in a loop, but only a fixed
    // number of times. After it's done we want all our resources cleaned up. To
    // achieve this we're using an `Rc`. The `Rc` will eventually store the
    // closure we want to execute on each frame, but to start out it contains
    // `None`.
    //
    // After the `Rc` is made we'll actually create the closure, and the closure
    // will reference one of the `Rc` instances. The other `Rc` reference is
    // used to store the closure, request the first frame, and then is dropped
    // by this function.
    //
    // Inside the closure we've got a persistent `Rc` reference, which we use
    // for all future iterations of the loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        space_ship.update_position(movement.get());
        canvas_context.fill_rect(0., 0., canvas.width() as f64, canvas.height() as f64);
        canvas_context.draw_image_with_html_image_element(&image, space_ship.get_x(), canvas.height() as f64 - 50.).unwrap();        

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
