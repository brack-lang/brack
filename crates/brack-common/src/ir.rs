use std::collections::HashMap;

pub struct Html {
    pub tag: HtmlTag,
    pub attributes: HashMap<HtmlAttribute, Vec<String>>,
    pub children: Vec<Html>,
}

pub enum HtmlTag {
    Fragment,
    Div,
    Span,
    P,
}

pub enum HtmlAttribute {
    Class,
    Id,
    Style,
}
