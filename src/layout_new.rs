use crate::{
    html_parser::{Node, NodeData},
    CONTEXT,
};
use core::fmt;
use lazy_static::lazy_static;
use std::{
    borrow::BorrowMut,
    cell::{Cell, Ref, RefCell},
    collections::HashSet,
    fmt::format,
    rc::{Rc, Weak},
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

static WIDTH: f64 = 800.0;
static HEIGHT: f64 = 600.0;
static HSTEP: f64 = 13.0;
static VSTEP: f64 = 18.0;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum LayoutMode {
    Inline,
    Block,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
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
pub enum FontStyle {
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

pub trait Layout<'html>: 'html {
    fn layout(self: Rc<Self>);
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn paint(&self) -> Vec<Box<dyn Drawable<'html>>>;
    fn children(&self) -> Ref<'_, Vec<Rc<BlockLayout<'html>>>>;
}

pub struct BlockLayout<'html> {
    node: Rc<Node<'html>>,
    parent: Weak<dyn Layout<'html>>,
    previous: Option<Weak<BlockLayout<'html>>>,
    children: RefCell<Vec<Rc<BlockLayout<'html>>>>,
    x: Cell<f64>,
    y: Cell<f64>,
    width: Cell<f64>,
    height: Cell<f64>,
    cursor_x: Cell<f64>,
    cursor_y: Cell<f64>,
    weight: Cell<FontWeight>,
    style: Cell<FontStyle>,
    size: Cell<f64>,
    line: RefCell<Vec<DisplayInfo<'html>>>,
    display_list: RefCell<Vec<DisplayInfo<'html>>>,
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
            cursor_x: Cell::new(0.0),
            cursor_y: Cell::new(0.0),
            weight: Cell::new(FontWeight::Normal),
            style: Cell::new(FontStyle::Normal),
            size: Cell::new(12.0),
            line: RefCell::new(Vec::new()),
            display_list: RefCell::new(Vec::new()),
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

    fn word(&self, word: &'html str) {
        let font = format!(
            "{} {} {}px serif",
            self.style.get(),
            self.weight.get(),
            self.size.get()
        );

        let context = CONTEXT.with(|context| context.clone());
        context.set_font(&font);
        let width = context.measure_text(word).unwrap().width();
        if self.cursor_x.get() + width > self.width.get() {
            self.flush();
        }
        self.line.borrow_mut().push(DisplayInfo {
            x: self.cursor_x.get(),
            y: self.cursor_y.get(),
            text: word,
            size: self.size.get(),
            style: self.style.get(),
            weight: self.weight.get(),
        });
        self.cursor_x
            .set(self.cursor_x.get() + width + context.measure_text(" ").unwrap().width());
    }

    fn flush(&self) {
        if self.line.borrow().is_empty() {
            return;
        }
        let context = CONTEXT.with(|context| context.clone());
        let (max_ascent, max_descent) = self
            .line
            .borrow()
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

        let base_line = self.cursor_y.get() + 1.25 * max_ascent;
        self.line.borrow().iter().for_each(|display_info| {
            let x = self.x.get() + display_info.x;
            let y = self.y.get() + base_line;
            self.display_list.borrow_mut().push(DisplayInfo {
                x,
                y,
                ..(*display_info)
            });
        });

        self.cursor_x.set(0.0);
        self.line.borrow_mut().clear();
        self.cursor_y.set(base_line + 1.25 * max_descent);
    }

    fn recurse(&self, tree: Rc<Node<'html>>) {
        match tree.data {
            NodeData::Text { text } => text.split_whitespace().for_each(|word| self.word(word)),
            NodeData::Element { tag, .. } => {
                self.open_tag(tag);
                tree.children.borrow().iter().for_each(|child| {
                    self.recurse(child.clone());
                });
                self.close_tag(tag);
            }
        }
    }

    fn open_tag(&self, tag: &'html str) {
        match tag {
            "i" => self.style.set(FontStyle::Italic),
            "b" => self.weight.set(FontWeight::Bold),
            "small" => self.size.set(self.size.get() - 2.0),
            "big" => self.size.set(self.size.get() + 4.0),
            "br" => self.flush(),
            _ => {}
        }
    }

    fn close_tag(&self, tag: &'html str) {
        match tag {
            "i" => self.style.set(FontStyle::Normal),
            "b" => self.weight.set(FontWeight::Normal),
            "small" => self.size.set(self.size.get() + 2.0),
            "big" => self.size.set(self.size.get() - 4.0),
            "p" => {
                self.flush();
                self.cursor_y.set(self.cursor_y.get() + VSTEP);
            }
            _ => {}
        }
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
        } else {
            self.recurse(self.node.clone());
            self.flush();
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
            self.height.set(self.cursor_y.get());
        }
    }

    fn x(&self) -> f64 {
        self.x.get()
    }

    fn y(&self) -> f64 {
        self.y.get()
    }

    fn paint(&self) -> Vec<Box<dyn Drawable<'html>>> {
        let mut cmds: Vec<Box<dyn Drawable>> = Vec::new();
        if let NodeData::Element { tag, .. } = self.node.data {
            if tag == "pre" {
                let (x2, y2) = (self.x.get(), self.y.get() + self.height.get());
                let rect = DrawRect::new(self.x.get(), self.y.get(), x2, y2, "gray".to_string());
                cmds.push(Box::new(rect));
            }
        }

        if self.layout_mode() == LayoutMode::Inline {
            self.display_list.borrow().iter().for_each(|display_info| {
                let text = DrawText::new(
                    display_info.x,
                    display_info.y,
                    display_info.text,
                    display_info.weight,
                    display_info.style,
                    display_info.size,
                );
                cmds.push(Box::new(text));
            });
        }

        cmds
    }

    fn children(&self) -> Ref<'_, Vec<Rc<BlockLayout<'html>>>> {
        self.children.borrow()
    }
}

