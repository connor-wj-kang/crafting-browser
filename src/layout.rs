use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::html_parser::Node;

trait Layout {}

struct DocuementLayout {}

struct BlockLayout {}

struct LineLayout<'html> {
    node: Weak<Node<'html>>,
    parent: Weak<dyn Layout>,
    previous: Option<Weak<dyn Layout>>,
    children: RefCell<Vec<Rc<dyn Layout>>>,
}

impl<'html> LineLayout<'html> {
    fn new(
        node: Weak<Node<'html>>,
        parent: Weak<dyn Layout>,
        previous: Option<Weak<dyn Layout>>,
    ) -> Rc<Self> {
        Rc::new(Self {
            node,
            parent,
            previous,
            children: RefCell::new(Vec::new()),
        })
    }
}

struct TextLayout<'html> {
    node: Weak<Node<'html>>,
    word: &'html str,
    parent: Weak<dyn Layout>,
    previous: Option<Weak<dyn Layout>>,
    children: RefCell<Vec<Rc<dyn Layout>>>,
}

impl<'html> TextLayout<'html> {
    fn new(
        node: Weak<Node<'html>>,
        word: &'html str,
        parent: Weak<dyn Layout>,
        previous: Option<Weak<dyn Layout>>,
    ) -> Rc<Self> {
        Rc::new(Self {
            node,
            word,
            parent,
            previous,
            children: RefCell::new(Vec::new()),
        })
    }
}

struct InputLayout {}
