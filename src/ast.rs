use anyhow::Result;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct InnerNode {
    pub id: Uuid,
    pub children: Vec<AST>,
}

#[derive(Debug, PartialEq)]
pub struct LeafNode {
    pub id: Uuid,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub enum AST {
    Document(InnerNode),
    Stmt(InnerNode),
    Expr(InnerNode),
    Angle(InnerNode),
    Square(InnerNode),
    Curly(InnerNode),
    Identifier(LeafNode),
    Text(LeafNode),
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
            AST::Identifier(_) | AST::Text(_) => panic!("Leaf node has no children"),
        }
    }

    pub fn value(&self) -> String {
        match self {
            AST::Identifier(leaf) | AST::Text(leaf) => leaf.value.clone(),
            AST::Document(_)
            | AST::Stmt(_)
            | AST::Expr(_)
            | AST::Angle(_)
            | AST::Square(_)
            | AST::Curly(_) => panic!("Inner node has no value"),
        }
    }
}

pub fn new_document() -> AST {
    AST::Document(InnerNode {
        id: Uuid::new_v4(),
        children: vec![],
    })
}

pub fn new_document_with_children(children: Vec<AST>) -> AST {
    AST::Document(InnerNode {
        id: Uuid::new_v4(),
        children,
    })
}

pub fn new_stmt() -> AST {
    AST::Stmt(InnerNode {
        id: Uuid::new_v4(),
        children: vec![],
    })
}

pub fn new_stmt_with_children(children: Vec<AST>) -> AST {
    AST::Stmt(InnerNode {
        id: Uuid::new_v4(),
        children,
    })
}

pub fn new_expr() -> AST {
    AST::Expr(InnerNode {
        id: Uuid::new_v4(),
        children: vec![],
    })
}

pub fn new_expr_with_children(children: Vec<AST>) -> AST {
    AST::Expr(InnerNode {
        id: Uuid::new_v4(),
        children,
    })
}

pub fn new_angle() -> AST {
    AST::Angle(InnerNode {
        id: Uuid::new_v4(),
        children: vec![],
    })
}

pub fn new_angle_with_children(children: Vec<AST>) -> AST {
    AST::Angle(InnerNode {
        id: Uuid::new_v4(),
        children,
    })
}

pub fn new_curly() -> AST {
    AST::Curly(InnerNode {
        id: Uuid::new_v4(),
        children: vec![],
    })
}

pub fn new_curly_with_children(children: Vec<AST>) -> AST {
    AST::Curly(InnerNode {
        id: Uuid::new_v4(),
        children,
    })
}

pub fn new_square() -> AST {
    AST::Square(InnerNode {
        id: Uuid::new_v4(),
        children: vec![],
    })
}

pub fn new_square_with_children(children: Vec<AST>) -> AST {
    AST::Square(InnerNode {
        id: Uuid::new_v4(),
        children,
    })
}

pub fn new_ident(value: String) -> AST {
    AST::Identifier(LeafNode {
        id: Uuid::new_v4(),
        value,
    })
}

pub fn new_text(value: String) -> AST {
    AST::Text(LeafNode {
        id: Uuid::new_v4(),
        value,
    })
}

impl AST {
    pub fn add(&mut self, ast: AST) -> Result<()> {
        match self {
            AST::Document(node)
            | AST::Stmt(node)
            | AST::Expr(node)
            | AST::Angle(node)
            | AST::Square(node)
            | AST::Curly(node) => {
                node.children.push(ast);
            }
            AST::Identifier(_) | AST::Text(_) => {
                anyhow::bail!("Cannot add child to leaf node");
            }
        }
        Ok(())
    }
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
        (AST::Identifier(leaf1), AST::Identifier(leaf2)) => assert_leaf_node_eq(leaf1, leaf2),
        (AST::Text(leaf1), AST::Text(leaf2)) => assert_leaf_node_eq(leaf1, leaf2),
        _ => panic!(
            "Mismatched AST node types or unexpected AST node\nleft: {:?}\nright: {:?}",
            node1, node2
        ),
    }
}
