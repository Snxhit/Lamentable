// LEXER SPLITS THE FILE INTO TOKENS.


#[derive(Debug, Clone, PartialEq)]
pub enum TokenList
{
    // Data Types
    Number(f64),
    String(String),
    Identifier(String),
    // Operations
    Plus,
    Sub,
    Mult,
    Div,
    Equals,
    // Keywords
    Let,
    Const,
    Shout,
    // Markers
    BracketL,
    BracketR,
    EOL,
    EOF,
}

pub fn tokenize(input: &str) -> Vec<TokenList>
{
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek()
    {
        match c
        {
            ' ' | '\n' | '\t' | '\r' =>
            {
                chars.next();
            }
            '+' =>
            {
                tokens.push(TokenList::Plus);
                chars.next();
            }
            '-' =>
            {
                tokens.push(TokenList::Sub);
                chars.next();
            }
            '*' =>
            {
                tokens.push(TokenList::Mult);
                chars.next();
            }
            '/' =>
            {
                tokens.push(TokenList::Div);
                chars.next();
            }
            '(' =>
            {
                tokens.push(TokenList::BracketR);
                chars.next();
            }
            ')' =>
            {
                tokens.push(TokenList::BracketL);
                chars.next();
            }
            ',' =>
            {
                tokens.push(TokenList::EOL);
                chars.next();
            }
            'a'..='z' | 'A'..='Z' =>
            {
                let mut ident = String::new();
                while let Some(&ch) = chars.peek()
                {
                    if ch.is_alphanumeric()
                    {
                        ident.push(ch);
                        chars.next();
                    }
                    else
                    {
                        break;
                    }
                }
                match ident.as_str()
                {
                    "shout" => tokens.push(TokenList::Shout),
                    "let" => tokens.push(TokenList::Let),
                    "const" => tokens.push(TokenList::Const),
                    _ => tokens.push(TokenList::Identifier(ident)),
                }
            }
            '0'..='9' =>
            {
                let mut num = String::new();
                while let Some(&ch) = chars.peek()
                {
                    if ch.is_digit(10)
                    {
                        num.push(ch);
                        chars.next();

                    }
                    else if ch == '.'
                    {
                        num.push('.');
                        chars.next();

/*                        while let Some(&d_ch) = chars.peek()
                        {
                            if d_ch.is_digit(10)
                            {
                                num.push(ch);
                                chars.next();
                            }
                            else
                            {
                                break;
                            }
                        }*/
                    }
                    else
                    {
                        break;
                    }
                }
                tokens.push(TokenList::Number(num.parse().unwrap()));
            }
            '=' =>
            {
                tokens.push(TokenList::Equals);
                chars.next();
            }
            '"' =>
            {
                chars.next();
                let mut string_text = String::new();
                while let Some(&st) = chars.peek()
                {
                    if st != '"'
                    {
                        string_text.push(st);
                        chars.next();
                    }
                    else
                    {
                        chars.next();
                        break;
                    }
                }
                tokens.push(TokenList::String(string_text));
            }

            _ =>
            {
                panic!("Unexpected character: {}!", c);
            }
        }
    }

    tokens.push(TokenList::EOF);
    tokens
}
