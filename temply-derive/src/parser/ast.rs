#[derive(Debug)]
pub struct Ast<'s> {
    pub items: Vec<Item<'s>>,
}

#[derive(Debug)]
pub enum Item<'s> {
    Text(&'s str),
    Comment(&'s str),
    Expr(&'s str, &'s str),
    Let(&'s str),
    Scope(Ast<'s>),
    For {
        for_: &'s str,
        pre: Option<&'s str>,
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
}
