use super::util::{inner_asts, inner_asts_mut};
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
                    let extra_indent = indent_of_str(text.trailing);
                    base_indent += extra_indent;
                    indent_add = extra_indent == text.trailing.len();
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
                    // Dedent self
                    let inner_indent = indent(ast, true).unwrap_or(0);
                    if inner_indent > base_indent {
                        let dedent = inner_indent - base_indent;
                        dedent_ast(ast, dedent, true);
                    }

                    // Dedent items of ast
                    dedent_items(&mut ast.items, base_indent);
                }
            }
        }
    }
}

fn dedent_ast(ast: &mut ast::Ast<'_>, dedent: usize, mut is_at_line_start: bool) {
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

                let is_empty = text.trailing.is_empty();
                if is_at_line_start && !(is_last && text.trailing.chars().all(|c| c == ' ')) {
                    text.trailing = &text.trailing[dedent..];
                }
                if !is_empty {
                    is_at_line_start = false;
                }
            }
            _ => {
                for ast in inner_asts_mut(item) {
                    dedent_ast(ast, dedent, false);
                }
                is_at_line_start = false;
            }
        }
    }
}

fn indent(ast: &ast::Ast<'_>, mut is_at_line_start: bool) -> Option<usize> {
    let mut min_indent = None;
    let mut update = |indent| match (&mut min_indent, indent) {
        (Some(min_indent), Some(indent)) => {
            if indent < *min_indent {
                *min_indent = indent;
            }
        }
        (_, None) => (),
        (None, _) => min_indent = indent,
    };

    for (idx, item) in ast.items.iter().enumerate() {
        let is_last = idx == ast.items.len() - 1;

        match item {
            ast::Item::Text(text) => {
                for line in &text.lines {
                    if is_at_line_start && !line.content.is_empty() {
                        update(Some(indent_of_str(line.content)))
                    }
                    is_at_line_start = true;
                }

                let is_empty = text.trailing.is_empty();
                if is_at_line_start && !(is_last && text.trailing.chars().all(|c| c == ' ')) {
                    update(Some(indent_of_str(text.trailing)))
                }
                if !is_empty {
                    is_at_line_start = false;
                }
            }
            _ => {
                if is_at_line_start {
                    return Some(0);
                }
                for ast in inner_asts(item) {
                    update(indent(ast, false));
                }
            }
        }
    }

    min_indent
}

fn indent_of_str(s: &str) -> usize {
    s.len() - s.trim_start_matches(' ').len()
}
