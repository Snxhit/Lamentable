#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Ident(String),
    Let,
    Const,
    Shout,
    If,
    Plus,
    Minus,
    Mult,
    Div,
    Assign,
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }
    EOF,
}