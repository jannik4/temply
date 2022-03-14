#[derive(Debug)]
pub struct Ast<'s> {
    pub items: Vec<Item<'s>>,
}

#[derive(Debug)]
pub enum Item<'s> {
    Text(Text<'s>),
    Comment(&'s str),
    Expr(&'s str, &'s str),
    Let(&'s str),
    Scope(Ast<'s>),
    For {
        for_: &'s str,
        pre: Option<Text<'s>>,
        body: Ast<'s>,
    },
    If {
        if_: (&'s str, Ast<'s>),
        else_ifs: Vec<(&'s str, Ast<'s>)>,
        else_: Option<Ast<'s>>,
    },
    Match {
        match_: &'s str,
        wheres: Vec<(&'s str, Ast<'s>)>,
    },
    Macro {
        name: &'s str,
        params: Vec<&'s str>,
        body: Ast<'s>,
    },
    Call {
        name: &'s str,
        args: Vec<&'s str>,
    },
}

#[derive(Debug)]
pub struct Text<'s> {
    pub lines: Vec<Line<'s>>,
    pub trailing: &'s str,
}

#[derive(Debug)]
pub struct Line<'s> {
    pub content: &'s str,
    pub new_line: &'s str,
}
