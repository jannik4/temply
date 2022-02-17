pub mod ast;

use crate::lexer::{Span, Spanned, Token};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Write;

#[derive(Debug)]
pub struct Error {
    span: Span,
    token: Option<Token>,
    expected: HashSet<Expected>,
}

impl Error {
    fn new(span: Span, token: Option<Token>, expected: HashSet<Expected>) -> Self {
        Self {
            span,
            token,
            expected,
        }
    }

    pub fn format(&self, source: &str) -> String {
        let mut buffer = String::new();

        write!(
            &mut buffer,
            "@{}..{} '{}': found `{:?}`, expected one of ",
            self.span.start,
            self.span.end,
            &source[self.span.range()],
            self.token
        )
        .unwrap();

        let mut expected = self.expected.iter().collect::<Vec<_>>();
        expected.sort();
        for (idx, expected) in expected.iter().enumerate() {
            if idx != 0 {
                write!(&mut buffer, ", ").unwrap();
            }
            match expected {
                Expected::Block(filter) => {
                    write!(&mut buffer, "{}-block", filter.name()).unwrap();
                }
                Expected::Token(token) => write!(&mut buffer, "`{:?}`", token).unwrap(),
            }
        }

        buffer
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Expected {
    Block(BlockFilter),
    Token(Token),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum BlockFilter {
    Equals(&'static str),
    StartsWith(&'static str),
}

impl BlockFilter {
    fn name(&self) -> &'static str {
        match self {
            BlockFilter::Equals(filter) => filter,
            BlockFilter::StartsWith(filter) => filter,
        }
    }
}

pub type Result<'t, T> = std::result::Result<(&'t [Spanned<Token>], T), Error>;

pub fn parse<'s>(
    source: &'s str,
    tokens: &[Spanned<Token>],
) -> std::result::Result<ast::Ast<'s>, Error> {
    let (tokens, (_, ast)) = parse_ast(source, tokens, false)?;

    if tokens.is_empty() {
        Ok(ast)
    } else {
        Err(parse_item(source, tokens).unwrap_err())
    }
}

fn parse_ast<'s, 't>(
    source: &'s str,
    mut tokens: &'t [Spanned<Token>],
    trim: bool,
) -> Result<'t, (Option<&'s str>, ast::Ast<'s>)> {
    let mut items = Vec::new();

    while let Ok((rest, item)) = parse_item(source, tokens) {
        tokens = rest;
        items.push(item);
    }

    let ast = ast::Ast { items };
    let (pre, ast) = if trim { trim_ast(ast) } else { (None, ast) };

    Ok((tokens, (pre, ast)))
}

fn parse_item<'s, 't>(source: &'s str, tokens: &'t [Spanned<Token>]) -> Result<'t, ast::Item<'s>> {
    parse_text(source, tokens)
        .alt(|| parse_comment(source, tokens))
        .alt(|| parse_expr(source, tokens))
        .alt(|| parse_let(source, tokens))
        .alt(|| parse_scope(source, tokens))
        .alt(|| parse_for(source, tokens))
        .alt(|| parse_if(source, tokens))
        .alt(|| parse_match(source, tokens))
}

fn parse_text<'s, 't>(source: &'s str, tokens: &'t [Spanned<Token>]) -> Result<'t, ast::Item<'s>> {
    let (rest, span) = exact(source, tokens, Token::Other)?;
    Ok((rest, ast::Item::Text(&source[span.range()])))
}

fn parse_comment<'s, 't>(
    source: &'s str,
    tokens: &'t [Spanned<Token>],
) -> Result<'t, ast::Item<'s>> {
    // Error
    let unclosed_comment = || {
        Error::new(
            (source.len()..source.len()).into(),
            None,
            [Expected::Token(Token::CommentEnd)].into_iter().collect(),
        )
    };

    // Start
    let (mut tokens, _span) = exact(source, tokens, Token::CommentStart)?;
    let mut depth = 1;
    let span_start = tokens.first().ok_or_else(unclosed_comment)?.span.start;
    let mut span_end = span_start;

    // Body/End
    while let Some(t) = tokens.first() {
        match t.node {
            Token::CommentStart => depth += 1,
            Token::CommentEnd => depth -= 1,
            _ => span_end = t.span.end,
        }
        tokens = &tokens[1..];

        if depth == 0 {
            return Ok((tokens, ast::Item::Comment(&source[span_start..span_end])));
        }
    }

    Err(unclosed_comment())
}

