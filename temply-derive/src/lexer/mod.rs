mod span;
mod token;

pub use self::{
    span::{Span, Spanned},
    token::Token,
};

#[derive(Debug, Clone, Copy)]
enum State {
    Default,
    BraceLeft,
    BraceRight,
    PercentSign,
    HashSign,
}

pub fn lex(source: &str) -> Vec<Spanned<Token>> {
    let mut state = State::Default;
    let mut pos_start = 0;
    let mut pos_current = 0;

    let mut tokens = Vec::new();

    for (idx, c) in source.chars().into_iter().enumerate() {
        pos_current = idx;

        state = match state {
            State::Default => match c {
                '{' => State::BraceLeft,
                '}' => State::BraceRight,
                '%' => State::PercentSign,
                '#' => State::HashSign,
                _ => State::Default,
            },
            State::BraceLeft => match c {
                '{' | '%' | '#' => {
                    if pos_start < pos_current - 1 {
                        tokens.push(Spanned {
                            node: Token::Other,
                            span: (pos_start..pos_current - 1).into(),
                        });
                    }
                    tokens.push(Spanned {
                        node: match c {
                            '{' => Token::ExprStart,
                            '%' => Token::BlockStart,
                            '#' => Token::CommentStart,
                            _ => unreachable!(),
                        },
                        span: (pos_current - 1..pos_current + 1).into(),
                    });
                    pos_start = pos_current + 1;
                    State::Default
                }
                _ => State::Default,
            },
            State::BraceRight | State::PercentSign | State::HashSign => match c {
                '}' => {
                    if pos_start < pos_current - 1 {
                        tokens.push(Spanned {
                            node: Token::Other,
                            span: (pos_start..pos_current - 1).into(),
                        });
                    }
                    tokens.push(Spanned {
                        node: match state {
                            State::BraceRight => Token::ExprEnd,
                            State::PercentSign => Token::BlockEnd,
                            State::HashSign => Token::CommentEnd,
                            _ => unreachable!(),
                        },
                        span: (pos_current - 1..pos_current + 1).into(),
                    });
                    pos_start = pos_current + 1;
                    State::Default
                }
                _ => State::Default,
            },
        };
    }

    if pos_start < pos_current + 1 {
        tokens.push(Spanned {
            node: Token::Other,
            span: (pos_start..pos_current + 1).into(),
        });
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr() {
        let source = "{{ x }} }} {{{{  { {  {\n{ } } }\n} }}";
        let tokens = lex(source);
        assert_eq!(
            tokens,
            vec![
                Spanned {
                    node: Token::ExprStart,
                    span: Span { start: 0, end: 2 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 2, end: 5 }
                },
                Spanned {
                    node: Token::ExprEnd,
                    span: Span { start: 5, end: 7 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 7, end: 8 }
                },
                Spanned {
                    node: Token::ExprEnd,
                    span: Span { start: 8, end: 10 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 10, end: 11 }
                },
                Spanned {
                    node: Token::ExprStart,
                    span: Span { start: 11, end: 13 }
                },
                Spanned {
                    node: Token::ExprStart,
                    span: Span { start: 13, end: 15 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 15, end: 34 }
                },
                Spanned {
                    node: Token::ExprEnd,
                    span: Span { start: 34, end: 36 }
                },
            ]
        );
    }

    #[test]
    fn test_block() {
        let source = "{% x %} %} {%{%  { {  {\n{ } } }\n} %}";
        let tokens = lex(source);
        assert_eq!(
            tokens,
            vec![
                Spanned {
                    node: Token::BlockStart,
                    span: Span { start: 0, end: 2 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 2, end: 5 }
                },
                Spanned {
                    node: Token::BlockEnd,
                    span: Span { start: 5, end: 7 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 7, end: 8 }
                },
                Spanned {
                    node: Token::BlockEnd,
                    span: Span { start: 8, end: 10 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 10, end: 11 }
                },
                Spanned {
                    node: Token::BlockStart,
                    span: Span { start: 11, end: 13 }
                },
                Spanned {
                    node: Token::BlockStart,
                    span: Span { start: 13, end: 15 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 15, end: 34 }
                },
                Spanned {
                    node: Token::BlockEnd,
                    span: Span { start: 34, end: 36 }
                },
            ]
        );
    }

    #[test]
    fn test_comment() {
        let source = "{# x #} #} {#{#  { {  {\n{ } } }\n} #}";
        let tokens = lex(source);
        assert_eq!(
            tokens,
            vec![
                Spanned {
                    node: Token::CommentStart,
                    span: Span { start: 0, end: 2 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 2, end: 5 }
                },
                Spanned {
                    node: Token::CommentEnd,
                    span: Span { start: 5, end: 7 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 7, end: 8 }
                },
                Spanned {
                    node: Token::CommentEnd,
                    span: Span { start: 8, end: 10 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 10, end: 11 }
                },
                Spanned {
                    node: Token::CommentStart,
                    span: Span { start: 11, end: 13 }
                },
                Spanned {
                    node: Token::CommentStart,
                    span: Span { start: 13, end: 15 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 15, end: 34 }
                },
                Spanned {
                    node: Token::CommentEnd,
                    span: Span { start: 34, end: 36 }
                },
            ]
        );
    }

    #[test]
    fn test_misc() {
        let source = "#}\n\t\t\t   Hello{#World Test{{ %} {{ {% }} {:#?} {} []";
        let tokens = lex(source);
        assert_eq!(
            tokens,
            vec![
                Spanned {
                    node: Token::CommentEnd,
                    span: Span { start: 0, end: 2 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 2, end: 14 }
                },
                Spanned {
                    node: Token::CommentStart,
                    span: Span { start: 14, end: 16 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 16, end: 26 }
                },
                Spanned {
                    node: Token::ExprStart,
                    span: Span { start: 26, end: 28 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 28, end: 29 }
                },
                Spanned {
                    node: Token::BlockEnd,
                    span: Span { start: 29, end: 31 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 31, end: 32 }
                },
                Spanned {
                    node: Token::ExprStart,
                    span: Span { start: 32, end: 34 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 34, end: 35 }
                },
                Spanned {
                    node: Token::BlockStart,
                    span: Span { start: 35, end: 37 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 37, end: 38 }
                },
                Spanned {
                    node: Token::ExprEnd,
                    span: Span { start: 38, end: 40 }
                },
                Spanned {
                    node: Token::Other,
                    span: Span { start: 40, end: 52 }
                }
            ]
        );
    }
}
