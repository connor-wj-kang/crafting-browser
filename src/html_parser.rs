use std::{cell::RefCell, rc::Rc};

struct Text<'text> {
    text: &'text str,
    parent: Option<Rc<RefCell<Node<'text>>>>,
}

struct Element<'tag> {
    tag: &'tag str,
    children: Vec<Rc<RefCell<Node<'tag>>>>,
    parent: Option<Rc<RefCell<Node<'tag>>>>,
}

enum Node<'body> {
    Text(Text<'body>),
    Element(Element<'body>),
}

impl<'body> Node<'body> {
    fn get_parent(&self) -> Option<Rc<RefCell<Self>>> {
        match self {
            Node::Text(_) => None,
            Node::Element(element) => element.parent.clone(),
        }
    }
}

struct HtmlParser<'body> {
    body: &'body str,
    unfinsihed: Vec<Rc<RefCell<Node<'body>>>>,
}

impl<'body> HtmlParser<'body> {
    fn new(body: &'body str) -> Self {
        Self {
            body,
            unfinsihed: Vec::new(),
        }
    }

    fn parse(&mut self) {}

    fn add_text(&self, text: &'body str) {
        let parent = self.unfinsihed.last().unwrap();

        let node = Rc::new(RefCell::new(Node::Text(Text {
            text,
            parent: Some(parent.clone()),
        })));

        if let Node::Element(ref mut element) = *parent.borrow_mut() {
            element.children.push(node)
        };
    }

    fn add_tag(&mut self, tag: &'body str) {
        if tag.starts_with("/") {
        } else {
            let parent = self.unfinsihed.last().unwrap();
            let node = Rc::new(RefCell::new(Node::Element(Element {
                tag,
                children: Vec::new(),
                parent: Some(parent.clone()),
            })));
            self.unfinsihed.push(node);
        }
    }
}
