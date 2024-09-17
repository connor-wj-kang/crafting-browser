// use crate::html_parser1::Node;
// use std::{cell::RefCell, rc::Rc};

// struct DocumentLayout<'body> {
//     node: Rc<RefCell<Node<'body>>>,
//     children: Vec<Rc<RefCell<BlockLayout<'body>>>>,
// }

// impl<'body> DocumentLayout<'body> {
//     fn layout(&self) {}
// }

// struct BlockLayout<'body> {
//     node: Rc<RefCell<Node<'body>>>,
//     children: Vec<Rc<RefCell<BlockLayout<'body>>>>,
//     parent: Rc<RefCell<BlockLayout<'body>>>,
//     previous: Option<Rc<RefCell<BlockLayout<'body>>>>,
// }

// impl<'body> BlockLayout<'body> {
//     fn new(
//         node: Rc<RefCell<Node<'body>>>,
//         parent: Rc<RefCell<BlockLayout<'body>>>,
//         previous: Option<Rc<RefCell<BlockLayout<'body>>>>,
//     ) -> Self {
//         Self {
//             node,
//             parent,
//             previous,
//             children: Vec::new(),
//         }
//     }

//     fn layout_intermediate(self: Rc<RefCell<Self>>) {
//         let mut previous = None;
//         if let Node::Element(ref element) = *self.node.borrow() {
//             for child in element.children.iter() {
//                 let next = Self::new(child.clone(), Rc::new(RefCell::new(self)), previous.clone());
//             }
//         }
//     }
// }
