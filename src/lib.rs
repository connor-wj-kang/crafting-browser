mod html_parser;
mod layout;

use html_parser::HtmlParser;
use layout::Layout;
use std::{cell::RefCell, f64, rc::Rc};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast, JsValue,
};
use web_sys::{console, Event, WheelEvent, Window};

#[wasm_bindgen(start)]
fn start() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);
    let canvas = Rc::new(canvas);
    let window = Rc::new(window);
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let context = Rc::new(context);
    let scroll_y = Rc::new(RefCell::new(0.0));
    let tree = HtmlParser::new("<p>4-4<big>Quoted attributes</big>. Quoted attributes can contain spaces and right angle brackets. Fix the lexer so that this is supported properly. Hint: the current lexer is a finite state machine, with two states (determined by <code>in_tag</code>). You’ll need more states.</p>").parse();

    Layout::new(&context, &canvas)
        .calc_display_list(tree)
        .into_iter()
        .for_each(|display_info| {
            context.set_font(
                format!(
                    "{} {} {}px serif",
                    display_info.font_style, display_info.font_weight, display_info.font_size
                )
                .as_str(),
            );

            context
                .fill_text(
                    display_info.text,
                    display_info.x,
                    display_info.y - *scroll_y.borrow(),
                )
                .unwrap();
        });
}
