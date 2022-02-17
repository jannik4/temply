#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
    /// `{{`
    ExprStart,
    /// `}}`
    ExprEnd,

    /// `{%`
    BlockStart,
    /// `%}`
    BlockEnd,

    /// `{#`
    CommentStart,
    /// `#}`
    CommentEnd,

    Other,
}
