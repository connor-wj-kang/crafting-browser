mod html_parser;
mod layout;
mod token;

use layout::Layout;
use std::{
    cell::{Cell, RefCell},
    f64,
    rc::Rc,
};
use token::Token;
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

    {
        let tokens = Token::lex(
            "<p>Right now, all of the text on the page is drawn with one font. But
web pages sometimes specify that text should be <strong>bold</strong> or
<em>italic</em> using the <code>&lt;b&gt;</code> and
<code>&lt;i&gt;</code> tags. It’d be nice to support that, but right
now, the code resists this: the <code>layout</code> function only
receives the text of the page as input, and so has no idea where the
bold and italics tags are.</p>",
        );
        let canvas_inside = canvas.clone();
        let window_inside = window.clone();
        let scroll_y = scroll_y.clone();
        let context_inside_closure = context.clone();
        let closure = Closure::<dyn Fn(Event)>::new(move |_| {
            console::log_1(&JsValue::from_f64(*scroll_y.borrow()));
            canvas_inside.set_width(window_inside.inner_width().unwrap().as_f64().unwrap() as u32);
            canvas_inside
                .set_height(window_inside.inner_height().unwrap().as_f64().unwrap() as u32);
            context_inside_closure.clear_rect(
                0.0,
                0.0,
                canvas_inside.width() as f64,
                canvas_inside.height() as f64,
            );

            Layout::new(&context_inside_closure, &canvas_inside)
                .calc_display_list(&tokens)
                .into_iter()
                .for_each(|display_info| {
                    context_inside_closure.set_font(
                        format!(
                            "{} {} {}px serif",
                            display_info.font_style,
                            display_info.font_weight,
                            display_info.font_size
                        )
                        .as_str(),
                    );

                    context_inside_closure
                        .fill_text(
                            display_info.text,
                            display_info.x,
                            display_info.y - *scroll_y.borrow() * 0.05,
                        )
                        .unwrap();
                });
        });

        window
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    let tokens = Token::lex(
        "<p>Right now, all of the text on the page is drawn with one font. But
web pages sometimes specify that text should be <strong>bold</strong> or
<em>italic</em> using the <code>&lt;b&gt;</code> and
<code>&lt;i&gt;</code> tags. It’d be nice to support that, but right
now, the code resists this: the <code>layout</code> function only
receives the text of the page as input, and so has no idea where the
bold and italics tags are.</p>",
    );

    {
        let tokens = Token::lex(
            "<p>Right now, all of the text on the page is drawn with one font. But
web pages sometimes specify that text should be <strong>bold</strong> or
<em>italic</em> using the <code>&lt;b&gt;</code> and
<code>&lt;i&gt;</code> tags. It’d be nice to support that, but right
now, the code resists this: the <code>layout</code> function em <b><i>bold italic</i></b> only
receives the text of the page as input, and so has no idea where the
bold and italics tags are.</p>",
        );
        let canvas_inside_closure = canvas.clone();
        let context_inside_closure = context.clone();
        let scroll_y = scroll_y.clone();
        let handle_wheel_move = Closure::<dyn FnMut(_)>::new(move |event: WheelEvent| {
            *scroll_y.borrow_mut() += event.delta_y();
            context_inside_closure.clear_rect(
                0.0,
                0.0,
                canvas_inside_closure.width() as f64,
                canvas_inside_closure.height() as f64,
            );

            Layout::new(&context_inside_closure, &canvas_inside_closure)
                .calc_display_list(&tokens)
                .into_iter()
                .for_each(|display_info| {
                    context_inside_closure.set_font(
                        format!(
                            "{} {} {}px serif",
                            display_info.font_style,
                            display_info.font_weight,
                            display_info.font_size
                        )
                        .as_str(),
                    );

                    context_inside_closure
                        .fill_text(
                            display_info.text,
                            display_info.x,
                            display_info.y - *scroll_y.borrow() * 0.05,
                        )
                        .unwrap();
                });

            context_inside_closure.begin_path();
            context_inside_closure.set_fill_style(&JsValue::from_str("gray"));
            context_inside_closure
                .round_rect_with_f64(
                    canvas_inside_closure.width() as f64 - 15.0,
                    5.0 + *scroll_y.borrow() * 0.01,
                    10.0,
                    50.0,
                    999.0,
                )
                .unwrap();
            context_inside_closure.fill();
        });

        canvas
            .add_event_listener_with_callback("wheel", handle_wheel_move.as_ref().unchecked_ref())
            .unwrap();
        handle_wheel_move.forget();
    }

    Layout::new(&context, &canvas)
        .calc_display_list(&tokens)
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

    context.begin_path();
    context.set_fill_style(&JsValue::from_str("gray"));
    context
        .round_rect_with_f64(
            canvas.width() as f64 - 15.0,
            5.0 + *scroll_y.borrow() * 0.01,
            10.0,
            50.0,
            999.0,
        )
        .unwrap();
    context.fill();
}
