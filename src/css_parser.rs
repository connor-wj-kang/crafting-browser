use crate::html_parser::{Node, NodeData};
use core::fmt;
use lazy_static::lazy_static;
use std::{
    borrow::{Borrow, BorrowMut},
    collections::{HashMap, HashSet},
    fmt::format,
    rc::Rc,
};

lazy_static! {
    pub static ref INHERITED_PROPERTIES: HashMap<&'static str, &'static str> = HashMap::from([
        ("font-size", "16px"),
        ("font-style", "normal"),
        ("font-weight", "normal"),
        ("color", "black")
    ]);
}

pub struct CssParser<'css> {
    source: &'css str,
    current_index: usize,
}

impl<'css> CssParser<'css> {
    pub fn new(s: &'css str) -> Self {
        Self {
            source: s,
            current_index: 0,
        }
    }

    fn whitespace(&mut self) {
        while self.current_index < self.source.len()
            && self.source.as_bytes()[self.current_index].is_ascii_whitespace()
        {
            self.current_index += 1;
        }
    }

    fn literal(&mut self, literal: char) -> Result<(), String> {
        if !(self.current_index < self.source.len()
            && self.source.as_bytes()[self.current_index] == literal as u8)
        {
            return Err(String::from("Parsing Error"));
        }

        self.current_index += 1;

        Ok(())
    }

    fn word(&mut self) -> Result<&'css str, String> {
        let mut in_quote = false;
        let start = self.current_index;
        let symbols = [',', '/', '#', '-', '.', '%', '(', ')', '"', '\''];

        while self.current_index < self.source.len() {
            let current_char = self.source.as_bytes()[self.current_index] as char;

            if current_char == '\'' {
                in_quote = !in_quote;
            }

            if current_char.is_ascii_alphanumeric()
                || symbols.contains(&current_char)
                || (in_quote && current_char == ':')
            {
                self.current_index += 1;
            } else {
                break;
            }
        }

        if !(self.current_index > start) {
            Err(String::from("Parsing Error"))
        } else {
            Ok(&self.source[start..self.current_index])
        }
    }

    fn until_chars(&mut self, chars: &[char]) -> &'css str {
        let start = self.current_index;
        while self.current_index < self.source.len()
            && !chars.contains(&(self.source.as_bytes()[self.current_index] as char))
        {
            self.current_index += 1;
        }

        &self.source[start..self.current_index]
    }

    // lower
    fn pair(&mut self, until: &[char]) -> Result<(&'css str, &'css str), String> {
        let prop = self.word()?;
        self.whitespace();
        self.literal(':')?;
        self.whitespace();
        let val = self.until_chars(until);
        Ok((prop, val))
    }

    fn ignor_until(&mut self, chars: &[char]) -> Option<char> {
        while self.current_index < self.source.len() {
            let current_char = self.source.as_bytes()[self.current_index];
            if chars.contains(&(current_char as char)) {
                return Some(current_char as char);
            }
            self.current_index += 1;
        }

        None
    }

    fn body(&mut self) -> Result<HashMap<&'css str, &'css str>, String> {
        let mut pairs = HashMap::new();
        while self.current_index < self.source.len()
            && self.source.as_bytes()[self.current_index] != b'}'
        {
            let mut run_parsing = || -> Result<(), String> {
                let (prop, val) = self.pair(&[';', '}'])?;
                pairs.insert(prop, val);
                self.whitespace();
                self.literal(';')?;
                self.whitespace();
                Ok(())
            };

            if let Err(_) = run_parsing() {
                let why = self.ignor_until(&[';', '}']);
                if why.is_some_and(|char| char == ';') {
                    self.literal(';')?;
                    self.whitespace();
                } else {
                    break;
                }
            }
        }

        Ok(pairs)
    }

    fn selector(&mut self) -> Box<dyn Selector> {
        let mut out: Box<dyn Selector> =
            Box::new(TagSelector::new(self.word().unwrap().to_lowercase()));
        self.whitespace();
        while self.current_index < self.source.len()
            && self.source.as_bytes()[self.current_index] != b'{'
        {
            let tag = self.word().unwrap();
            let descendant = TagSelector::new(tag.to_lowercase());
            out = Box::new(DescendantSeletor::new(out, Box::new(descendant)));
            self.whitespace();
        }
        out
    }

    pub fn parse(&mut self) -> Vec<(Box<dyn Selector>, HashMap<&'css str, &'css str>)> {
        let mut rules = Vec::new();
        while self.current_index < self.source.len() {
            let mut run_pasing = || -> Result<(), String> {
                self.whitespace();
                let selector = self.selector();
                self.literal('{')?;
                self.whitespace();
                let body = self.body()?;
                self.literal('}')?;
                self.whitespace();
                rules.push((selector, body));

                Ok(())
            };

            if let Err(_) = run_pasing() {
                let why = self.ignor_until(&['}']);
                if why.is_some_and(|char| char == '}') {
                    self.literal('}').unwrap();
                    self.whitespace();
                } else {
                    break;
                }
            }
        }

        rules
    }
}

