use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            chars: source.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.chars.next() {
            match ch {
                ' ' | '\n' => continue,
                '+' => tokens.push(Token::Plus),

                '0'..='9' => {
                    let mut num_str = ch.to_string();

                    while let Some(&next_ch) = self.chars.peek() {
                        if next_ch.is_ascii_digit() {
                            num_str.push(self.chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    let val = num_str.parse::<i64>().unwrap();
                    tokens.push(Token::Number((val)));
                }

                _ => panic!("Lexing Error: Unknown character {}", ch),
            }
        }

        tokens.push(Token::EOF);
        tokens
    }
}