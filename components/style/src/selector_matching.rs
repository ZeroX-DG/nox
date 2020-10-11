use dom::element::Element;
use dom::dom_ref::NodeRef;
use css::selector::structs::*;

fn get_parent(el: &NodeRef) -> Option<NodeRef> {
    el.borrow().as_node().parent()
}

fn get_prev_sibling(el: &NodeRef) -> Option<NodeRef> {
    el.borrow().as_node().prev_sibling()
}

pub fn is_match_selectors(element: &NodeRef, selectors: &Vec<Selector>) -> bool {
    selectors.iter().any(|selector| is_match_selector(element.clone(), selector))
}

pub fn is_match_selector(element: NodeRef, selector: &Selector) -> bool {
    let mut current_element = Some(element);
    for (selector_seq, combinator) in selector.values().iter().rev() {
        if let Some(el) = current_element.clone() {
            match combinator {
                Some(Combinator::Child) => {
                    let parent = get_parent(&el);
                    if let Some(p) = &parent {
                        if !is_match_simple_selector_seq(p, selector_seq) {
                            return false
                        }
                    }
                    current_element = parent;
                }
                Some(Combinator::Descendant) => {
                    loop {
                        let parent = get_parent(&el);
                        if let Some(p) = &parent {
                            if is_match_simple_selector_seq(p, selector_seq) {
                                current_element = parent;
                                break
                            }
                        } else {
                            return false;
                        }
                    }
                }
                Some(Combinator::NextSibling) => {
                    let sibling = get_prev_sibling(&el);
                    if let Some(sibling) = &sibling {
                        if !is_match_simple_selector_seq(sibling, selector_seq) {
                            return false
                        }
                    }
                    current_element = sibling;
                }
                Some(Combinator::SubsequentSibling) => {
                    loop {
                        let sibling = get_prev_sibling(&el);
                        if let Some(s) = &sibling {
                            if is_match_simple_selector_seq(s, selector_seq) {
                                current_element = sibling;
                                break
                            }
                        } else {
                            return false;
                        }
                    }
                }
                None => {
                    if !is_match_simple_selector_seq(&el, selector_seq) {
                        return false
                    }
                }
            }
        } else {
            return false
        }
    }
    true
}

fn is_match_simple_selector_seq(element: &NodeRef, sequence: &SimpleSelectorSequence) -> bool {
    let element = element.borrow();
    let element = element.as_element().expect("Node is not an element");
    sequence.values().iter().all(|selector| is_match_simple_selector(element, selector))
}

fn is_match_simple_selector(element: &Element, selector: &SimpleSelector) -> bool {
    match selector.selector_type() {
        SimpleSelectorType::Universal => true,
        SimpleSelectorType::Type => {
            if let Some(type_name) = selector.value() {
                return element.tag_name() == *type_name;
            }
            false
        }
        SimpleSelectorType::Class => {
            if let Some(type_name) = selector.value() {
                return element.class_list().contains(&type_name);
            }
            false
        }
        SimpleSelectorType::ID => {
            if let Some(id) = selector.value() {
                return element.id() == id;
            }
            false
        }
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use css::parser::Parser;
    use css::tokenizer::Tokenizer;
    use css::cssom::css_rule::CSSRule;
    use dom::node::Node;

    #[test]
    fn match_simple_type() {
        let element = NodeRef::new(Element::new("h1".to_string()));
        let css = "h1 { color: red; }";

        let tokenizer = Tokenizer::new(css.to_string());
        let tokens = tokenizer.run();
        let mut parser = Parser::new(tokens);
        let stylesheet = parser.parse_a_css_stylesheet();

        let rule = stylesheet.first().unwrap();

        match rule {
            CSSRule::Style(style) => {
                let selectors = &style.selectors;
                assert!(is_match_selectors(&element, selectors));
            }
        }
    }

    #[test]
    fn match_simple_id() {
        let mut element = Element::new("h1".to_string());
        element.set_attribute("id", "button");
        let element = NodeRef::new(element);
        let css = "h1#button { color: red; }";

        let tokenizer = Tokenizer::new(css.to_string());
        let tokens = tokenizer.run();
        let mut parser = Parser::new(tokens);
        let stylesheet = parser.parse_a_css_stylesheet();

        let rule = stylesheet.first().unwrap();

        match rule {
            CSSRule::Style(style) => {
                let selectors = &style.selectors;
                assert!(is_match_selectors(&element, selectors));
            }
        }
    }

    #[test]
    fn match_simple_decendant() {
        let parent = NodeRef::new(Element::new("h1".to_string()));
        let child = NodeRef::new(Element::new("button".to_string()));
        Node::append_child(parent.clone(), child.clone());

        let css = "h1 button { color: red; }";

        let tokenizer = Tokenizer::new(css.to_string());
        let tokens = tokenizer.run();
        let mut parser = Parser::new(tokens);
        let stylesheet = parser.parse_a_css_stylesheet();

        let rule = stylesheet.first().unwrap();

        match rule {
            CSSRule::Style(style) => {
                let selectors = &style.selectors;
                assert!(is_match_selectors(&child, selectors));
            }
        }
    }

    #[test]
    fn match_simple_child() {
        let parent = NodeRef::new(Element::new("h1".to_string()));
        let child = NodeRef::new(Element::new("button".to_string()));
        Node::append_child(parent.clone(), child.clone());
        
        let css = "h1 > button { color: red; }";

        let tokenizer = Tokenizer::new(css.to_string());
        let tokens = tokenizer.run();
        let mut parser = Parser::new(tokens);
        let stylesheet = parser.parse_a_css_stylesheet();

        let rule = stylesheet.first().unwrap();

        match rule {
            CSSRule::Style(style) => {
                let selectors = &style.selectors;
                assert!(is_match_selectors(&child, selectors));
            }
        }
    }

    #[test]
    fn match_invalid_child() {
        let parent = NodeRef::new(Element::new("h1".to_string()));
        let child = NodeRef::new(Element::new("button".to_string()));
        Node::append_child(parent.clone(), child.clone());
        
        let css = "button > h1 { color: red; }";

        let tokenizer = Tokenizer::new(css.to_string());
        let tokens = tokenizer.run();
        let mut parser = Parser::new(tokens);
        let stylesheet = parser.parse_a_css_stylesheet();

        let rule = stylesheet.first().unwrap();

        match rule {
            CSSRule::Style(style) => {
                let selectors = &style.selectors;
                assert!(!is_match_selectors(&child, selectors));
            }
        }
    }

    #[test]
    fn match_invalid_id() {
        let parent = NodeRef::new(Element::new("h1".to_string()));
        let child = NodeRef::new(Element::new("button".to_string()));
        Node::append_child(parent.clone(), child.clone());
        
        let css = "h1#name > button { color: red; }";

        let tokenizer = Tokenizer::new(css.to_string());
        let tokens = tokenizer.run();
        let mut parser = Parser::new(tokens);
        let stylesheet = parser.parse_a_css_stylesheet();

        let rule = stylesheet.first().unwrap();

        match rule {
            CSSRule::Style(style) => {
                let selectors = &style.selectors;
                assert!(!is_match_selectors(&child, selectors));
            }
        }
    }
}
