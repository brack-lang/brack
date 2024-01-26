use anyhow::Result;
use brack_parser::ast::AST;
use brack_plugin::plugin::{PluginArgument, Plugins};
use extism::convert::Json;

use crate::{curly, expr, identifier, text};

pub fn generate(ast: &AST, plugins: &mut Plugins) -> Result<String> {
    let mut module_name = String::from("");
    let mut ident_name = String::from("");

    let mut arguments = vec![];
    for (i, child) in ast.children().iter().enumerate() {
        let res = match child {
            AST::Expr(_) => expr::generate(&child, plugins)?,
            AST::Curly(_) => curly::generate(&child, plugins)?,
            AST::Square(_) => generate(&child, plugins)?,
            AST::Identifier(_) => identifier::generate(&child)?,
            AST::Text(_) => text::generate(&child)?,
            AST::Angle(_) => anyhow::bail!("Angle must be expanded by the macro expander."),
            _ => anyhow::bail!("Square cannot contain Document, Stmt, Expr and Square"),
        };
        if i == 0 {
            let (module, ident) = match res.split_once(" ") {
                Some((module, ident)) => (module, ident),
                None => anyhow::bail!("Square must contain module and identifier"),
            };
            module_name = module.to_string();
            ident_name = ident.to_string();
        } else {
            arguments.push(res);
        }
    }

    let plugin_argument = PluginArgument::new(arguments);
    let plugin = plugins
        .get_mut(&module_name)
        .ok_or_else(|| anyhow::anyhow!("Module {} not found", module_name))?;
    let res = plugin.call::<Json<PluginArgument>, &str>(&ident_name, Json(plugin_argument))?;

    Ok(format!("{}", res))
}
