pub mod html_node;

use html_node::HtmlNode;
use std::{collections::HashMap, rc::Rc};
use unicode_segmentation::UnicodeSegmentation;

static HEAD_TAGS: [&str; 9] = [
    "base", "basefont", "bgsound", "noscript", "link", "meta", "title", "style", "script",
];

static SELF_COLSING_TAG: [&str; 14] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

pub struct HtmlParser<'html> {
    source: &'html str,
    unfinished: Vec<Rc<HtmlNode<'html>>>,
}

impl<'html> HtmlParser<'html> {
    pub fn new(source: &'html str) -> Self {
        Self {
            source,
            unfinished: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Rc<HtmlNode<'html>> {
        let mut start = 0;
        let mut end = 0;
        let mut in_tag = false;

        self.source.graphemes(true).for_each(|char| match char {
            "<" => {
                in_tag = true;
                if end != start {
                    self.add_text(&self.source[start..end]);
                }
                end += 1;
                start = end;
            }
            ">" => {
                in_tag = false;
                self.add_tag(&self.source[start..end]);
                end += 1;
                start = end;
            }
            _ => end += char.len(),
        });

        if !in_tag && end != start {
            self.add_text(&self.source[start..end]);
        }

        self.finish()
    }

    fn add_text(&mut self, text: &'html str) {
        if text.trim().is_empty() {
            return;
        }

        self.implicit_tags("");

        let parent = self.unfinished.last().unwrap();
        let node = HtmlNode::new_text_node(text, Some(Rc::downgrade(parent)));
        parent.children.borrow_mut().push(node);
    }

    fn add_tag(&mut self, tag: &'html str) {
        let (tag, attributes) = self.parse_attributes(tag);
        self.implicit_tags(tag);

        if tag.starts_with("/") && self.unfinished.len() > 1 {
            let node = self.unfinished.pop().unwrap();
            let parent = self.unfinished.last().unwrap();
            parent.children.borrow_mut().push(node);
            return;
        }

        if SELF_COLSING_TAG.contains(&tag) {
            let parent = self.unfinished.last().unwrap();
            let node = HtmlNode::new_element_node(tag, attributes, Some(Rc::downgrade(parent)));
            parent.children.borrow_mut().push(node);
            return;
        }

        let parent = self
            .unfinished
            .last()
            .map(|rc_parent| Rc::downgrade(rc_parent));
        let node = HtmlNode::new_element_node(tag, attributes, parent);
        self.unfinished.push(node);
    }

    fn parse_attributes(&self, text: &'html str) -> (&'html str, HashMap<String, String>) {
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
                .map(|node| node.get_tag().unwrap())
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
                if HEAD_TAGS.contains(&tag) {
                    self.add_tag("head");
                } else {
                    self.add_tag("body");
                }
                continue;
            }

            if open_tags.len() == 2
                && open_tags[0] == "html"
                && open_tags[1] == "head"
                && !HEAD_TAGS.contains(&tag)
                && tag != "/head"
            {
                self.add_tag("/head");
                continue;
            }

            break;
        }
    }

    fn finish(&mut self) -> Rc<HtmlNode<'html>> {
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