pub struct DocumentLayout<'html> {
    node: Rc<Node<'html>>,
    children: RefCell<Vec<Rc<BlockLayout<'html>>>>,
    x: Cell<f64>,
    y: Cell<f64>,
    width: Cell<f64>,
    height: Cell<f64>,
}

impl<'html> Layout<'html> for DocumentLayout<'html> {
    fn layout(self: Rc<Self>) {
        let child = BlockLayout::new(
            self.node.clone(),
            Rc::downgrade(&(self.clone() as Rc<dyn Layout<'html>>)),
            None,
        );
        self.children.borrow_mut().push(child.clone());
        self.width.set(WIDTH - 2.0 * HSTEP);
        self.x.set(HSTEP);
        self.y.set(VSTEP);
        child.clone().layout();
        self.height.set(child.height.get());
    }

    fn x(&self) -> f64 {
        self.x.get()
    }

    fn y(&self) -> f64 {
        self.y.get()
    }

    fn paint(&self) -> Vec<Box<dyn Drawable<'html>>> {
        Vec::new()
    }

    fn children(&self) -> Ref<'_, Vec<Rc<BlockLayout<'html>>>> {
        self.children.borrow()
    }
}

pub trait Drawable<'html>: 'html {
    fn execute(&self, scroll: f64);
}

pub struct DrawRect {
    top: f64,
    left: f64,
    bottom: f64,
    right: f64,
    color: String,
}

impl DrawRect {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64, color: String) -> Self {
        Self {
            top: y1,
            left: x1,
            bottom: y2,
            right: x2,
            color,
        }
    }
}

impl<'html> Drawable<'html> for DrawRect {
    fn execute(&self, scroll: f64) {
        let context = CONTEXT.with(|context| context.clone());
        context.set_fill_style(JsValue::from_str(&self.color).as_ref());
        context.fill_rect(
            self.left,
            self.top - scroll,
            self.right - self.left,
            self.bottom - self.top,
        );
    }
}

pub struct DrawText<'html> {
    top: f64,
    left: f64,
    text: &'html str,
    weight: FontWeight,
    style: FontStyle,
    size: f64,
}

impl<'html> DrawText<'html> {
    pub fn new(
        x1: f64,
        y1: f64,
        text: &'html str,
        weight: FontWeight,
        style: FontStyle,
        size: f64,
    ) -> Self {
        Self {
            top: y1,
            left: x1,
            text,
            weight,
            size,
            style,
        }
    }
}

impl<'html> Drawable<'html> for DrawText<'html> {
    fn execute(&self, scroll: f64) {
        let context = CONTEXT.with(|context| context.clone());
        let font = format!("{} {} {}px serif", self.style, self.weight, self.size);
        context.set_font(&font);
        context
            .fill_text(&self.text, self.left, self.top - scroll)
            .unwrap();
    }
}
