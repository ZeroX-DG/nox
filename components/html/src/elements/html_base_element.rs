use super::HTMLElement;
use dom::dom_ref::DOMObject;
use dom::node::Node;
use std::any::Any;

pub struct HTMLBaseElement {
    html_element: HTMLElement,
    href: String,
    target: String,
}

impl HTMLBaseElement {
    pub fn new(html_element: HTMLElement) -> Self {
        Self {
            html_element,
            href: String::new(),
            target: String::new()
        }
    }
}

impl DOMObject for HTMLBaseElement {
    fn as_node(&self) -> &Node {
        self.html_element.as_node()
    }

    fn as_node_mut(&mut self) -> &mut Node {
        self.html_element.as_node_mut()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
