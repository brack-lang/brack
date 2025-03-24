use anyhow::Result;
use brack_plugin::{plugins::Plugins, types::Type};
use brack_transformer::ast::AST;

use crate::errors::LoweringError;
use crate::op_code::OpCode;
use crate::{expr, square, text};

pub(crate) fn lowering(ast: &AST, plugins: &Plugins) -> Result<Vec<OpCode>, LoweringError> {
    let AST::Curly(_) = ast else { panic!("Curly must be a curly") };

    let mut op_codes = vec![];
    let mut result = vec![];

    let module = ast
        .children()
        .first()
        .expect("Curly must contain module");

    let module_name = match module {
        AST::Module(module) => module.value.clone(),
        _ => panic!("Module must be a module"),
    };

    let module_name = match module_name {
        Some(module_name) => module_name,
        None => panic!("Module name must be a string"),
    };

    let plugin = match plugins.name_to_plugin.get(&module_name) {
        Some(plugin) => plugin,
        None => return Err(LoweringError::PluginNotFound(module.location().clone())),
    };

    let ident = ast
        .children() 
        .get(1)
        .expect("Curly must contain ident");


    let ident_name = match ident {
        AST::Ident(ident) => ident.value.clone(),
        _ => panic!("Ident must be an ident"),
    };
    let ident_name = match ident_name {
        Some(ident_name) => ident_name,
        None => panic!("Ident name must be a string"),
    };

    let metadata = match plugin.signature_to_metadata.get(&(ident_name.clone(), Type::TBlock)) {
        Some(metadata) => metadata.clone(),
        None => return Err(LoweringError::CommandNotFound(ident.location().clone())),
    };

    let required_args = metadata.argument_types.iter().map(|(s, t)| (s.clone(), t.clone())).collect::<Vec<_>>();

    let childs = ast.children().iter().skip(2).collect::<Vec<_>>();

    for i in 0..required_args.len() {
        let (arg_name, arg_type) = &required_args[i];
        let child = childs.get(i).and_then(|child| Some(child.clone()));
        match arg_type {
            Type::TInline => {
                let child = match child {
                    Some(child) => child,
                    None => return Err(LoweringError::MissingArgument {
                        required: required_args.len(),
                        provided: i,
                        missing: required_args.iter().skip(i).map(|(s, _)| s.clone()).collect(),
                        location: ast.location().clone(),
                    }),
                };
                let res = expr::lowering(child, &plugins)?;
                op_codes.extend(res);
            }
            Type::TOption(t) => {
                let Type::TInline = **t else { panic!("Plugin requires illegal type") };
                let child = match child {
                    Some(child) => child,
                    None => {
                        op_codes.push(OpCode::ToOption(None));
                        continue;
                    }
                };
                let res = expr::lowering(child, &plugins)?;
                op_codes.extend(res);
                op_codes.push(OpCode::ToOption(Some(())));
            }
            Type::TArray(t) => {
                let Type::TInline = **t else { panic!("Plugin requires illegal type") };
                for child in childs.iter().skip(i) {
                    op_codes.extend(expr::lowering(child, &plugins)?);
                }
                op_codes.push(OpCode::ToArray(childs.len() - i));
            }
            _ => panic!("Plugin requires illegal type"),
        }
    }

    op_codes.push(OpCode::Join(ast.children().len() - 2));

    result.push(OpCode::Call { plugin_name: module_name, function_name: ident_name, return_type: Type::TBlock });

    Ok(result)
}
