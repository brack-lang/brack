use std::fmt;

use brack_tokenizer::tokens::{merge_location, Location};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct InnerNode {
    pub id: String,
    pub children: Vec<AST>,
    pub location: Location,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LeafNode {
    pub id: String,
    pub value: Option<String>,
    pub location: Location,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum AST {
    Document(InnerNode),
    Stmt(InnerNode),
    Expr(InnerNode),
    Angle(InnerNode),
    Square(InnerNode),
    Curly(InnerNode),
    Ident(LeafNode),
    Module(LeafNode),
    Text(LeafNode),
    Invalid(LeafNode),
    Ignored(LeafNode),
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display_with_ident(f, 0)
    }
}

impl AST {
    pub fn children(&self) -> &Vec<AST> {
        match self {
            AST::Document(node)
            | AST::Stmt(node)
            | AST::Expr(node)
            | AST::Angle(node)
            | AST::Square(node)
            | AST::Curly(node) => &node.children,
            AST::Ident(_) | AST::Module(_) | AST::Text(_) => {
                panic!("Leaf node has no children")
            }
            AST::Invalid(_) => panic!("This node is broken"),
            AST::Ignored(_) => panic!("This node has to be ignored"),
        }
    }

    pub fn value(&self) -> Option<String> {
        match self {
            AST::Ident(leaf) | AST::Module(leaf) | AST::Text(leaf) => leaf.value.clone(),
            AST::Document(_)
            | AST::Stmt(_)
            | AST::Expr(_)
            | AST::Angle(_)
            | AST::Square(_)
            | AST::Curly(_) => panic!("Inner node has no value"),
            AST::Invalid(_) => panic!("This node is broken"),
            AST::Ignored(_) => panic!("This node has to be ignored"),
        }
    }

    pub fn id(&self) -> String {
        match self {
            AST::Document(node)
            | AST::Stmt(node)
            | AST::Expr(node)
            | AST::Angle(node)
            | AST::Square(node)
            | AST::Curly(node) => node.id.clone(),
            AST::Ident(leaf)
            | AST::Module(leaf)
            | AST::Text(leaf)
            | AST::Invalid(leaf)
            | AST::Ignored(leaf) => leaf.id.clone(),
        }
    }

    pub fn add(&mut self, ast: AST) -> () {
        match self {
            AST::Document(node)
            | AST::Stmt(node)
            | AST::Expr(node)
            | AST::Angle(node)
            | AST::Square(node)
            | AST::Curly(node) => {
                node.children.push(ast.clone());
                let location_children = match ast {
                    AST::Document(inner)
                    | AST::Stmt(inner)
                    | AST::Expr(inner)
                    | AST::Angle(inner)
                    | AST::Square(inner)
                    | AST::Curly(inner) => inner.location,
                    AST::Text(leaf)
                    | AST::Ident(leaf)
                    | AST::Module(leaf)
                    | AST::Invalid(leaf)
                    | AST::Ignored(leaf) => leaf.location,
                };
                node.location = merge_location(&node.location, &location_children);
            }
            AST::Ident(_) | AST::Module(_) | AST::Text(_) => {
                panic!("Cannot add child to leaf node");
            }
            AST::Invalid(_) => panic!("This node is broken"),
            AST::Ignored(_) => panic!("This node has to be ignored"),
        }
    }

    pub fn get(&self, id: &str) -> Option<&AST> {
        match self {
            AST::Document(node)
            | AST::Stmt(node)
            | AST::Expr(node)
            | AST::Angle(node)
            | AST::Square(node)
            | AST::Curly(node) => {
                if node.id == id {
                    return Some(self);
                }
                for child in &node.children {
                    if let Some(ast) = child.get(id) {
                        return Some(ast);
                    }
                }
                None
            }
            AST::Ident(node)
            | AST::Module(node)
            | AST::Text(node)
            | AST::Invalid(node)
            | AST::Ignored(node) => {
                if node.id == id {
                    return Some(self);
                }
                None
            }
        }
    }

    pub fn display_with_ident(&self, f: &mut fmt::Formatter, ident: usize) -> fmt::Result {
        let ident_str = "  ".repeat(ident);
        match self {
            AST::Document(node) => {
                write!(f, "{}Document\n", ident_str)?;
                for child in &node.children {
                    child.display_with_ident(f, ident + 1)?;
                }
                Ok(())
            }
            AST::Stmt(node) => {
                write!(f, "{}Stmt\n", ident_str)?;
                for child in &node.children {
                    child.display_with_ident(f, ident + 1)?;
                }
                Ok(())
            }
            AST::Expr(node) => {
                write!(f, "{}Expr\n", ident_str)?;
                for child in &node.children {
                    child.display_with_ident(f, ident + 1)?;
                }
                Ok(())
            }
            AST::Angle(node) => {
                write!(f, "{}Angle\n", ident_str)?;
                for child in &node.children {
                    child.display_with_ident(f, ident + 1)?;
                }
                Ok(())
            }
            AST::Square(node) => {
                write!(f, "{}Square\n", ident_str)?;
                for child in &node.children {
                    child.display_with_ident(f, ident + 1)?;
                }
                Ok(())
            }
            AST::Curly(node) => {
                write!(f, "{}Curly\n", ident_str)?;
                for child in &node.children {
                    child.display_with_ident(f, ident + 1)?;
                }
                Ok(())
            }
            AST::Ident(node) => write!(f, "{}Ident: {}\n", ident_str, node.value.as_ref().unwrap()),
            AST::Module(node) => {
                write!(f, "{}Module: {}\n", ident_str, node.value.as_ref().unwrap())
            }
            AST::Text(node) => write!(f, "{}Text: {}\n", ident_str, node.value.as_ref().unwrap()),
            AST::Invalid(_) => write!(f, "{}Invalid\n", ident_str),
            AST::Ignored(_) => write!(f, "{}Ignored\n", ident_str),
        }
    }
}

pub fn new_document(children: Vec<AST>, location: Location) -> AST {
    AST::Document(InnerNode {
        id: Uuid::new_v4().to_string(),
        children,
        location,
    })
}

pub fn new_stmt(children: Vec<AST>, location: Location) -> AST {
    AST::Stmt(InnerNode {
        id: Uuid::new_v4().to_string(),
        children,
        location,
    })
}

pub fn new_expr(children: Vec<AST>, location: Location) -> AST {
    AST::Expr(InnerNode {
        id: Uuid::new_v4().to_string(),
        children,
        location,
    })
}

pub fn new_angle(children: Vec<AST>, location: Location) -> AST {
    AST::Angle(InnerNode {
        id: Uuid::new_v4().to_string(),
        children,
        location,
    })
}

pub fn new_square(children: Vec<AST>, location: Location) -> AST {
    AST::Square(InnerNode {
        id: Uuid::new_v4().to_string(),
        children,
        location,
    })
}

pub fn new_curly(children: Vec<AST>, location: Location) -> AST {
    AST::Curly(InnerNode {
        id: Uuid::new_v4().to_string(),
        children,
        location,
    })
}

pub fn new_ident(value: Option<String>, location: Location) -> AST {
    AST::Ident(LeafNode {
        id: Uuid::new_v4().to_string(),
        value,
        location,
    })
}

pub fn new_module(value: Option<String>, location: Location) -> AST {
    AST::Module(LeafNode {
        id: Uuid::new_v4().to_string(),
        value,
        location,
    })
}

pub fn new_text(value: Option<String>, location: Location) -> AST {
    AST::Text(LeafNode {
        id: Uuid::new_v4().to_string(),
        value,
        location,
    })
}

pub fn new_invalid(location: Location) -> AST {
    AST::Invalid(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: None,
        location,
    })
}

