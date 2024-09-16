use lazy_static::lazy_static;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::{write, Display},
    rc::Rc,
};
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    static ref HEAD_TAGS: HashSet<&'static str> = HashSet::from([
        "base", "basefont", "bgsound", "noscript", "link", "meta", "title", "style", "script",
    ]);
}

pub struct Text<'text> {
    pub text: &'text str,
    pub parent: Option<Rc<RefCell<Node<'text>>>>,
}

pub struct Element<'tag> {
    pub tag: &'tag str,
    pub children: Vec<Rc<RefCell<Node<'tag>>>>,
    pub attributes: HashMap<&'tag str, &'tag str>,
    pub parent: Option<Rc<RefCell<Node<'tag>>>>,
}

pub enum Node<'body> {
    Text(Text<'body>),
    Element(Element<'body>),
}

impl<'body> Display for Node<'body> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(text) => write!(f, "{}", text.text)?,
            Self::Element(element) => {
                write!(f, "<{}>", element.tag)?;
                for child in element.children.iter() {
                    write!(f, "{}", child.borrow())?;
                }
                write!(f, "</{}>", element.tag)?;
            }
        };

        Ok(())
    }
}

pub struct HtmlParser<'body> {
    body: &'body str,
    self_closing_tag: HashSet<&'static str>,
    unfinsihed: Vec<Rc<RefCell<Node<'body>>>>,
}

impl<'body> HtmlParser<'body> {
    pub fn new(body: &'body str) -> Self {
        let self_closing_tag = HashSet::from([
            "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
            "source", "track", "wbr",
        ]);

        Self {
            body,
            unfinsihed: Vec::new(),
            self_closing_tag,
        }
    }

    pub fn parse(&mut self) -> Rc<RefCell<Node<'body>>> {
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

    fn add_text(&mut self, text: &'body str) {
        if text.trim().is_empty() {
            return;
        }

        self.implicit_tags("");

        let parent = self.unfinsihed.last();

        let node = Rc::new(RefCell::new(Node::Text(Text {
            text,
            parent: parent.cloned(),
        })));

        if let Some(parent) = parent {
            if let Node::Element(ref mut element) = *parent.borrow_mut() {
                element.children.push(node)
            };
        }
    }

    fn add_tag(&mut self, tag: &'body str) {
        let (tag, attributes) = self.get_attributes(tag);

        if tag.starts_with("!") {
            return;
        }

        self.implicit_tags(tag);

        if tag.starts_with("/") {
            if self.unfinsihed.len() == 1 {
                return;
            }

            let node = self.unfinsihed.pop().unwrap();
            let parent = self.unfinsihed.last().unwrap();
            if let Node::Element(ref mut element) = *parent.borrow_mut() {
                element.children.push(node)
            };
            return;
        }

        if self.self_closing_tag.contains(tag) {
            let parent = self.unfinsihed.last();
            let node = Rc::new(RefCell::new(Node::Element(Element {
                tag,
                children: Vec::new(),
                attributes,
                parent: parent.cloned(),
            })));
            if let Node::Element(ref mut element) = *parent.unwrap().borrow_mut() {
                element.children.push(node)
            };
            return;
        }

        let parent = self.unfinsihed.last();
        let node = Rc::new(RefCell::new(Node::Element(Element {
            tag,
            children: Vec::new(),
            attributes,
            parent: parent.cloned(),
        })));
        self.unfinsihed.push(node);
    }

    fn finish(&mut self) -> Rc<RefCell<Node<'body>>> {
        if !self.unfinsihed.is_empty() {
            self.implicit_tags("");
        }

        while self.unfinsihed.len() > 1 {
            let node = self.unfinsihed.pop().unwrap();
            let parent = self.unfinsihed.last().unwrap();
            if let Node::Element(ref mut element) = *parent.borrow_mut() {
                element.children.push(node)
            };
        }

        self.unfinsihed.pop().unwrap()
    }

    fn get_attributes(&self, text: &'body str) -> (&'body str, HashMap<&'body str, &'body str>) {
        let parts = text.split_whitespace().collect::<Vec<&str>>();
        let tag = parts.get(0).unwrap();
        let mut attributes = HashMap::new();

        parts.iter().skip(1).for_each(|attribute_pair| {
            if attribute_pair.contains("=") {
                let mut key_and_value = attribute_pair.splitn(2, "=");
                let key = key_and_value.next().unwrap();
                let mut value = key_and_value.next().unwrap();
                if value.len() > 2 && (value.starts_with("'") || value.starts_with("\"")) {
                    value = &value[1..value.len() - 1];
                }
                attributes.insert(key, value);
            } else {
                attributes.insert(attribute_pair, "");
            }
        });

        (tag, attributes)
    }

    fn implicit_tags(&mut self, tag: &'body str) {
        loop {
            let open_tags = self
                .unfinsihed
                .iter()
                .map(|open_tag| {
                    if let Node::Element(ref element) = *open_tag.borrow() {
                        element.tag
                    } else {
                        ""
                    }
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
}
