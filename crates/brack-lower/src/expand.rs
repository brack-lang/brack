use brack_common::{ast::{InnerNode, AST}, errors::{Error, LoweringError}, plugins::{CommandType, Plugins}};

fn apply_expanding_to_node(node: &InnerNode, plugins: &Plugins) -> Result<InnerNode, Error> {
    let mut children = vec![];
    for child in node.children.clone() {
        children.push(expand_once(&child, plugins)?);
    }
    return Ok(InnerNode {
        id: node.id.clone(),
        location: node.location.clone(),
        children,
    })
}

fn expand_once(ast: &AST, plugins: &Plugins) -> Result<AST, Error> {
    match ast {
        AST::Document(node) => Ok(AST::Document(apply_expanding_to_node(node, plugins)?)),
        AST::Stmt(node) => Ok(AST::Stmt(apply_expanding_to_node(node, plugins)?)),
        AST::Expr(node) => Ok(AST::Expr(apply_expanding_to_node(node, plugins)?)),
        AST::Angle(node) => {
            let children = node.children.clone();
            let module_name = if let AST::Module(node) = children.get(0).ok_or(Error::LoweringError(LoweringError::ModuleNotFound))? {
                node.value
            } else {
                // エラーを返す
                return 
            };
            let ident_name = if let AST::Ident(node) = children.get(1).unwrap() {
                node.value().unwrap()
            } else {
                // error
            };
            let plugin = plugins.get_mut(module_name).unwrap();
            plugin.call(ident_name, &CommandType::Macro, args)
                .map_err(|err| Error::PluginError(err))
        },
        otherwise => Ok(otherwise.clone()),
    }
}

fn remain_macro_for_inner_node(node: &InnerNode) -> bool {
    let mut result = false;
    for child in node.children.clone() {
        result = result || remain_macro(&child);
    }
    result
}

fn remain_macro(ast: &AST) -> bool {
    match ast {
        AST::Document(node)
        | AST::Stmt(node)
        | AST::Expr(node)
        | AST::Curly(node)
        | AST::Square(node) => remain_macro_for_inner_node(node),
        AST::Angle(_) => true, 
        _ => false,
    }
}

pub(crate) fn expand(ast: &AST, plugins: &Plugins) -> Result<AST, Error> {
    let mut result = ast.clone();
    while !remain_macro(ast) {
        result = expand_once(ast, plugins)?;
    }
    return Ok(result)
}

