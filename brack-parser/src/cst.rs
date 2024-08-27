use brack_tokenizer::tokens::{merge_location, mock_location, Location};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct InnerNode {
    pub id: String,
    pub children: Vec<CST>,
    pub location: Location,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LeafNode {
    pub id: String,
    pub value: Option<String>,
    pub location: Location,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum CST {
    Document(InnerNode),
    Stmt(InnerNode),
    Expr(InnerNode),
    Bracket(InnerNode),
    AngleBracketOpen(LeafNode),
    AngleBracketClose(LeafNode),
    SquareBracketOpen(LeafNode),
    SquareBracketClose(LeafNode),
    CurlyBracketOpen(LeafNode),
    CurlyBracketClose(LeafNode),
    Module(LeafNode),
    Ident(LeafNode),
    Text(LeafNode),
    Whitespace(LeafNode),
    Newline(LeafNode),
    BackSlash(LeafNode),
    Dot(LeafNode),
    Comma(LeafNode),
    EOF(LeafNode),
}

impl CST {
    pub fn children(&self) -> &Vec<CST> {
        match self {
            CST::Document(node) | CST::Stmt(node) | CST::Expr(node) | CST::Bracket(node) => {
                &node.children
            }
            _ => panic!("This node does not have children"),
        }
    }

    pub fn location(&self) -> Location {
        match self {
            CST::Document(node) | CST::Stmt(node) | CST::Expr(node) | CST::Bracket(node) => {
                node.location.clone()
            }
            CST::AngleBracketOpen(leaf)
            | CST::AngleBracketClose(leaf)
            | CST::CurlyBracketOpen(leaf)
            | CST::CurlyBracketClose(leaf)
            | CST::SquareBracketOpen(leaf)
            | CST::SquareBracketClose(leaf)
            | CST::Module(leaf)
            | CST::Ident(leaf)
            | CST::Text(leaf)
            | CST::Whitespace(leaf)
            | CST::Newline(leaf)
            | CST::BackSlash(leaf)
            | CST::Dot(leaf)
            | CST::Comma(leaf)
            | CST::EOF(leaf) => leaf.location.clone(),
        }
    }

    pub fn set_location(&mut self, location: Location) -> () {
        match self {
            CST::Document(node) | CST::Stmt(node) | CST::Expr(node) | CST::Bracket(node) => {
                node.location = location;
            }
            CST::AngleBracketOpen(leaf)
            | CST::AngleBracketClose(leaf)
            | CST::CurlyBracketOpen(leaf)
            | CST::CurlyBracketClose(leaf)
            | CST::SquareBracketOpen(leaf)
            | CST::SquareBracketClose(leaf)
            | CST::Module(leaf)
            | CST::Ident(leaf)
            | CST::Text(leaf)
            | CST::Whitespace(leaf)
            | CST::Newline(leaf)
            | CST::BackSlash(leaf)
            | CST::Dot(leaf)
            | CST::Comma(leaf)
            | CST::EOF(leaf) => leaf.location = location,
        }
    }

    pub fn value(&self) -> Option<String> {
        match self {
            CST::AngleBracketOpen(leaf)
            | CST::AngleBracketClose(leaf)
            | CST::CurlyBracketOpen(leaf)
            | CST::CurlyBracketClose(leaf)
            | CST::SquareBracketOpen(leaf)
            | CST::SquareBracketClose(leaf)
            | CST::Module(leaf)
            | CST::Ident(leaf)
            | CST::Text(leaf)
            | CST::Whitespace(leaf)
            | CST::Newline(leaf)
            | CST::BackSlash(leaf)
            | CST::Dot(leaf)
            | CST::Comma(leaf)
            | CST::EOF(leaf) => leaf.value.clone(),
            _ => panic!("This node does not have a value"),
        }
    }

    pub fn add(&mut self, cst: CST) -> () {
        match self {
            CST::Document(node) | CST::Stmt(node) | CST::Expr(node) | CST::Bracket(node) => {
                node.children.push(cst.clone());
                let location_children = match cst {
                    CST::Document(node)
                    | CST::Stmt(node)
                    | CST::Expr(node)
                    | CST::Bracket(node) => node.location,
                    CST::AngleBracketOpen(leaf)
                    | CST::AngleBracketClose(leaf)
                    | CST::CurlyBracketOpen(leaf)
                    | CST::CurlyBracketClose(leaf)
                    | CST::SquareBracketOpen(leaf)
                    | CST::SquareBracketClose(leaf)
                    | CST::Module(leaf)
                    | CST::Ident(leaf)
                    | CST::Text(leaf)
                    | CST::Whitespace(leaf)
                    | CST::Newline(leaf)
                    | CST::BackSlash(leaf)
                    | CST::Dot(leaf)
                    | CST::Comma(leaf)
                    | CST::EOF(leaf) => leaf.location,
                };
                node.location = merge_location(&node.location, &location_children);
            }
            _ => panic!("This node does not have children"),
        }
    }
}

pub fn matches_kind(cst: &CST, kind: &CST) -> bool {
    match (cst, kind) {
        (CST::Document(_), CST::Document(_))
        | (CST::Stmt(_), CST::Stmt(_))
        | (CST::Expr(_), CST::Expr(_))
        | (CST::Bracket(_), CST::Bracket(_))
        | (CST::AngleBracketOpen(_), CST::AngleBracketOpen(_))
        | (CST::AngleBracketClose(_), CST::AngleBracketClose(_))
        | (CST::SquareBracketOpen(_), CST::SquareBracketOpen(_))
        | (CST::SquareBracketClose(_), CST::SquareBracketClose(_))
        | (CST::CurlyBracketOpen(_), CST::CurlyBracketOpen(_))
        | (CST::CurlyBracketClose(_), CST::CurlyBracketClose(_))
        | (CST::Module(_), CST::Module(_))
        | (CST::Ident(_), CST::Ident(_))
        | (CST::Text(_), CST::Text(_))
        | (CST::Whitespace(_), CST::Whitespace(_))
        | (CST::Newline(_), CST::Newline(_))
        | (CST::BackSlash(_), CST::BackSlash(_))
        | (CST::Dot(_), CST::Dot(_))
        | (CST::Comma(_), CST::Comma(_))
        | (CST::EOF(_), CST::EOF(_)) => true,
        _ => false,
    }
}

pub fn new_document() -> CST {
    CST::Document(InnerNode {
        id: Uuid::new_v4().to_string(),
        children: vec![],
        location: mock_location(),
    })
}

pub fn new_stmt() -> CST {
    CST::Stmt(InnerNode {
        id: Uuid::new_v4().to_string(),
        children: vec![],
        location: mock_location(),
    })
}

pub fn new_expr() -> CST {
    CST::Expr(InnerNode {
        id: Uuid::new_v4().to_string(),
        children: vec![],
        location: mock_location(),
    })
}

pub fn new_eof(location: Location) -> CST {
    CST::EOF(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: None,
        location,
    })
}

pub fn new_whitespace(location: Location) -> CST {
    CST::Whitespace(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: None,
        location,
    })
}

