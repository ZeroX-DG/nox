use dom::dom_ref::NodeRef;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ListOfActiveFormattingElements {
    entries: Vec<Entry>,
}

#[derive(Debug)]
pub enum Entry {
    Element(NodeRef),
    Marker,
}

impl ListOfActiveFormattingElements {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_marker(&mut self) {
        self.entries.push(Entry::Marker);
    }

    pub fn clear_up_to_last_marker(&mut self) {
        while let Some(entry) = self.entries.pop() {
            match entry {
                Entry::Marker => break,
                _ => continue,
            }
        }
    }

    pub fn get_element_after_last_marker(&self, element: &str) -> Option<NodeRef> {
        for entry in self.entries.iter().rev() {
            if let Entry::Marker = entry {
                return None;
            }
            if let Entry::Element(el) = entry {
                if el.borrow().as_element().tag_name() == element {
                    return Some(el.clone());
                }
            }
        }
        None
    }

    pub fn remove_element(&mut self, element: &NodeRef) {
        for (index, entry) in self.entries.iter().rev().enumerate() {
            if let Entry::Element(el) = entry {
                if el == element {
                    self.entries.remove(index);
                    return;
                }
            }
        }
    }

    pub fn contains_node(&self, node: &NodeRef) -> bool {
        for entry in self.entries.iter().rev() {
            if let Entry::Element(el) = entry {
                if el == node {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_index_of_node(&self, node: &NodeRef) -> Option<usize> {
        for (index, entry) in self.entries.iter().rev().enumerate() {
            if let Entry::Element(el) = entry {
                if el == node {
                    return Some(index);
                }
            }
        }
        None
    }
}

impl Deref for ListOfActiveFormattingElements {
    type Target = Vec<Entry>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl DerefMut for ListOfActiveFormattingElements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}
