#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i64),
    Plus,
    EOF,
}