fn assert_inner_node_eq(node1: &InnerNode, node2: &InnerNode) {
    assert_eq!(node1.children.len(), node2.children.len());
    for i in 0..node1.children.len() {
        assert_ast_eq(&node1.children[i], &node2.children[i]);
    }
}

fn assert_leaf_node_eq(node1: &LeafNode, node2: &LeafNode) {
    assert_eq!(node1.value, node2.value);
}

pub fn assert_ast_eq(node1: &AST, node2: &AST) {
    match (node1, node2) {
        (AST::Document(inner1), AST::Document(inner2)) => assert_inner_node_eq(inner1, inner2),
        (AST::Stmt(inner1), AST::Stmt(inner2)) => assert_inner_node_eq(inner1, inner2),
        (AST::Expr(inner1), AST::Expr(inner2)) => assert_inner_node_eq(inner1, inner2),
        (AST::Angle(inner1), AST::Angle(inner2)) => assert_inner_node_eq(inner1, inner2),
        (AST::Square(inner1), AST::Square(inner2)) => assert_inner_node_eq(inner1, inner2),
        (AST::Curly(inner1), AST::Curly(inner2)) => assert_inner_node_eq(inner1, inner2),
        (AST::Ident(leaf1), AST::Ident(leaf2)) => assert_leaf_node_eq(leaf1, leaf2),
        (AST::Module(leaf1), AST::Module(leaf2)) => assert_leaf_node_eq(leaf1, leaf2),
        (AST::Text(leaf1), AST::Text(leaf2)) => assert_leaf_node_eq(leaf1, leaf2),
        _ => panic!(
            "Mismatched AST node types or unexpected AST node\nleft: {:?}\nright: {:?}",
            node1, node2
        ),
    }
}

pub fn matches_kind(node1: &AST, node2: &AST) -> bool {
    match (node1, node2) {
        (AST::Document(_), AST::Document(_)) => true,
        (AST::Stmt(_), AST::Stmt(_)) => true,
        (AST::Expr(_), AST::Expr(_)) => true,
        (AST::Angle(_), AST::Angle(_)) => true,
        (AST::Curly(_), AST::Curly(_)) => true,
        (AST::Square(_), AST::Square(_)) => true,
        (AST::Module(_), AST::Module(_)) => true,
        (AST::Ident(_), AST::Ident(_)) => true,
        (AST::Text(_), AST::Text(_)) => true,
        (AST::Invalid(_), AST::Invalid(_)) => true,
        (AST::Ignored(_), AST::Ignored(_)) => true,
        _ => false,
    }
}
