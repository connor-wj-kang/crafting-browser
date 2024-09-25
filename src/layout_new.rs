use crate::{
    font::{get_font, FONTS},
    html_parser::{Node, NodeData},
};
use core::fmt;
use lazy_static::lazy_static;
use sdl2::hint::set_with_priority;
use skia_safe::{Font, Paint};
use std::{
    borrow::Borrow,
    cell::{Cell, Ref, RefCell},
    collections::HashSet,
    rc::{Rc, Weak},
};

static WIDTH: f32 = 800.0;
static HEIGHT: f32 = 600.0;
static HSTEP: f32 = 13.0;
static VSTEP: f32 = 18.0;

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

impl From<&'_ str> for FontWeight {
    fn from(value: &'_ str) -> Self {
        match value {
            "normal" => FontWeight::Normal,
            _ => FontWeight::Bold,
        }
    }
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

impl From<&'_ str> for FontStyle {
    fn from(value: &'_ str) -> Self {
        match value {
            "normal" => FontStyle::Normal,
            _ => FontStyle::Italic,
        }
    }
}

impl fmt::Display for FontStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Normal => write!(f, "normal"),
            Self::Italic => write!(f, "italic"),
        }
    }
}

#[derive(Debug, Default)]
struct Dimension {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Dimension {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug)]
pub struct DisplayInfo<'html> {
    pub x: f32,
    pub y: f32,
    pub text: &'html str,
    pub size: f32,
    pub style: FontStyle,
    pub weight: FontWeight,
    pub color: String,
}

pub trait Layout<'html>: 'html {
    fn layout(self: Rc<Self>);
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn width(&self) -> f32;
    fn paint(&self) -> Vec<Box<dyn Drawable<'html> + 'html>>;
    fn children(&self) -> Ref<'_, Vec<Rc<BlockLayout<'html>>>>;
}

pub struct BlockLayout<'html> {
    node: Weak<Node<'html>>,
    parent: Weak<dyn Layout<'html> + 'html>,
    previous: Option<Weak<BlockLayout<'html>>>,
    children: RefCell<Vec<Rc<LineLayout<'html>>>>,
    x: Cell<f32>,
    y: Cell<f32>,
    width: Cell<f32>,
    height: Cell<f32>,
    cursor_x: Cell<f32>,
    cursor_y: Cell<f32>,
    weight: Cell<FontWeight>,
    style: Cell<FontStyle>,
    size: Cell<f32>,
    line: RefCell<Vec<DisplayInfo<'html>>>,
    display_list: RefCell<Vec<DisplayInfo<'html>>>,
}

impl<'html> BlockLayout<'html> {
    pub fn new(
        node: Weak<Node<'html>>,
        parent: Weak<dyn Layout<'html> + 'html>,
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

    fn word(&self, node: Weak<Node<'html>>, word: &'html str) {
        let upgrade_node = node.upgrade().unwrap();
        let node_styles = upgrade_node.style.borrow();
        let weight = node_styles.get("font-weight").unwrap().into();
        let style = node_styles.get("font-style").unwrap().into();
        let size = node_styles.get("font-size").unwrap();
        let size = &size[0..size.len() - 2].parse::<f32>().unwrap() * 0.75;
        let font = get_font(size, weight, style);
        let width = font.measure_str(word, None).1.width();

        if self.cursor_x.get() + width > self.width.get() {
            self.new_line();
        }

        let lines = self.children.borrow();
        let line = lines.last().unwrap();
        let previous_word = line
            .children
            .borrow()
            .last()
            .map(|word| Rc::downgrade(word));
        let text = TextLayout::new(node, word, Some(Rc::downgrade(line)), previous_word);
        line.children.borrow_mut().push(text);
        self.cursor_x
            .set(self.cursor_x.get() + width + font.measure_str(" ", None).1.width());
    }

    fn new_line(&self) {}

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
                (f32::NEG_INFINITY, f32::NEG_INFINITY),
                |(ascent, descent), (x, y)| (ascent.max(x), descent.max(y)),
            );

        let base_line = self.cursor_y.get() + 1.25 * max_ascent;
        self.line.borrow().iter().for_each(|display_info| {
            let x = self.x.get() + display_info.x;
            let y = self.y.get() + base_line;
            // console::log_1(&JsValue::from_str(
            //     format!("{} {} {}", display_info.text, x, y).as_str(),
            // ));
            self.display_list.borrow_mut().push(DisplayInfo {
                x,
                y,
                color: display_info.color.clone(),
                ..(*display_info)
            });
        });