fn parse_expr<'s, 't>(source: &'s str, tokens: &'t [Spanned<Token>]) -> Result<'t, ast::Item<'s>> {
    let (tokens, _) = exact(source, tokens, Token::ExprStart)?;
    let (tokens, span) = exact(source, tokens, Token::Other)?;
    let (tokens, _) = exact(source, tokens, Token::ExprEnd)?;

    let expr = source[span.range()].trim();
    let (expr, format) = match expr.rfind("@{") {
        Some(pos) => (&expr[..pos], &expr[pos + 1..]),
        None => (expr, "{}"),
    };

    Ok((tokens, ast::Item::Expr(expr, format)))
}

fn parse_let<'s, 't>(source: &'s str, tokens: &'t [Spanned<Token>]) -> Result<'t, ast::Item<'s>> {
    let (tokens, let_) = parse_block(source, tokens, BlockFilter::StartsWith("let"))?;

    Ok((tokens, ast::Item::Let(let_)))
}

fn parse_scope<'s, 't>(source: &'s str, tokens: &'t [Spanned<Token>]) -> Result<'t, ast::Item<'s>> {
    // Start
    let (tokens, _) = parse_block(source, tokens, BlockFilter::Equals("scope"))?;

    // Body
    let (tokens, (_, body)) = parse_ast(source, tokens, true)?;

    // End
    let (tokens, _) = parse_block(source, tokens, BlockFilter::Equals("endscope"))?;

    Ok((tokens, ast::Item::Scope(body)))
}

fn parse_for<'s, 't>(source: &'s str, tokens: &'t [Spanned<Token>]) -> Result<'t, ast::Item<'s>> {
    // Start
    let (tokens, for_) = parse_block(source, tokens, BlockFilter::StartsWith("for"))?;

    // Body
    let (tokens, (pre, body)) = parse_ast(source, tokens, true)?;

    // End
    let (tokens, _) = parse_block(source, tokens, BlockFilter::Equals("endfor"))?;

    Ok((tokens, ast::Item::For { for_, pre, body }))
}

fn parse_if<'s, 't>(source: &'s str, tokens: &'t [Spanned<Token>]) -> Result<'t, ast::Item<'s>> {
    // Start
    let (tokens, if_) = parse_block(source, tokens, BlockFilter::StartsWith("if"))?;

    // Body
    let (mut tokens, (_, body)) = parse_ast(source, tokens, true)?;

    // Else ifs
    let mut else_ifs = Vec::new();
    while let Ok((rest, else_if)) = parse_else_if(source, tokens) {
        tokens = rest;
        else_ifs.push(else_if);
    }

    // Else
    let else_ = match parse_else(source, tokens) {
        Ok((rest, else_)) => {
            tokens = rest;
            Some(else_)
        }
        Err(_) => None,
    };

    // End
    let (tokens, _) = parse_block(source, tokens, BlockFilter::Equals("endif"))?;

    Ok((
        tokens,
        ast::Item::If {
            if_: (if_, body),
            else_ifs,
            else_,
        },
    ))
}

fn parse_else_if<'s, 't>(
    source: &'s str,
    tokens: &'t [Spanned<Token>],
) -> Result<'t, (&'s str, ast::Ast<'s>)> {
    // Start
    let (tokens, else_if) = parse_block(source, tokens, BlockFilter::StartsWith("else if"))?;

    // Body
    let (tokens, (_, body)) = parse_ast(source, tokens, true)?;

    Ok((tokens, (else_if, body)))
}

fn parse_else<'s, 't>(source: &'s str, tokens: &'t [Spanned<Token>]) -> Result<'t, ast::Ast<'s>> {
    // Start
    let (tokens, _) = parse_block(source, tokens, BlockFilter::Equals("else"))?;

    // Body
    let (tokens, (_, body)) = parse_ast(source, tokens, true)?;

    Ok((tokens, body))
}

fn parse_match<'s, 't>(source: &'s str, tokens: &'t [Spanned<Token>]) -> Result<'t, ast::Item<'s>> {
    // Start
    let (mut tokens, match_) = parse_block(source, tokens, BlockFilter::StartsWith("match"))?;

    // Wheres
    let mut wheres = Vec::new();
    let mut run = true;
    while run {
        let (rest, ()) = skip_empty_other(source, tokens)?;
        tokens = rest;

        match parse_where(source, tokens) {
            Ok((rest, where_)) => {
                tokens = rest;
                wheres.push(where_);
            }
            Err(_) => run = false,
        }

        let (rest, ()) = skip_empty_other(source, tokens)?;
        tokens = rest;
    }

    // End
    let (tokens, _) = parse_block(source, tokens, BlockFilter::Equals("endmatch"))?;

    Ok((tokens, ast::Item::Match { match_, wheres }))
}

