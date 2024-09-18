use crate::html_parser::{Node, NodeData};
use core::fmt;
use lazy_static::lazy_static;
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
    rc::{Rc, Weak},
};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

thread_local! {
    static CANVAS: Rc<HtmlCanvasElement> = {
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

    static CONTEXT: Rc<CanvasRenderingContext2d>= {
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

lazy_static! {
    static ref BLOCK_ELEMENTS: HashSet<&'static str> = HashSet::from([
        "html",
        "body",
        "article",
        "section",
        "nav",
        "aside",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "hgroup",
        "header",
        "footer",
        "address",
        "p",
        "hr",
        "pre",
        "blockquote",
        "ol",
        "ul",
        "menu",
        "li",
        "dl",
        "dt",
        "dd",
        "figure",
        "figcaption",
        "main",
        "div",
        "table",
        "form",
        "fieldset",
        "legend",
        "details",
        "summary"
    ]);
}

enum LayoutMode {
    Inline,
    Block,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FontWeight {
    Normal,
    Bold,
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Normal => write!(f, "normal"),
            Self::Bold => write!(f, "bold"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FontStyle {
    Normal,
    Italic,
}

impl fmt::Display for FontStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Normal => write!(f, "normal"),
            Self::Italic => write!(f, "italic"),
        }
    }
}

pub struct DisplayInfo<'html> {
    pub x: f64,
    pub y: f64,
    pub text: &'html str,
    pub size: f64,
    pub style: FontStyle,
    pub weight: FontWeight,
}

trait Layout<'html>: 'html {
    fn layout(self: Rc<Self>);
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

struct BlockLayout<'html> {
    node: Rc<Node<'html>>,
    parent: Weak<dyn Layout<'html>>,
    previous: Option<Weak<BlockLayout<'html>>>,
    children: RefCell<Vec<Rc<BlockLayout<'html>>>>,
    x: Cell<f64>,
    y: Cell<f64>,
    width: Cell<f64>,
    height: Cell<f64>,
    cursor_x: f64,
    cursor_y: f64,
    weight: FontWeight,
    style: FontStyle,
    size: f64,
    line: Vec<DisplayInfo<'html>>,
    display_list: Vec<DisplayInfo<'html>>,
}

impl<'html> BlockLayout<'html> {
    fn new(
        node: Rc<Node<'html>>,
        parent: Weak<dyn Layout<'html>>,
        previous: Option<Weak<BlockLayout<'html>>>,
    ) -> Rc<Self> {
        Rc::new(Self {
            node,
            parent,
            previous,
            children: RefCell::new(Vec::new()),
            x: Cell::new(0.0),
            y: Cell::new(0.0),
            width: Cell::new(0.0),
            height: Cell::new(0.0),
            cursor_x: 0.0,
            cursor_y: 0.0,
            weight: FontWeight::Normal,
            style: FontStyle::Normal,
            size: 12.0,
            line: Vec::new(),
            display_list: Vec::new(),
        })
    }

    fn layout_mode(&self) -> LayoutMode {
        use LayoutMode::*;

        if let NodeData::Text { .. } = self.node.data {
            return Inline;
        }

        if self.node.children.borrow().iter().any(|child| {
            if let NodeData::Element { tag, .. } = child.data {
                BLOCK_ELEMENTS.contains(tag)
            } else {
                false
            }
        }) {
            return Block;
        }

        if !self.node.children.borrow().is_empty() {
            return Inline;
        }

        Block
    }

    fn word(&mut self, word: &'html str) {
        let font = format!("{} {} {}px serif", self.style, self.weight, self.size);

        let context = CONTEXT.with(|context| context.clone());
        context.set_font(&font);
        let width = context.measure_text(word).unwrap().width();
        if self.cursor_x + width > self.width.get() {
            self.flush();
        }
        self.line.push(DisplayInfo {
            x: self.cursor_x,
            y: self.cursor_y,
            text: word,
            size: self.size,
            style: self.style,
            weight: self.weight,
        });
        self.cursor_x += width + context.measure_text(" ").unwrap().width();
    }

    fn flush(&mut self) {
        if self.line.is_empty() {
            return;
        }
        let context = CONTEXT.with(|context| context.clone());
        let (max_ascent, max_descent) = self
            .line
            .iter()
            .map(|display_info| {
                context.set_font(
                    format!(
                        "{} {} {}px serif",
                        display_info.style, display_info.weight, display_info.size
                    )
                    .as_str(),
                );

                let font_metric = context.measure_text(display_info.text).unwrap();
                (
                    font_metric.font_bounding_box_ascent(),
                    font_metric.font_bounding_box_ascent(),
                )
            })
            .fold(
                (f64::NEG_INFINITY, f64::NEG_INFINITY),
                |(ascent, descent), (x, y)| (ascent.max(x), descent.max(y)),
            );

        let base_line = self.cursor_y + 1.25 * max_ascent;
        self.line.iter().for_each(|display_info| {
            let x = self.x.get() + display_info.x;
            let y = self.y.get() + base_line;
            self.display_list.push(DisplayInfo {
                x,
                y,
                ..(*display_info)
            });
        });

        self.cursor_x = 0.0;
        self.line.clear();
        self.cursor_y = base_line + 1.25 * max_descent;
    }
}

impl<'html> Layout<'html> for BlockLayout<'html> {
    fn layout(self: Rc<Self>) {
        self.x.set(self.parent.upgrade().unwrap().x());

        if let Some(ref previous) = self.previous {
            let previous = previous.upgrade().unwrap();
            self.y.set(previous.y.get() + previous.height.get());
        } else {
            self.y.set(self.parent.upgrade().unwrap().y());
        }

        let mode = self.layout_mode();

        if let LayoutMode::Block = mode {
            let mut previous = None;
            for child in self.node.children.borrow().iter() {
                let next = Self::new(
                    child.clone(),
                    Rc::downgrade(&(self.clone() as Rc<dyn Layout<'html> + 'html>)),
                    previous.take(),
                );
                self.children.borrow_mut().push(next.clone());
                previous = Some(Rc::downgrade(&next));
            }
        }

        self.children
            .borrow()
            .iter()
            .for_each(|child| child.clone().layout());

        if let LayoutMode::Block = mode {
            self.height.set(
                self.children
                    .borrow()
                    .iter()
                    .map(|child| child.height.get())
                    .sum::<f64>(),
            );
        } else {
            self.height.set(self.cursor_y);
        }
    }

    fn x(&self) -> f64 {
        unimplemented!()
    }

    fn y(&self) -> f64 {
        unimplemented!()
    }
}