        self.cursor_x.set(0.0);
        self.line.borrow_mut().clear();
        self.cursor_y.set(base_line + 1.25 * max_descent);
    }

    fn recurse(&self, node: Rc<Node<'html>>) {
        match node.data {
            NodeData::Text { text } => text
                .split_whitespace()
                .for_each(|word| self.word(self.node.clone(), word)),
            NodeData::Element { tag, .. } => {
                if tag == "br" {
                    self.flush();
                }
                node.children.borrow().iter().for_each(|child| {
                    self.recurse(child.clone());
                });
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
        self.width.set(self.parent.upgrade().unwrap().width());

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
                    .sum::<f32>(),
            );
        } else {
            self.height.set(self.cursor_y.get());
        }
    }

    fn x(&self) -> f32 {
        self.x.get()
    }

    fn y(&self) -> f32 {
        self.y.get()
    }

    fn width(&self) -> f32 {
        self.width.get()
    }

    fn paint(&self) -> Vec<Box<dyn Drawable<'html> + 'html>> {
        let mut cmds: Vec<Box<dyn Drawable<'html> + 'html>> = Vec::new();
        let styles = self.node.style.borrow();
        let bg_color = styles
            .get("background-color")
            .map(|bg| bg.as_str())
            .unwrap_or("transparent");

        if bg_color != "transparent" {
            let (x2, y2) = (self.x.get(), self.y.get() + self.height.get());
            let rect = DrawRect::new(self.x.get(), self.y.get(), x2, y2, bg_color.to_string());
            cmds.push(Box::new(rect));
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
                    display_info.color.clone(),
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

#[derive(Debug)]
pub struct DocumentLayout<'html> {
    node: Rc<Node<'html>>,
    children: RefCell<Vec<Rc<BlockLayout<'html>>>>,
    x: Cell<f32>,
    y: Cell<f32>,
    width: Cell<f32>,
    height: Cell<f32>,
}

impl<'html> DocumentLayout<'html> {
    pub fn new(node: Rc<Node<'html>>) -> Rc<Self> {
        Rc::new(Self {
            node,
            children: RefCell::new(Vec::new()),
            x: Cell::new(0.0),
            y: Cell::new(0.0),
            width: Cell::new(0.0),
            height: Cell::new(0.0),
        })
    }
}

impl<'html> Layout<'html> for DocumentLayout<'html> {
    fn layout(self: Rc<Self>) {
        let child = BlockLayout::new(
            self.node.clone(),
            Rc::downgrade(&(self.clone() as Rc<dyn Layout<'html> + 'html>)),
            None,
        );
        self.children.borrow_mut().push(child.clone());
        self.width.set(WIDTH - 2.0 * HSTEP);
        self.x.set(HSTEP);
        self.y.set(VSTEP);
        child.clone().layout();
        self.height.set(child.height.get());
    }

    fn x(&self) -> f32 {
        self.x.get()
    }

    fn y(&self) -> f32 {
        self.y.get()
    }
    fn width(&self) -> f32 {
        self.width.get()
    }

    fn paint(&self) -> Vec<Box<dyn Drawable<'html> + 'html>> {
        Vec::new()
    }

    fn children(&self) -> Ref<'_, Vec<Rc<BlockLayout<'html>>>> {
        self.children.borrow()
    }
}

pub trait Drawable<'html>: 'html {
    fn execute(&self, scroll: f32);
}

pub struct DrawRect {
    top: f32,
    left: f32,
    bottom: f32,
    right: f32,
    color: String,
}

impl DrawRect {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32, color: String) -> Self {
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
    fn execute(&self, scroll: f32) {
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
    top: f32,
    left: f32,
    text: &'html str,
    weight: FontWeight,
    style: FontStyle,
    size: f32,
    color: String,
}

impl<'html> DrawText<'html> {
    pub fn new(
        x1: f32,
        y1: f32,
        text: &'html str,
        weight: FontWeight,
        style: FontStyle,
        size: f32,
        color: String,
    ) -> Self {
        Self {
            top: y1,
            left: x1,
            text,
            weight,
            size,
            style,
            color,
        }
    }
}

impl<'html> Drawable<'html> for DrawText<'html> {
    fn execute(&self, scroll: f32) {
        let context = CONTEXT.with(|context| context.clone());
        let font = format!("{} {} {}px serif", self.style, self.weight, self.size);
        console::log_1(&JsValue::from_str(format!("{font}").as_str()));
        context.set_font(&font);
        context.set_fill_style(&JsValue::from_str(&self.color));
        context
            .fill_text(&self.text, self.left, self.top - scroll)
            .unwrap();
    }
}

struct LineLayout<'html> {
    node: Weak<Node<'html>>,
    parent: Weak<BlockLayout<'html>>,
    previous: Option<Weak<LineLayout<'html>>>,
    children: RefCell<Vec<Rc<TextLayout<'html>>>>,
    dimension: RefCell<Dimension>,
}

