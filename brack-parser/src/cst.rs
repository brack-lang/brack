use std::fmt::{self};

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
    Angle(InnerNode),
    Curly(InnerNode),
    Square(InnerNode),
    BackSlash(InnerNode),
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
    Dot(LeafNode),
    Comma(LeafNode),
    EOF(LeafNode),
}

impl fmt::Display for CST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_with_indent(f, 0)
    }
}

impl CST {
    pub fn children(&self) -> &Vec<CST> {
        match self {
            CST::Document(node)
            | CST::Stmt(node)
            | CST::Expr(node)
            | CST::Angle(node)
            | CST::Curly(node)
            | CST::Square(node) => &node.children,
            _ => panic!("This node does not have children"),
        }
    }

    pub fn location(&self) -> Location {
        match self {
            CST::Document(node)
            | CST::Stmt(node)
            | CST::Expr(node)
            | CST::BackSlash(node)
            | CST::Angle(node)
            | CST::Curly(node)
            | CST::Square(node) => node.location.clone(),
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
            | CST::Dot(leaf)
            | CST::Comma(leaf)
            | CST::EOF(leaf) => leaf.location.clone(),
        }
    }

    pub fn set_location(&mut self, location: Location) -> () {
        match self {
            CST::Document(node)
            | CST::Stmt(node)
            | CST::Expr(node)
            | CST::BackSlash(node)
            | CST::Angle(node)
            | CST::Curly(node)
            | CST::Square(node) => {
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
            | CST::Dot(leaf)
            | CST::Comma(leaf)
            | CST::EOF(leaf) => leaf.value.clone(),
            _ => panic!("This node does not have a value"),
        }
    }

    pub fn add(&mut self, cst: CST) -> () {
        match self {
            CST::Document(node)
            | CST::Stmt(node)
            | CST::Expr(node)
            | CST::BackSlash(node)
            | CST::Angle(node)
            | CST::Square(node)
            | CST::Curly(node) => {
                node.children.push(cst.clone());
                let location_children = match cst {
                    CST::Document(node)
                    | CST::Stmt(node)
                    | CST::Expr(node)
                    | CST::BackSlash(node)
                    | CST::Angle(node)
                    | CST::Curly(node)
                    | CST::Square(node) => node.location,
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
                    | CST::Dot(leaf)
                    | CST::Comma(leaf)
                    | CST::EOF(leaf) => leaf.location,
                };
                node.location = merge_location(&node.location, &location_children);
            }
            _ => panic!("This node does not have children"),
        }
    }

    fn display_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let indent_str = "  ".repeat(indent);
        match self {
            CST::Document(inner) => {
                writeln!(f, "{}Document(id: {}) [", indent_str, inner.id)?;
                for child in &inner.children {
                    child.display_with_indent(f, indent + 1)?;
                    writeln!(f, ",")?;
                }
                write!(f, "{}]", indent_str)
            }
            CST::Stmt(inner) => {
                writeln!(f, "{}Stmt(id: {}) [", indent_str, inner.id)?;
                for child in &inner.children {
                    child.display_with_indent(f, indent + 1)?;
                    writeln!(f, ",")?;
                }
                write!(f, "{}]", indent_str)
            }
            CST::Expr(inner) => {
                writeln!(f, "{}Expr(id: {}) [", indent_str, inner.id)?;
                for child in &inner.children {
                    child.display_with_indent(f, indent + 1)?;
                    writeln!(f, ",")?;
                }
                write!(f, "{}]", indent_str)
            }
            CST::Angle(inner) => {
                writeln!(f, "{}Angle(id: {}) [", indent_str, inner.id)?;
                for child in &inner.children {
                    child.display_with_indent(f, indent + 1)?;
                    writeln!(f, ",")?;
                }
                write!(f, "{}]", indent_str)
            }
            CST::Curly(inner) => {
                writeln!(f, "{}Curly(id: {}) [", indent_str, inner.id)?;
                for child in &inner.children {
                    child.display_with_indent(f, indent + 1)?;
                    writeln!(f, ",")?;
                }
                write!(f, "{}]", indent_str)
            }
            CST::Square(inner) => {
                writeln!(f, "{}Square(id: {}) [", indent_str, inner.id)?;
                for child in &inner.children {
                    child.display_with_indent(f, indent + 1)?;
                    writeln!(f, ",")?;
                }
                write!(f, "{}]", indent_str)
            }
            CST::AngleBracketOpen(leaf) => {
                write!(f, "{}AngleBracketOpen(id: {})", indent_str, leaf.id)
            }
            CST::AngleBracketClose(leaf) => {
                write!(f, "{}AngleBracketClose(id: {})", indent_str, leaf.id)
            }
            CST::SquareBracketOpen(leaf) => {
                write!(f, "{}SquareBracketOpen(id: {})", indent_str, leaf.id)
            }
            CST::SquareBracketClose(leaf) => {
                write!(f, "{}SquareBracketClose(id: {})", indent_str, leaf.id)
            }
            CST::CurlyBracketOpen(leaf) => {
                write!(f, "{}CurlyBracketOpen(id: {})", indent_str, leaf.id)
            }
            CST::CurlyBracketClose(leaf) => {
                write!(f, "{}CurlyBracketClose(id: {})", indent_str, leaf.id)
            }
            CST::Module(leaf) => write!(
                f,
                "{}Module(id: {}, value: {:?})",
                indent_str, leaf.id, leaf.value
            ),
            CST::Ident(leaf) => write!(
                f,
                "{}Ident(id: {}, value: {:?})",
                indent_str, leaf.id, leaf.value
            ),
            CST::Text(leaf) => write!(
                f,
                "{}Text(id: {}, value: {:?})",
                indent_str, leaf.id, leaf.value
            ),
            CST::Whitespace(leaf) => write!(f, "{}Whitespace(id: {})", indent_str, leaf.id),
            CST::Newline(leaf) => write!(f, "{}Newline(id: {})", indent_str, leaf.id),
            CST::BackSlash(leaf) => write!(f, "{}BackSlash(id: {})", indent_str, leaf.id),
            CST::Dot(leaf) => write!(f, "{}Dot(id: {})", indent_str, leaf.id),
            CST::Comma(leaf) => write!(f, "{}Comma(id: {})", indent_str, leaf.id),
            CST::EOF(leaf) => write!(f, "{}EOF(id: {})", indent_str, leaf.id),
        }
    }
}

pub fn matches_kind(cst: &CST, kind: &CST) -> bool {
    match (cst, kind) {
        (CST::Document(_), CST::Document(_))
        | (CST::Stmt(_), CST::Stmt(_))
        | (CST::Expr(_), CST::Expr(_))
        | (CST::Angle(_), CST::Angle(_))
        | (CST::Curly(_), CST::Curly(_))
        | (CST::Square(_), CST::Square(_))
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
    CST::BackSlash(InnerNode {
        id: Uuid::new_v4().to_string(),
        children: vec![],
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

pub fn new_angle() -> CST {
    CST::Angle(InnerNode {
        id: Uuid::new_v4().to_string(),
        children: vec![],
        location: mock_location(),
    })
}

pub fn new_curly() -> CST {
    CST::Curly(InnerNode {
        id: Uuid::new_v4().to_string(),
        children: vec![],
        location: mock_location(),
    })
}

pub fn new_square() -> CST {
    CST::Square(InnerNode {
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
