use anyhow::Result;
use brack_plugin::{plugins::Plugins, types::Type};
use brack_transformer::ast::AST;

use crate::errors::LoweringError;
use crate::expr;
use crate::op_code::OpCode;

pub(crate) fn lowering(ast: &AST, plugins: &Plugins) -> Result<Vec<OpCode>, LoweringError> {
    let AST::Curly(_) = ast else {
        return Err(LoweringError::Panic {
            message: format!("Curly must be a curly but found {:?}", ast),
            location: ast.location().clone(),
        });
    };

    let mut op_codes = vec![];
    let mut result = vec![];

    let module = ast.children().first().expect("Curly must contain module");

    let module_name = match module {
        AST::Module(module) => module.value.clone(),
        _ => return Err(LoweringError::Panic {
            message: format!("Module must be a module but found {:?}", module),
            location: module.location().clone(),
        }),
    };

    let Some(module_name) = module_name else {
        return Err(LoweringError::Panic {
            message: "Module name must be a string".to_string(),
            location: module.location().clone(),
        });
    };

    let Some(plugin) = plugins.name_to_plugin.get(&module_name) else {
        return Err(LoweringError::PluginNotFound(module.location().clone()));
    };

    let ident = ast.children().get(1).expect("Curly must contain ident");

    let ident_name = match ident {
        AST::Ident(ident) => ident.value.clone(),
        _ => return Err(LoweringError::Panic {
            message: format!("Ident must be an ident but found {:?}", ident),
            location: ident.location().clone(),
        }),
    };

    let Some(ident_name) = ident_name else {
        return Err(LoweringError::Panic {
            message: "Ident name must be a string".to_string(),
            location: ident.location().clone(),
        });
    };

    let Some(metadata) = plugin
        .signature_to_metadata
        .get(&(ident_name.clone(), Type::TBlock))
    else {
        return Err(LoweringError::CommandNotFound(ident.location().clone()));
    };

    let required_args = metadata
        .argument_types
        .iter()
        .map(|(s, t)| (s.clone(), t.clone()))
        .collect::<Vec<_>>();

    let childs = ast.children().iter().skip(2).collect::<Vec<_>>();
    let mut arg_count = 0;

    for i in 0..required_args.len() {
        let arg_type = required_args[i].1.clone();
        let child = childs.get(i).and_then(|child| Some(child.clone()));
        match arg_type {
            Type::TInline => {
                let Some(child) = child else {
                    return Err(LoweringError::MissingArgument {
                        required: required_args.len(),
                        provided: i,
                        missing: required_args
                            .iter()
                            .skip(i)
                            .map(|(s, _)| s.clone())
                            .collect(),
                        location: ast.location().clone(),
                    });
                };

                let res = expr::lowering(child, &plugins)?;
                op_codes.extend(res);
                arg_count += 1;
            }
            Type::TOption(t) => {
                let Type::TInline = *t else {
                    return Err(LoweringError::Panic {
                        message: format!("Plugin requires illegal type {:?}", t),
                        location: ast.location().clone(),
                    });
                };
                let Some(child) = child else {
                    op_codes.push(OpCode::ToOption(None));
                    break;
                };
                let res = expr::lowering(child, &plugins)?;
                op_codes.extend(res);
                op_codes.push(OpCode::ToOption(Some(())));
                arg_count += 1;
            }
            Type::TArray(t) => {
                let Type::TInline = *t else {
                    return Err(LoweringError::Panic {
                        message: format!("Plugin requires illegal type {:?}", t),
                        location: ast.location().clone(),
                    });
                };
                for child in childs.iter().skip(i) {
                    op_codes.extend(expr::lowering(child, &plugins)?);
                }
                op_codes.push(OpCode::ToArray(childs.len() - i));
                arg_count += childs.len() - i;
                break;
            }
            _ => {
                return Err(LoweringError::Panic {
                    message: format!("Plugin requires illegal type {:?}", arg_type),
                    location: ast.location().clone(),
                });
            }
        }
    }

    if childs.len() > arg_count {
        return Err(LoweringError::TooManyArguments {
            required: required_args.len(),
            provided: childs.len(),
            location: ast.location().clone(),
        });
    }

    result.push(OpCode::Call {
        plugin_name: module_name,
        function_name: ident_name,
        return_type: Type::TBlock,
    });

    Ok(result)
}