impl<'html> LineLayout<'html> {
    fn new(
        node: Weak<Node<'html>>,
        parent: Weak<BlockLayout<'html>>,
        previous: Option<Weak<LineLayout<'html>>>,
    ) -> Rc<Self> {
        Rc::new(Self {
            node,
            parent,
            previous,
            children: RefCell::new(Vec::new()),
            dimension: RefCell::new(Dimension::default()),
        })
    }
}

impl<'html> Layout<'html> for LineLayout<'html> {
    fn layout(self: Rc<Self>) {
        let parent = self.parent.upgrade().unwrap();
        let mut dimension = self.dimension.borrow_mut();
        dimension.width = parent.width.get();
        dimension.x = parent.x.get();

        if let Some(ref previous) = self.previous {
            let previous = previous.upgrade().unwrap();
            let previous = previous.dimension.borrow();
            dimension.y = previous.y + previous.width;
        } else {
            dimension.y = parent.y.get();
        }

        self.children
            .borrow()
            .iter()
            .for_each(|word| word.clone().layout());

        if self.children.borrow().is_empty() {
            dimension.height = 0.0;
            return;
        }

        let max_ascent = self
            .children
            .borrow()
            .iter()
            .map(|word| -word.font.metrics().1.ascent)
            .reduce(f32::max)
            .unwrap();
        let baseline = dimension.y + 1.25 * max_ascent;
        self.children.borrow().iter().for_each(|word| {
            word.dimension.borrow_mut().y = baseline + word.font.metrics().1.ascent
        });
        let max_descent = self
            .children
            .borrow()
            .iter()
            .map(|word| word.font.metrics().1.descent)
            .reduce(f32::max)
            .unwrap();
        dimension.height = 1.25 * (max_ascent + max_descent);
    }

    fn x(&self) -> f32 {
        todo!()
    }

    fn y(&self) -> f32 {
        todo!()
    }

    fn width(&self) -> f32 {
        todo!()
    }

    fn paint(&self) -> Vec<Box<dyn Drawable<'html> + 'html>> {
        todo!()
    }

    fn children(&self) -> Ref<'_, Vec<Rc<BlockLayout<'html>>>> {
        todo!()
    }
}

struct TextLayout<'html> {
    node: Weak<Node<'html>>,
    word: &'html str,
    font: RefCell<Font>,
    parent: Weak<LineLayout<'html>>,
    previous: Option<Weak<TextLayout<'html>>>,
    dimension: RefCell<Dimension>,
}

impl<'html> TextLayout<'html> {
    fn new(
        node: Weak<Node<'html>>,
        font: RefCell<Font>,
        word: &'html str,
        parent: Weak<LineLayout<'html>>,
        previous: Option<Weak<TextLayout<'html>>>,
    ) -> Rc<Self> {
        Rc::new(Self {
            node,
            font: RefCell::new(Font::default()),
            word,
            parent,
            previous,
            dimension: RefCell::new(Dimension::default()),
        })
    }
}

impl<'html> Layout<'html> for TextLayout<'html> {
    fn layout(self: Rc<Self>) {
        let dimension = self.dimension.borrow_mut();
        let upgrade_node = self.node.upgrade().unwrap();
        let styles = upgrade_node.style.borrow();
        let weight = styles.get("font-weight").unwrap().into();
        let style = styles.get("font-style").unwrap().into();
        let size = styles.get("font-size").unwrap();
        let size = size[0..size.len() - 2].parse::<f32>().unwrap() * 0.75;
        self.font.replace(get_font(size, weight, style));
        if let Some(ref previous) = self.previous {
            let previous = previous.upgrade().unwrap();
            let space_width = previous.font.borrow().measure_str(" ", None).1.width();
            let previous = previous.dimension.borrow();
            dimension.x = previous.x + previous.width + space_width;
        } else {
            dimension.x = self.parent.upgrade().unwrap().dimension.borrow().x;
        }

        dimension.height = 
    }

    fn x(&self) -> f32 {
        todo!()
    }

    fn y(&self) -> f32 {
        todo!()
    }

    fn width(&self) -> f32 {
        todo!()
    }

    fn paint(&self) -> Vec<Box<dyn Drawable<'html> + 'html>> {
        todo!()
    }

    fn children(&self) -> Ref<'_, Vec<Rc<BlockLayout<'html>>>> {
        todo!()
    }
}

fn measure_text_width(text: &str, style: FontStyle, weight: FontWeight, size: f32) -> f32 {
    let context = CONTEXT.with(|ctx| ctx.clone());
    context.set_font(&format!("{} {} {}px serif", style, weight, size.round()));
    context.measure_text(text).unwrap().width()
}
