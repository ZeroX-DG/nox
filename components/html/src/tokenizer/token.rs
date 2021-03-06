#[derive(Clone, Debug, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
    pub prefix: String,
    pub namespace: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    DOCTYPE {
        name: Option<String>,
        public_identifier: Option<String>,
        system_identifier: Option<String>,
        force_quirks: bool,
    },
    Tag {
        tag_name: String,
        self_closing: bool,
        attributes: Vec<Attribute>,
        is_end_tag: bool,
        self_closing_acknowledged: bool,
    },
    Comment(String),
    Character(char),
    EOF,
}

impl Token {
    pub fn new_start_tag() -> Self {
        Token::Tag {
            tag_name: String::new(),
            is_end_tag: false,
            self_closing: false,
            attributes: Vec::new(),
            self_closing_acknowledged: false,
        }
    }

    pub fn new_end_tag() -> Self {
        Token::Tag {
            tag_name: String::new(),
            is_end_tag: true,
            self_closing: false,
            attributes: Vec::new(),
            self_closing_acknowledged: false,
        }
    }

    pub fn new_start_tag_with_name(name: &str) -> Self {
        Token::Tag {
            tag_name: name.to_owned(),
            is_end_tag: false,
            self_closing: false,
            attributes: Vec::new(),
            self_closing_acknowledged: false,
        }
    }

    pub fn new_comment(data: &str) -> Self {
        Token::Comment(data.to_owned())
    }

    pub fn new_doctype() -> Self {
        Token::DOCTYPE {
            name: None,
            public_identifier: None,
            system_identifier: None,
            force_quirks: false,
        }
    }

    pub fn set_force_quirks(&mut self, value: bool) {
        if let Token::DOCTYPE {
            ref mut force_quirks,
            ..
        } = self
        {
            *force_quirks = value;
        }
    }

    pub fn is_self_closing(&self) -> bool {
        if let Token::Tag { self_closing, .. } = self {
            return *self_closing;
        }
        return false;
    }

    pub fn is_start_tag(&self) -> bool {
        if let Token::Tag { is_end_tag, .. } = self {
            return !*is_end_tag;
        }
        return false;
    }

    pub fn is_end_tag(&self) -> bool {
        if let Token::Tag { is_end_tag, .. } = self {
            return *is_end_tag;
        }
        return false;
    }

    pub fn tag_name(&self) -> &String {
        if let Token::Tag { tag_name, .. } = self {
            return tag_name;
        }
        panic!("Token is not a tag");
    }

    pub fn set_tag_name(&mut self, new_name: &str) {
        if let Token::Tag {
            ref mut tag_name, ..
        } = self
        {
            *tag_name = new_name.to_owned();
        }
        panic!("Token is not a tag");
    }

    pub fn is_eof(&self) -> bool {
        if let Token::EOF = self {
            return true;
        }
        return false;
    }

    pub fn attributes(&self) -> &Vec<Attribute> {
        if let Token::Tag { attributes, .. } = self {
            return attributes;
        }
        panic!("Token is not a tag");
    }

    pub fn attributes_mut(&mut self) -> &mut Vec<Attribute> {
        if let Token::Tag {
            ref mut attributes, ..
        } = self
        {
            return attributes;
        }
        panic!("Token is not a tag");
    }

    pub fn attribute(&self, name: &str) -> Option<&String> {
        if let Token::Tag { attributes, .. } = self {
            return match attributes.iter().find(|attr| attr.name == name) {
                Some(attr) => Some(&attr.name),
                _ => None,
            };
        }
        panic!("Token is not a tag");
    }

    pub fn drop_attributes(&mut self) {
        if let Token::Tag {
            ref mut attributes, ..
        } = self
        {
            *attributes = Vec::new();
        }
        panic!("Token is not a tag");
    }

    pub fn acknowledge_self_closing_if_set(&mut self) {
        if let Token::Tag {
            ref mut self_closing_acknowledged,
            self_closing,
            ..
        } = self
        {
            if *self_closing {
                *self_closing_acknowledged = true;
            }
        }
    }
}

impl Attribute {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
            prefix: String::new(),
            namespace: String::new(),
        }
    }

    pub fn from_name_value(name: String, value: String) -> Self {
        Self {
            name,
            value,
            prefix: String::new(),
            namespace: String::new(),
        }
    }
}
