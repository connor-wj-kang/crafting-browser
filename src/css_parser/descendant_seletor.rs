use super::Selector;
use crate::html_parser::html_node::HtmlNode;
use std::rc::Rc;

#[derive(Debug)]
pub struct DescendantSeletor {
    ancestor: Box<dyn Selector>,
    descendant: Box<dyn Selector>,
    priority: usize,
}

impl DescendantSeletor {
    pub fn new(ancestor: Box<dyn Selector>, descendant: Box<dyn Selector>) -> Self {
        let priority = ancestor.priority() + descendant.priority();

        Self {
            ancestor,
            descendant,
            priority,
        }
    }
}

impl Selector for DescendantSeletor {
    fn matches<'html>(&self, node: Rc<HtmlNode<'html>>) -> bool {
        let mut node = node;
        if !self.descendant.matches(node.clone()) {
            return false;
        }

        while let Some(ref parent) = node.parent {
            if self.ancestor.matches(parent.upgrade().unwrap()) {
                return true;
            }

            node = parent.upgrade().unwrap();
        }

        false
    }

    fn priority(&self) -> usize {
        self.priority
    }
}
