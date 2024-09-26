use crate::html_parser::html_node::HtmlNode;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Accessibility<'html> {
    node: Rc<HtmlNode<'html>>,
    children: RefCell<Vec<Rc<Accessibility<'html>>>>,
    parent: Option<Weak<Accessibility<'html>>>,
    text: &'html str,
}
