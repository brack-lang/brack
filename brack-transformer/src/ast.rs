use brack_tokenizer::tokens::{merge_location, mock_location, Location};
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
}

fn merge_all_locations(asts: &Vec<AST>) -> Location {
    let mut location = mock_location();
    for ast in asts {
        match ast {
            AST::Document(inner)
            | AST::Stmt(inner)
            | AST::Expr(inner)
            | AST::Angle(inner)
            | AST::Square(inner)
            | AST::Curly(inner) => {
                location = merge_location(&location, &inner.location);
            }
            AST::Ident(leaf)
            | AST::Module(leaf)
            | AST::Text(leaf)
            | AST::Invalid(leaf)
            | AST::Ignored(leaf) => {
                location = merge_location(&location, &leaf.location);
            }
        }
    }
    location
}

pub fn new_document() -> AST {
    AST::Document(InnerNode {
        id: Uuid::new_v4().to_string(),
        children: vec![],
        location: mock_location(),
    })
}

pub fn new_document_with_children(children: Vec<AST>) -> AST {
    let location = merge_all_locations(&children);
    AST::Document(InnerNode {
        id: Uuid::new_v4().to_string(),
        children,
        location,
    })
}

pub fn new_stmt() -> AST {
    AST::Stmt(InnerNode {
        id: Uuid::new_v4().to_string(),
        children: vec![],
        location: mock_location(),
    })
}

pub fn new_stmt_with_children(children: Vec<AST>) -> AST {
    let location = merge_all_locations(&children);
    AST::Stmt(InnerNode {
        id: Uuid::new_v4().to_string(),
        children,
        location,
    })
}

pub fn new_expr() -> AST {
    AST::Expr(InnerNode {
        id: Uuid::new_v4().to_string(),
        children: vec![],
        location: mock_location(),
    })
}

pub fn new_expr_with_children(children: Vec<AST>) -> AST {
    let location = merge_all_locations(&children);
    AST::Expr(InnerNode {
        id: Uuid::new_v4().to_string(),
        children,
        location,
    })
}

pub fn new_angle() -> AST {
    AST::Angle(InnerNode {
        id: Uuid::new_v4().to_string(),
        children: vec![],
        location: mock_location(),
    })
}

pub fn new_angle_with_children(children: Vec<AST>) -> AST {
    let location = merge_all_locations(&children);
    AST::Angle(InnerNode {
        id: Uuid::new_v4().to_string(),
        children,
        location,
    })
}

pub fn new_curly() -> AST {
    AST::Curly(InnerNode {
        id: Uuid::new_v4().to_string(),
        children: vec![],
        location: mock_location(),
    })
}

pub fn new_curly_with_children(children: Vec<AST>) -> AST {
    let location = merge_all_locations(&children);
    AST::Curly(InnerNode {
        id: Uuid::new_v4().to_string(),
        children,
        location,
    })
}

pub fn new_square() -> AST {
    AST::Square(InnerNode {
        id: Uuid::new_v4().to_string(),
        children: vec![],
        location: mock_location(),
    })
}

pub fn new_square_with_children(children: Vec<AST>) -> AST {
    let location = merge_all_locations(&children);
    AST::Square(InnerNode {
        id: Uuid::new_v4().to_string(),
        children,
        location,
    })
}

pub fn new_ident(value: String, children: Vec<AST>) -> AST {
    let location = merge_all_locations(&children);
    AST::Ident(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: Some(value),
        location,
    })
}

pub fn new_text(value: String, location: Location) -> AST {
    AST::Text(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: Some(value),
        location,
    })
}

pub fn new_invalid() -> AST {
    AST::Invalid(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: None,
        location: mock_location(),
    })
}

pub fn new_ignored() -> AST {
    AST::Ignored(LeafNode {
        id: Uuid::new_v4().to_string(),
        value: None,
        location: mock_location(),
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
