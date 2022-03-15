use crate::parser::ast;

pub fn inner_asts<'a, 's>(item: &'a ast::Item<'s>) -> impl Iterator<Item = &'a ast::Ast<'s>> + 'a {
    let mut asts = Vec::new();

    match item {
        ast::Item::Text(_) => (),
        ast::Item::Comment(_) => (),
        ast::Item::Expr(_, _) => (),
        ast::Item::Let(_) => (),
        ast::Item::Scope(body) => {
            asts.push(body);
        }
        ast::Item::For {
            for_: _,
            pre: _,
            body,
        } => {
            asts.push(body);
        }
        ast::Item::If {
            if_,
            else_ifs,
            else_,
        } => {
            asts.push(&if_.1);
            for else_if in else_ifs {
                asts.push(&else_if.1);
            }
            if let Some(else_) = else_ {
                asts.push(else_);
            }
        }
        ast::Item::Match { match_: _, wheres } => {
            for where_ in wheres {
                asts.push(&where_.1);
            }
        }
        ast::Item::Macro {
            name: _,
            params: _,
            body,
        } => {
            asts.push(body);
        }
        ast::Item::Call {
            name: _,
            args: _,
            ind: _,
        } => (),
    }

    asts.into_iter()
}

pub fn inner_asts_mut<'a, 's>(
    item: &'a mut ast::Item<'s>,
) -> impl Iterator<Item = &'a mut ast::Ast<'s>> + 'a {
    let mut asts = Vec::new();

    match item {
        ast::Item::Text(_) => (),
        ast::Item::Comment(_) => (),
        ast::Item::Expr(_, _) => (),
        ast::Item::Let(_) => (),
        ast::Item::Scope(body) => {
            asts.push(body);
        }
        ast::Item::For {
            for_: _,
            pre: _,
            body,
        } => {
            asts.push(body);
        }
        ast::Item::If {
            if_,
            else_ifs,
            else_,
        } => {
            asts.push(&mut if_.1);
            for else_if in else_ifs {
                asts.push(&mut else_if.1);
            }
            if let Some(else_) = else_ {
                asts.push(else_);
            }
        }
        ast::Item::Match { match_: _, wheres } => {
            for where_ in wheres {
                asts.push(&mut where_.1);
            }
        }
        ast::Item::Macro {
            name: _,
            params: _,
            body,
        } => {
            asts.push(body);
        }
        ast::Item::Call {
            name: _,
            args: _,
            ind: _,
        } => (),
    }

    asts.into_iter()
}
