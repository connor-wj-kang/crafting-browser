use lazy_static::lazy_static;
use std::{
    borrow::Borrow,
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
    fmt::{self, write, Display},
    rc::{Rc, Weak},
};
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    static ref HEAD_TAGS: HashSet<&'static str> = HashSet::from([
        "base", "basefont", "bgsound", "noscript", "link", "meta", "title", "style", "script",
    ]);
    static ref SELF_COLSING_TAG: HashSet<&'static str> = HashSet::from([
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
    ]);
}

#[derive(Debug)]
pub enum NodeData<'html> {
    Text {
        text: &'html str,
    },
    Element {
        tag: &'html str,
        attributes: HashMap<String, String>,
    },
}

pub struct Node<'html> {
    pub parent: Option<Weak<Node<'html>>>,
    pub style: RefCell<HashMap<String, String>>,
    pub children: RefCell<Vec<Rc<Node<'html>>>>,
    pub data: NodeData<'html>,
}

impl<'html> Node<'html> {
    fn new(data: NodeData<'html>, parent: Option<Weak<Node<'html>>>) -> Rc<Self> {
        Rc::new(Self {
            parent,
            style: RefCell::new(HashMap::new()),
            children: RefCell::new(Vec::new()),
            data,
        })
    }
}

impl<'html> fmt::Display for Node<'html> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            NodeData::Text { text } => write!(f, "{text}")?,
            NodeData::Element { tag, .. } => {
                write!(f, "<{tag}>")?;
                for child in self.children.borrow().iter() {
                    write!(f, "{child}")?;
                }
                write!(f, "</{tag}>")?;
            }
        }

        Ok(())
    }
}

impl<'html> fmt::Debug for Node<'html> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Node")
            .field("data", &self.data)
            .field("children", &self.children)
            .finish()
    }
}

pub struct HtmlParser<'html> {
    body: &'html str,
    unfinished: Vec<Rc<Node<'html>>>,
}

impl<'html> HtmlParser<'html> {
    pub fn new(body: &'html str) -> Self {
        Self {
            body,
            unfinished: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Rc<Node<'html>> {
        let mut start = 0;
        let mut end = 0;
        let mut in_tag = false;

        self.body.graphemes(true).for_each(|char| match char {
            "<" => {
                in_tag = true;
                if end != start {
                    self.add_text(&self.body[start..end]);
                }
                end += 1;
                start = end;
            }
            ">" => {
                in_tag = false;
                self.add_tag(&self.body[start..end]);
                end += 1;
                start = end;
            }
            _ => end += char.len(),
        });

        if !in_tag && end != start {
            self.add_text(&self.body[start..end]);
        }

        self.finish()
    }

    fn add_text(&mut self, text: &'html str) {
        if text.trim().is_empty() {
            return;
        }

        self.implicit_tags("");

        let parent = self.unfinished.last().unwrap();
        let node = Node::new(NodeData::Text { text }, Some(Rc::downgrade(parent)));
        parent.children.borrow_mut().push(node);
    }

    fn add_tag(&mut self, tag: &'html str) {
        let (tag, attributes) = self.get_attributes(tag);
        self.implicit_tags(tag);

        if tag.starts_with("/") && self.unfinished.len() > 1 {
            let node = self.unfinished.pop().unwrap();
            let parent = self.unfinished.last().unwrap();
            parent.children.borrow_mut().push(node);
            return;
        }

        if SELF_COLSING_TAG.contains(tag) {
            let parent = self.unfinished.last().unwrap();
            let node = Node::new(
                NodeData::Element { tag, attributes },
                Some(Rc::downgrade(parent)),
            );
            parent.children.borrow_mut().push(node);
            return;
        }

        let parent = self
            .unfinished
            .last()
            .map(|rc_parent| Rc::downgrade(rc_parent));
        let node = Node::new(NodeData::Element { tag, attributes }, parent);
        self.unfinished.push(node);
    }

    fn get_attributes(&self, text: &'html str) -> (&'html str, HashMap<String, String>) {
        let mut parts = text.split_whitespace();
        let tag = parts.next().unwrap();
        let mut attributes = HashMap::new();

        parts.for_each(|attrpair| {
            if attrpair.contains("=") {
                let mut key_value = attrpair.splitn(2, "=");
                let key = key_value.next().unwrap();
                let mut value = key_value.next().unwrap();
                if value.len() > 2 && (value.starts_with("'") || value.starts_with("\"")) {
                    value = &value[1..value.len() - 1];
                }
                attributes.insert(key.to_lowercase(), value.to_string());
            } else {
                attributes.insert(attrpair.to_lowercase(), String::new());
            }
        });

        (tag, attributes)
    }

    fn implicit_tags(&mut self, tag: &'html str) {
        loop {
            let open_tags = self
                .unfinished
                .iter()
                .map(|node| {
                    if let NodeData::Element { tag, .. } = *node.data.borrow() {
                        return tag;
                    }

                    unreachable!()
                })
                .collect::<Vec<&str>>();

            if open_tags.is_empty() && tag != "html" {
                self.add_tag("html");
                continue;
            }

            if open_tags.len() == 1
                && open_tags[0] == "html"
                && tag != "head"
                && tag != "body"
                && tag != "/html"
            {
                if HEAD_TAGS.contains(tag) {
                    self.add_tag("head");
                } else {
                    self.add_tag("body");
                }
                continue;
            }

            if open_tags.len() == 2
                && open_tags[0] == "html"
                && open_tags[1] == "head"
                && !HEAD_TAGS.contains(tag)
                && tag != "/head"
            {
                self.add_tag("/head");
                continue;
            }

            break;
        }
    }

    fn finish(&mut self) -> Rc<Node<'html>> {
        if !self.unfinished.is_empty() {
            self.implicit_tags("");
        }

        while self.unfinished.len() > 1 {
            let node = self.unfinished.pop().unwrap();
            let parent = self.unfinished.last().unwrap();
            parent.children.borrow_mut().push(node);
        }

        self.unfinished.pop().unwrap()
    }
}
