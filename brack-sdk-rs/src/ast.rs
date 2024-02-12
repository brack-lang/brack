use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct InnerNode {
    pub id: String,
    pub children: Vec<AST>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LeafNode {
    pub id: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum AST {
    Document(InnerNode),
    Stmt(InnerNode),
    Expr(InnerNode),
    Angle(InnerNode),
    Square(InnerNode),
    Curly(InnerNode),
    Identifier(InnerNode),
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
            | AST::Curly(node)
            | AST::Identifier(node) => &node.children,
            AST::Text(_) => panic!("Leaf node has no children"),
        }
    }

    pub fn value(&self) -> String {
        match self {
            AST::Text(leaf) => leaf.value.clone(),
            AST::Document(_)
            | AST::Stmt(_)
            | AST::Expr(_)
            | AST::Angle(_)
            | AST::Square(_)
            | AST::Curly(_)
            | AST::Identifier(_) => panic!("Inner node has no value"),
        }
    }

    pub fn id(&self) -> String {
        match self {
            AST::Document(node)
            | AST::Stmt(node)
            | AST::Expr(node)
            | AST::Angle(node)
            | AST::Square(node)
            | AST::Curly(node)
            | AST::Identifier(node) => node.id.clone(),
            AST::Text(leaf) => leaf.id.clone(),
        }
    }
}

impl AST {
    pub fn add(&mut self, ast: AST) -> Result<()> {
        match self {
            AST::Document(node)
            | AST::Stmt(node)
            | AST::Expr(node)
            | AST::Angle(node)
            | AST::Square(node)
            | AST::Curly(node)
            | AST::Identifier(node) => {
                node.children.push(ast);
            }
            AST::Text(_) => {
                anyhow::bail!("Cannot add child to leaf node");
            }
        }
        Ok(())
    }
}