pub fn new_newline(location: Location) -> CST {
    CST::Newline(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: None,
        location,
    })
}

pub fn new_backslash(location: Location) -> CST {
    CST::BackSlash(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: None,
        location,
    })
}

pub fn new_text(value: String, location: Location) -> CST {
    CST::Text(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: Some(value),
        location,
    })
}

pub fn new_ident(value: String, location: Location) -> CST {
    CST::Ident(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: Some(value),
        location,
    })
}

pub fn new_module(value: String, location: Location) -> CST {
    CST::Module(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: Some(value),
        location,
    })
}

pub fn new_bracket() -> CST {
    CST::Bracket(InnerNode {
        id: Uuid::new_v4().to_string(),
        children: vec![],
        location: mock_location(),
    })
}

pub fn new_comma(location: Location) -> CST {
    CST::Comma(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: None,
        location,
    })
}

pub fn new_dot(location: Location) -> CST {
    CST::Dot(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: None,
        location,
    })
}

pub fn new_angle_bracket_open(location: Location) -> CST {
    CST::AngleBracketOpen(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: None,
        location,
    })
}

pub fn new_angle_bracket_close(location: Location) -> CST {
    CST::AngleBracketClose(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: None,
        location,
    })
}

pub fn new_square_bracket_open(location: Location) -> CST {
    CST::SquareBracketOpen(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: None,
        location,
    })
}

pub fn new_square_bracket_close(location: Location) -> CST {
    CST::SquareBracketClose(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: None,
        location,
    })
}

pub fn new_curly_bracket_open(location: Location) -> CST {
    CST::CurlyBracketOpen(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: None,
        location,
    })
}

pub fn new_curly_bracket_close(location: Location) -> CST {
    CST::CurlyBracketClose(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: None,
        location,
    })
}
