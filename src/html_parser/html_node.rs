use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    fmt::{self},
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub enum HtmlNodeInner<'html> {
    Text {
        text: &'html str,
    },
    Element {
        tag: &'html str,
        attributes: HashMap<String, String>,
    },
}

pub struct HtmlNode<'html> {
    pub parent: Option<Weak<HtmlNode<'html>>>,
    pub styles: RefCell<HashMap<String, String>>,
    pub children: RefCell<Vec<Rc<HtmlNode<'html>>>>,
    pub inner: HtmlNodeInner<'html>,
}

impl<'html> HtmlNode<'html> {
    pub fn paint_tree(self: Rc<Self>, indent: usize) {
        println!("{}{}", " ".repeat(indent), self);
        self.children.borrow().iter().for_each(|child| {
            child.clone().paint_tree(indent + 2);
        });
        if self.is_element_node() {
            println!("{}{}", " ".repeat(indent), self);
        }
    }

    fn new(data: HtmlNodeInner<'html>, parent: Option<Weak<HtmlNode<'html>>>) -> Rc<Self> {
        Rc::new(Self {
            parent,
            styles: RefCell::new(HashMap::new()),
            children: RefCell::new(Vec::new()),
            inner: data,
        })
    }

    pub fn new_text_node(text: &'html str, parent: Option<Weak<HtmlNode<'html>>>) -> Rc<Self> {
        Rc::new(Self {
            parent,
            styles: RefCell::new(HashMap::new()),
            children: RefCell::new(Vec::new()),
            inner: HtmlNodeInner::Text { text },
        })
    }

    pub fn new_element_node(
        tag: &'html str,
        attributes: HashMap<String, String>,
        parent: Option<Weak<HtmlNode<'html>>>,
    ) -> Rc<Self> {
        Rc::new(Self {
            parent,
            styles: RefCell::new(HashMap::new()),
            children: RefCell::new(Vec::new()),
            inner: HtmlNodeInner::Element { tag, attributes },
        })
    }

    pub fn is_text_node(&self) -> bool {
        match self.inner {
            HtmlNodeInner::Text { .. } => true,
            _ => false,
        }
    }

    pub fn is_element_node(&self) -> bool {
        match self.inner {
            HtmlNodeInner::Element { .. } => true,
            _ => false,
        }
    }

    pub fn get_tag(&self) -> Option<&str> {
        match self.inner {
            HtmlNodeInner::Element { tag, .. } => Some(tag),
            _ => None,
        }
    }

    pub fn get_attributes(&self) -> Option<&HashMap<String, String>> {
        match self.inner {
            HtmlNodeInner::Element { ref attributes, .. } => Some(attributes),
            _ => None,
        }
    }
}

impl<'html> fmt::Display for HtmlNode<'html> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            HtmlNodeInner::Text { text } => write!(f, "{text}")?,
            HtmlNodeInner::Element { tag, .. } => {
                write!(f, "<{tag}>")?;
            }
        };

        Ok(())
    }
}

impl<'html> fmt::Debug for HtmlNode<'html> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("HtmlNode")
            .field("data", &self.inner)
            .field("children", &self.children)
            .finish()
    }
}
