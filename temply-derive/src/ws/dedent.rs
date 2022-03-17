use super::util::inner_asts_mut;
use crate::parser::ast;

pub fn dedent(ast: &mut ast::Ast<'_>) {
    // Do not dedent at top level, instead call dedent items
    dedent_items(&mut ast.items, 0)
}

fn dedent_items(items: &mut [ast::Item<'_>], mut base_indent: usize) {
    let mut indent_add = true;

    for item in items {
        match item {
            ast::Item::Text(text) => {
                if !text.lines.is_empty() {
                    base_indent = 0;
                    indent_add = true;
                }
                if indent_add {
                    let ind = text.trailing.len() - text.trailing.trim_start_matches(' ').len();
                    base_indent += ind;
                    indent_add = ind == text.trailing.len();
                }
            }
            _ => {
                // Special case macro/call
                let base_indent = match item {
                    ast::Item::Macro { .. } => 0,
                    ast::Item::Call { ind, .. } => {
                        *ind = base_indent;
                        base_indent
                    }
                    _ => base_indent,
                };

                for ast in inner_asts_mut(item) {
                    // Dedent ast
                    if let Some(items_indent) = ast.items_indent {
                        if items_indent > base_indent {
                            let dedent = items_indent - base_indent;
                            dedent_ast(ast, dedent);
                        }
                    }

                    // Dedent items of ast
                    dedent_items(&mut ast.items, base_indent);
                }
            }
        }
    }
}

fn dedent_ast(ast: &mut ast::Ast<'_>, dedent: usize) {
    if let Some(items_indent) = &mut ast.items_indent {
        *items_indent -= dedent;
    }

    let mut is_at_line_start = false;
    let len = ast.items.len();

    for (idx, item) in ast.items.iter_mut().enumerate() {
        let is_last = idx == len - 1;

        match item {
            ast::Item::Text(text) => {
                for line in &mut text.lines {
                    if is_at_line_start && !line.content.is_empty() {
                        line.content = &line.content[dedent..];
                    }
                    is_at_line_start = true;
                }

                if is_at_line_start && !text.trailing.is_empty() {
                    if !(is_last && text.trailing.chars().all(|c| c == ' ')) {
                        text.trailing = &text.trailing[dedent..];
                    }
                    is_at_line_start = false;
                }
            }
            _ => {
                for ast in inner_asts_mut(item) {
                    dedent_ast(ast, dedent);
                }
                is_at_line_start = false;
            }
        }
    }
}
