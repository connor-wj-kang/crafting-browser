use super::Selector;
use crate::html_parser::html_node::HtmlNode;
use std::rc::Rc;

#[derive(Debug)]
pub struct TagSelector {
    tag: String,
    priority: usize,
}

impl TagSelector {
    pub fn new(tag: String) -> Self {
        Self { tag, priority: 1 }
    }
}

impl Selector for TagSelector {
    fn matches<'html>(&self, node: Rc<HtmlNode<'html>>) -> bool {
        node.get_tag().is_some_and(|tag| self.tag == tag)
    }

    fn priority(&self) -> usize {
        self.priority
    }
}
