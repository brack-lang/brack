use anyhow::Result;

use crate::ast::AST;

fn generate_document(ast: &AST) -> Result<String> {
    let mut result = String::from("");
    for child in ast.children() {
        let res = match child {
            AST::Stmt(_) => generate_stmt(&child)?,
            AST::Expr(_) => generate_expr(&child)?,
            AST::Angle(_) => generate_angle(&child)?,
            AST::Curly(_) => generate_curly(&child)?,
            AST::Square(_) => generate_square(&child)?,
            AST::Identifier(_) => generate_identifier(&child)?,
            AST::Text(_) => generate_text(&child)?,
            _ => anyhow::bail!("Document cannot contain Document"),
        };
        result.push_str(&res);
    }
    Ok(result)
}

fn generate_stmt(ast: &AST) -> Result<String> {
    let mut result = String::from("");
    for child in ast.children() {
        let res = match child {
            AST::Expr(_) => generate_expr(&child)?,
            AST::Angle(_) => generate_angle(&child)?,
            AST::Curly(_) => generate_curly(&child)?,
            AST::Square(_) => generate_square(&child)?,
            AST::Identifier(_) => generate_identifier(&child)?,
            AST::Text(_) => generate_text(&child)?,
            _ => anyhow::bail!("Stmt cannot contain Document and Stmt"),
        };
        result.push_str(&res);
    }
    Ok(result + "\n")
}

fn generate_expr(ast: &AST) -> Result<String> {
    let mut result = String::from("");
    for child in ast.children() {
        let res = match child {
            AST::Angle(_) => generate_angle(&child)?,
            AST::Square(_) => generate_square(&child)?,
            AST::Identifier(_) => generate_identifier(&child)?,
            AST::Text(_) => generate_text(&child)?,
            _ => anyhow::bail!("Expr cannot contain Document, Stmt and Expr"),
        };
        result.push_str(&res);
    }
    Ok(result)
}

fn generate_angle(ast: &AST) -> Result<String> {
    let mut result = String::from("");
    for child in ast.children() {
        let res = match child {
            AST::Expr(_) => generate_expr(&child)?,
            AST::Angle(_) => generate_angle(&child)?,
            AST::Curly(_) => generate_curly(&child)?,
            AST::Square(_) => generate_square(&child)?,
            AST::Identifier(_) => generate_identifier(&child)?,
            AST::Text(_) => generate_text(&child)?,
            _ => anyhow::bail!("Angle cannot contain Document, Stmt, Expr and Angle"),
        };
        result.push_str(&res);
    }
    Ok(format!("<{}>", result))
}

fn generate_curly(ast: &AST) -> Result<String> {
    let mut result = String::from("");
    for child in ast.children() {
        let res = match child {
            AST::Expr(_) => generate_expr(&child)?,
            AST::Angle(_) => generate_angle(&child)?,
            AST::Square(_) => generate_square(&child)?,
            AST::Identifier(_) => generate_identifier(&child)?,
            AST::Text(_) => generate_text(&child)?,
            _ => anyhow::bail!("Curly cannot contain Document, Stmt, Expr and Curly"),
        };
        result.push_str(&res);
    }
    Ok(format!("{{{}}}", result))
}

fn generate_square(ast: &AST) -> Result<String> {
    let mut result = String::from("");
    for child in ast.children() {
        let res = match child {
            AST::Expr(_) => generate_expr(&child)?,
            AST::Angle(_) => generate_angle(&child)?,
            AST::Square(_) => generate_square(&child)?,
            AST::Identifier(_) => generate_identifier(&child)?,
            AST::Text(_) => generate_text(&child)?,
            _ => anyhow::bail!("Square cannot contain Document, Stmt, Expr and Curly"),
        };
        result.push_str(&res);
    }
    Ok(format!("[{}]", result))
}

fn generate_identifier(ast: &AST) -> Result<String> {
    Ok(format!("{} ", ast.value()))
}

fn generate_text(ast: &AST) -> Result<String> {
    Ok(ast.value())
}

pub fn generate(ast: &AST) -> Result<String> {
    generate_document(ast)
}