fn parse_where<'s, 't>(
    source: &'s str,
    tokens: &'t [Spanned<Token>],
) -> Result<'t, (&'s str, ast::Ast<'s>)> {
    // Start
    let (tokens, where_) = parse_block(source, tokens, BlockFilter::StartsWith("where"))?;
    let arm = where_["where".len()..].trim_start();

    // Body
    let (tokens, (_, body)) = parse_ast(source, tokens, true)?;

    // End
    let (tokens, _) = parse_block(source, tokens, BlockFilter::Equals("endwhere"))?;

    Ok((tokens, (arm, body)))
}

fn parse_block<'s, 't>(
    source: &'s str,
    tokens: &'t [Spanned<Token>],
    filter: BlockFilter,
) -> Result<'t, &'s str> {
    let error = || [Expected::Block(filter)].into_iter().collect();

    let (tokens, _) = exact(source, tokens, Token::BlockStart).map_expected(|_| error())?;
    let (tokens, span) = exact(source, tokens, Token::Other).map_expected(|_| error())?;
    let (tokens, _) = exact(source, tokens, Token::BlockEnd).map_expected(|_| error())?;

    let content = source[span.range()].trim();

    match filter {
        BlockFilter::Equals(f) => {
            if content != f {
                return Err(Error::new(span, Some(Token::BlockStart), error()));
            }
        }
        BlockFilter::StartsWith(f) => {
            if !content.starts_with(f) {
                return Err(Error::new(span, Some(Token::BlockStart), error()));
            }
        }
    }

    Ok((tokens, source[span.range()].trim()))
}

fn skip_empty_other<'s, 't>(source: &'s str, tokens: &'t [Spanned<Token>]) -> Result<'t, ()> {
    match exact(source, tokens, Token::Other) {
        Ok((tokens, span)) if source[span.range()].trim().is_empty() => return Ok((tokens, ())),
        _ => (),
    }

    Ok((tokens, ()))
}

fn exact<'s, 't>(source: &'s str, tokens: &'t [Spanned<Token>], token: Token) -> Result<'t, Span> {
    match tokens.first() {
        Some(t) if t.node == token => Ok((&tokens[1..], t.span)),
        Some(t) => Err(Error::new(
            t.span,
            Some(t.node),
            [Expected::Token(token)].into_iter().collect(),
        )),
        None => Err(Error::new(
            (source.len()..source.len()).into(),
            None,
            [Expected::Token(token)].into_iter().collect(),
        )),
    }
}

trait ResultExt {
    fn alt<F: Fn() -> Self>(self, f: F) -> Self;

    fn map_expected<F: Fn(HashSet<Expected>) -> HashSet<Expected>>(self, f: F) -> Self;
}

impl<T> ResultExt for Result<'_, T> {
    fn alt<F: Fn() -> Self>(self, f: F) -> Self {
        let err = match self {
            Ok(ok) => return Ok(ok),
            Err(err) => err,
        };

        let alt_err = match f() {
            Ok(ok) => return Ok(ok),
            Err(alt_err) => alt_err,
        };

        match err.span.cmp(&alt_err.span) {
            Ordering::Greater => Err(err),
            Ordering::Less => Err(alt_err),
            Ordering::Equal => Err(Error::new(
                err.span,
                err.token,
                err.expected
                    .into_iter()
                    .chain(alt_err.expected.into_iter())
                    .collect(),
            )),
        }
    }

    fn map_expected<F: Fn(HashSet<Expected>) -> HashSet<Expected>>(self, f: F) -> Self {
        match self {
            Ok(ok) => Ok(ok),
            Err(mut err) => {
                err.expected = f(err.expected);
                Err(err)
            }
        }
    }
}

fn trim_ast(mut ast: ast::Ast<'_>) -> (Option<&'_ str>, ast::Ast<'_>) {
    let mut pre = None;

    if let Some(ast::Item::Text(text)) = ast.items.first_mut() {
        let text_trimmed = text.trim_start();

        match &text[0..text.len() - text_trimmed.len()] {
            "" => (),
            p => pre = Some(p),
        }

        *text = text_trimmed;
        if text.is_empty() {
            ast.items.remove(0);
        }
    }

    if let Some(ast::Item::Text(text)) = ast.items.last_mut() {
        *text = text.trim_end();
        if text.is_empty() {
            ast.items.pop().unwrap();
        }
    }

    (pre, ast)
}
