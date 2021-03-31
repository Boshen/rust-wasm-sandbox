use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

pub fn window() -> web_sys::Window {
    web_sys::window().unwrap()
}

pub fn document() -> web_sys::Document {
    window().document().unwrap()
}

pub fn canvas(id: &str) -> HtmlCanvasElement {
    document()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

pub fn canvas_context<T: JsCast>(canvas: &HtmlCanvasElement, ctx: &str) -> T {
    canvas
        .get_context(ctx)
        .unwrap()
        .unwrap()
        .dyn_into::<T>()
        .unwrap()
}

fn run_request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

pub fn request_animation_frame<F>(callback: F)
where
    F: Fn() + 'static,
{
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        callback();
        run_request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    run_request_animation_frame(g.borrow().as_ref().unwrap());
}

pub fn add_mouse_event_listener<F>(canvas: &HtmlCanvasElement, event: &str, callback: F)
where
    F: Fn(web_sys::MouseEvent) + 'static,
{
    let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_)>);
    canvas
        .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
}