pub trait Selector: fmt::Debug {
    fn matches<'html>(&self, node: Rc<Node<'html>>) -> bool;
    fn priority(&self) -> usize;
}

#[derive(Debug)]
pub struct TagSelector {
    tag: String,
    priority: usize,
}

impl TagSelector {
    fn new(tag: String) -> Self {
        Self { tag, priority: 1 }
    }
}

impl Selector for TagSelector {
    fn matches<'html>(&self, node: Rc<Node<'html>>) -> bool {
        if let NodeData::Element { tag, .. } = node.data {
            self.tag == tag
        } else {
            false
        }
    }

    fn priority(&self) -> usize {
        self.priority
    }
}

#[derive(Debug)]
pub struct DescendantSeletor {
    ancestor: Box<dyn Selector>,
    descendant: Box<dyn Selector>,
    priority: usize,
}

impl DescendantSeletor {
    fn new(ancestor: Box<dyn Selector>, descendant: Box<dyn Selector>) -> Self {
        let priority = ancestor.priority() + descendant.priority();

        Self {
            ancestor,
            descendant,
            priority,
        }
    }
}

impl Selector for DescendantSeletor {
    fn matches<'html>(&self, node: Rc<Node<'html>>) -> bool {
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

pub fn style<'html>(
    node: Rc<Node<'html>>,
    rules: &Vec<(Box<dyn Selector>, HashMap<&'html str, &'html str>)>,
) {
    INHERITED_PROPERTIES
        .iter()
        .for_each(|(property, default_value)| {
            if let Some(ref parent) = node.parent {
                node.style.borrow_mut().insert(
                    property.to_string(),
                    parent
                        .upgrade()
                        .unwrap()
                        .style
                        .borrow()
                        .get(*property)
                        .unwrap()
                        .clone(),
                );
            } else {
                node.style
                    .borrow_mut()
                    .insert(property.to_string(), default_value.to_string());
            }
        });

    rules.iter().for_each(|(selector, body)| {
        if !selector.matches(node.clone()) {
            return;
        }
        body.iter().for_each(|(property, value)| {
            node.style
                .borrow_mut()
                .insert(property.to_string(), value.to_string());
        });
    });

    match node.data {
        NodeData::Element { ref attributes, .. } if attributes.contains_key("style") => {
            let pairs = CssParser::new(attributes.get("style").unwrap())
                .body()
                .unwrap();
            pairs.iter().for_each(|(property, value)| {
                node.style
                    .borrow_mut()
                    .insert(property.to_string(), value.to_string());
            });
        }
        _ => {}
    };

    if node.style.borrow().get("font-size").unwrap().ends_with("%") {
        let parent_font_size;
        if let Some(ref parent) = node.parent {
            parent_font_size = parent
                .upgrade()
                .unwrap()
                .style
                .borrow()
                .get("font-size")
                .unwrap()
                .to_string();
        } else {
            parent_font_size = INHERITED_PROPERTIES.get("font-size").unwrap().to_string();
        }
        let node_style = node.style.borrow();
        let font_size = node_style.get("font-size").unwrap();
        let node_pct = font_size[0..font_size.len() - 1].parse::<f64>().unwrap() / 100.0;
        let parent_px = parent_font_size[0..parent_font_size.len() - 2]
            .parse::<f64>()
            .unwrap();
        node.style.borrow_mut().insert(
            "font-size".to_string(),
            format!("{}px", node_pct * parent_px),
        );
    }

    node.children.borrow().iter().for_each(|child| {
        style(child.clone(), rules);
    });
}
