pub mod html_parser;
pub mod layout_new;

use html_parser::HtmlParser;
use js_sys::wasm_bindgen;
use layout_new::{DrawRect, DrawText, Drawable, Layout};
use std::{borrow::Borrow, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

thread_local! {
    pub static CANVAS: Rc<HtmlCanvasElement> = {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        Rc::new(canvas)
    };

    pub static CONTEXT: Rc<CanvasRenderingContext2d>= {
        let context = CANVAS
            .with(|canvas| canvas.clone())
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Rc::new(context)
    }
}

fn paint_tree<'html>(
    layout_object: Rc<dyn Layout<'html> + 'html>,
) -> Vec<Box<dyn Drawable<'html> + 'html>> {
    let mut display_list: Vec<Box<dyn Drawable<'html>>> = Vec::new();
    display_list.extend(layout_object.clone().paint().into_iter());
    layout_object.children().iter().for_each(|child| {
        let a = paint_tree(child.clone() as Rc<dyn Layout<'html>>);
        display_list.extend(a.into_iter());
    });
    display_list
}

#[wasm_bindgen(start)]
pub fn start() {
    let nodes = HtmlParser::new("<h1>Block Layout</h1><p>So far, we’ve focused on text layout—and text is laid out horizontally in lines.<span><span>In European languages, at least!</span></span> But web pages are really constructed out of larger blocks, like headings, paragraphs, and menus, that stack vertically one after another. We need to add support for this kind of layout to our browser, and the way we’re going to do that involves expanding on the layout tree we’ve already built.</p><p>The core idea is that we’ll have a whole tree of <code>BlockLayout</code> objects (with a <code>DocumentLayout</code> at the root). Some will represent leaf blocks that contain text, and they’ll lay out their contents the way we’ve already implemented. But there will also be new, intermediate <code>BlockLayout</code>s with <code>BlockLayout</code> children, and they will stack their children vertically. (An example is shown in Figure 1.)</p>").parse();

    println!("{nodes}");
}
