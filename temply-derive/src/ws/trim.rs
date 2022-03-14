use crate::parser::ast;

pub fn trim(ast: &mut ast::Ast<'_>) {
    // Do not trim at top level, instead call trim items
    trim_items(&mut ast.items)
}

fn trim_items(items: &mut [ast::Item<'_>]) {
    for item in items {
        match item {
            ast::Item::Text(_) => (),
            ast::Item::Comment(_) => (),
            ast::Item::Expr(_, _) => (),
            ast::Item::Let(_) => (),
            ast::Item::Scope(body) => {
                trim_ast(body);
            }
            ast::Item::For { for_: _, pre, body } => {
                *pre = trim_ast(body);
            }
            ast::Item::If {
                if_,
                else_ifs,
                else_,
            } => {
                trim_ast(&mut if_.1);
                for else_if in else_ifs {
                    trim_ast(&mut else_if.1);
                }
                if let Some(else_) = else_ {
                    trim_ast(else_);
                }
            }
            ast::Item::Match { match_: _, wheres } => {
                for where_ in wheres {
                    trim_ast(&mut where_.1);
                }
            }
            ast::Item::Macro {
                name: _,
                params: _,
                body,
            } => {
                trim_ast(body);
            }
            ast::Item::Call { name: _, args: _ } => (),
        }
    }
}

fn trim_ast<'s>(ast: &mut ast::Ast<'s>) -> Option<ast::Text<'s>> {
    // Trim items of ast
    trim_items(&mut ast.items);

    // Trim self
    let mut pre = None;
    if let Some(ast::Item::Text(text)) = ast.items.first_mut() {
        pre = text_trim_start(text);
        if text_is_empty(text) {
            ast.items.remove(0);
        }
    }
    if let Some(ast::Item::Text(text)) = ast.items.last_mut() {
        text_trim_end(text);
        if text_is_empty(text) {
            ast.items.pop().unwrap();
        }
    }
    pre
}

fn text_trim_start<'s>(text: &mut ast::Text<'s>) -> Option<ast::Text<'s>> {
    let mut pre = ast::Text {
        lines: Vec::new(),
        trailing: "",
    };

    loop {
        match text.lines.first_mut() {
            Some(line) => {
                let trimmed = line.content.trim_start();
                if trimmed.len() == 0 {
                    pre.lines.push(text.lines.remove(0));
                } else if trimmed.len() < line.content.len() {
                    pre.trailing = &line.content[0..line.content.len() - trimmed.len()];
                    line.content = trimmed;
                    break;
                } else {
                    break;
                }
            }
            None => {
                let trimmed = text.trailing.trim_start();
                pre.trailing = &text.trailing[0..text.trailing.len() - trimmed.len()];
                text.trailing = trimmed;
                break;
            }
        }
    }

    if text_is_empty(&pre) {
        None
    } else {
        Some(pre)
    }
}

fn text_trim_end(text: &mut ast::Text<'_>) {
    loop {
        text.trailing = text.trailing.trim_end();

        if text.trailing.is_empty() {
            match text.lines.pop() {
                Some(line) => text.trailing = line.content,
                None => break,
            }
        } else {
            break;
        }
    }
}

fn text_is_empty(text: &ast::Text<'_>) -> bool {
    text.lines.is_empty() && text.trailing.is_empty()
}
