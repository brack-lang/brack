use anyhow::Result;
use brack_plugin::{plugins::Plugins, types::Type};
use brack_transformer::ast::AST;

use crate::errors::LoweringError;
use crate::op_code::OpCode;
use crate::{expr, square, text};

pub(crate) fn lowering(ast: &AST, plugins: &Plugins) -> Result<Vec<OpCode>, LoweringError> {
    match ast {
        AST::Curly(_) => (),
        _ => panic!("Curly must be a curly"),
    }

    let mut op_codes = vec![];
    let mut result = vec![];

    let module = ast
        .children()
        .first()
        .expect("Curly must contain module");

    let ident = ast
        .children()
        .get(1)
        .expect("Curly must contain ident");

    let module_name = match module {
        AST::Module(module) => module.value.clone(),
        _ => panic!("Module must be a module"),
    };
    let module_name = match module_name {
        Some(module_name) => module_name,
        None => panic!("Module name must be a string"),
    };

    let ident_name = match ident {
        AST::Ident(ident) => ident.value.clone(),
        _ => panic!("Ident must be an ident"),
    };
    let ident_name = match ident_name {
        Some(ident_name) => ident_name,
        None => panic!("Ident name must be a string"),
    };

    for child in ast.children().iter().skip(2) {
        let res = match child {
            AST::Expr(_) => expr::lowering(child, &plugins)?,
            AST::Curly(_) => lowering(child, &plugins)?,
            AST::Square(_) => square::lowering(child, &plugins)?,
            AST::Text(_) => text::lowering(child, &plugins)?,
            AST::Angle(_) => panic!("Angle must be expanded by the macro expander."),
            ast => panic!("Curly cannot contain the following node\n{}", ast),
        };
        op_codes.extend(res);
    }

    op_codes.push(OpCode::Join(ast.children().len() - 2));

    result.push(OpCode::Call { plugin_name: module_name, function_name: ident_name,return_type: Type::TBlock });

    Ok(result)
}
