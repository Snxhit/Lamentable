use crate::lexer::TokenList;
use crate::ast::ASTNode;
use crate::interpreter::Interpreter;
use std::collections::HashMap;

pub struct Parser
{
    tokens: Vec<TokenList>,
    pos: usize,
    strings: HashMap<String, (String, bool)>,
    floats: HashMap<String, (f64, bool)>,
}

impl Parser
{
    pub fn new(tokens: Vec<TokenList>) -> Self
    {
        Self
        {
            tokens,
            pos: 0,
            strings: HashMap::new(),
            floats: HashMap::new(),
        }
    }

    pub fn parse_math_expr(&mut self) -> f64
    {
        let mut result = self.parse_term();

        while let Some(token) = self.current()
        {
            match token
            {
                TokenList::Plus =>
                {
                    self.advance();
                    result += self.parse_term();
                }
                TokenList::Sub =>
                {
                    self.advance();
                    result -= self.parse_term();
                }
                TokenList::EOL | TokenList::BracketL => break,
                _ => panic!("Unexpected token in math expression: {:?}!", token),
            }
        }

        result
    }

    fn parse_term(&mut self) -> f64
    {
        let mut result = self.parse_atom();

        while let Some(token) = self.current()
        {
            match token
            {
                TokenList::Mult =>
                {
                    self.advance();
                    result *= self.parse_atom();
                }
                TokenList::Div =>
                {
                    self.advance();
                    result /= self.parse_atom();
                }
                _ => break
            }
        }

        result
    }

    fn parse_atom(&mut self) -> f64
    {
        match self.current()
        {
            Some(TokenList::Number(n)) =>
            {
                let value = *n;
                self.advance();
                value.into()
            }
            Some(TokenList::BracketR) =>
            {
                self.advance();
                let value = self.parse_math_expr();
                match self.current()
                {
                    Some(TokenList::BracketL) => self.advance(),
                    _ => panic!("Rounded bracket is never closed!"),
                }
                value
            }
            other => panic!("Unexpected token, expected number, got: {:?}!", other),
        }
    }

    fn current(&self) -> Option<&TokenList>
    {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self)
    {
        self.pos += 1;
    }

    pub fn replace_identifiers(&mut self)
    {
        let original_pos = self.pos;
        let mut replaceable_idents = vec![];

        while let Some(token) = self.current()
        {
            if let TokenList::EOL = token
            {
                break;
            }

            if let TokenList::Identifier(name) = token
            {
                if let Some((val, _)) = self.floats.get(name)
                {
                    replaceable_idents.push((self.pos, TokenList::Number(*val)));
                }
                else
                {
                    panic!("Two different data types cannot be added!");
                }
            }

            self.advance();
        }

        for (i, token) in replaceable_idents
        {
            self.tokens[i] = token;
        }

        self.pos = original_pos;
    }

    pub fn parse(&mut self) -> Vec<ASTNode>
    {
        let mut nodes = Vec::new();

        while let Some(token) = self.current()
        {
            match token
            {
                TokenList::Shout =>
                {
                    self.advance();
                    let mut identbool;
                    let val = match self.current().cloned()
                    {
                        Some(TokenList::String(s)) =>
                        {
                            self.advance();
                            identbool = false;
                            s
                        }
                        Some(TokenList::Identifier(t)) =>
                        {
                            if let Some((val, _)) = self.floats.get(t.as_str())
                            {
                                self.replace_identifiers();
                                let x = self.parse_math_expr();
                                identbool = false;
                                self.advance();
                                x.to_string()
                            }
                            else
                            {
                                identbool = true;
                                self.advance();
                                t
                            }
                        }
                        Some(TokenList::Number(_)) | Some(TokenList::BracketR) =>
                        {
                            identbool = false;
                            self.parse_math_expr().to_string()
                        }

                        _ => panic!("Identifier, String (text) or mathematical expression expected after shout!"),
                    };


                    nodes.push(ASTNode::ShoutStatement { val, ident: identbool } );
                }

                TokenList::Let =>
                {
                    self.advance();

                    let datatype = if let Some(TokenList::Identifier(datatype)) = self.current()
                    {
                        let t = datatype.clone();
                        self.advance();
                        t
                    }
                    else
                    {
                        panic!("Expected type of variable after 'let'!");
                    };

                    let name = if let Some(TokenList::Identifier(name)) = self.current()
                    {
                        let n = name.clone();
                        self.advance();
                        n
                    }
                    else
                    {
                        panic!("Expected variable name (Text) after 'let'!");
                    };

                    if self.current() != Some(&TokenList::Equals)
                    {
                        panic!("Equals (=) expected after identifier in let statement!");
                    }
                    self.advance();

                    let val;

                    match datatype.as_str()
                    {
                        "string" =>
                        {
                            val = if let Some(TokenList::String(val)) = self.current()
                            {
                                let t = val.clone();
                                self.advance();
                                t
                            }
                            else
                            {
                                panic!("String or Integer expected after equals!");
                            }
                        }

                        "integer" =>
                        {
                            let token = self.current().cloned();
                            val = match token
                            {
                                Some(TokenList::Number(_)) | Some(TokenList::BracketR) =>
                                {
                                    self.parse_math_expr().to_string()
                                }
                                invtoken => panic!("Invalid token {:?}!", invtoken),
                            }
                        }

                        _ => panic!("Unknown datatype"),
                    }

                    if let Ok(num) = val.parse::<f64>()
                    {
                        if self.strings.get(&name).is_none()
                        {
                            if self.floats.get(&name).is_some()
                            {
                                if let Some((_, constant)) = self.floats.get(&name)
                                {
                                    if *constant == false { self.floats.insert(name.clone(), (num, false)); }
                                    else { panic!("Variable \"{}\" already exists as a constant {}. (Cannot be overwritten!)", name.clone(), datatype); }
                                }
                            }
                            else
                            {
                                self.floats.insert(name.clone(), (num, false));
                            }
                        }
                        else
                        {
                            panic!("{} is already a variable with the datatype {}!", name, datatype);
                        }
                    }

                    if let Err(_) = val.parse::<f64>()
                    {
                        if self.floats.get(&name).is_none()
                        {
                            if self.strings.get(&name).is_some()
                            {
                                if let Some((_, constant)) = self.strings.get(&name)
                                {
                                    if *constant == false { self.strings.insert(name.clone(), (val.clone(), false)); }
                                    else { panic!("Variable \"{}\" already exists as a constant {}. (Cannot be overwritten!)", name.clone(), datatype); }
                                }
                            }
                            else
                            {
                                self.strings.insert(name.clone(), (val.clone(), false));
                            }
                        }
                        else
                        {
                            panic!("{} is already a variable with the datatype {}!", name, datatype);
                        }
                    }

                    nodes.push(ASTNode::LetStatement {datatype, name, val} );
                }

                TokenList::Const =>
                {
                    self.advance();

                    let datatype = if let Some(TokenList::Identifier(datatype)) = self.current()
                    {
                        let t = datatype.clone();
                        self.advance();
                        t
                    }
                    else
                    {
                        panic!("Expected type of variable after 'let'!");
                    };

                    let name = if let Some(TokenList::Identifier(name)) = self.current()
                    {
                        let n = name.clone();
                        self.advance();
                        n
                    }
                    else
                    {
                        panic!("Expected variable name (Text) after 'let'!");
                    };

                    if self.current() != Some(&TokenList::Equals)
                    {
                        panic!("Equals (=) expected after identifier in let statement!");
                    }
                    self.advance();

                    let val;

                    match datatype.as_str()
                    {
                        "string" =>
                        {
                            val = if let Some(TokenList::String(val)) = self.current()
                            {
                                let t = val.clone();
                                self.advance();
                                t
                            }
                            else
                            {
                                panic!("String or Integer expected after equals!");
                            }
                        }

                        "integer" =>
                        {
                            let token = self.current().cloned();
                            val = match token
                            {
                                Some(TokenList::Number(_)) | Some(TokenList::BracketR) =>
                                {
                                    self.parse_math_expr().to_string()
                                }
                                invtoken => panic!("Invalid token {:?}!", invtoken),
                            }
                        }

                        _ => panic!("Unknown datatype"),
                    }

                    if let Ok(num) = val.parse::<f64>()
                    {
                        if self.strings.get(&name).is_none() && self.floats.get(&name).is_none()
                        {
                            self.floats.insert(name.clone(), (num, true));
                        }
                        else if self.strings.get(&name).is_some()
                        {
                            panic!("{} is already a variable with the datatype {}!", name, datatype);
                        }
                        else if self.floats.get(&name).is_some()
                        {
                            panic!("Variable \"{}\" already exists as a constant {}. (Cannot be overwritten!)", name.clone(), datatype);
                        }
                    }

                    if let Err(_) = val.parse::<f64>()
                    {
                        if self.floats.get(&name).is_none() && self.strings.get(&name).is_none()
                        {
                            self.strings.insert(name.clone(), (val.clone(), true));
                        }
                        else if self.floats.get(&name).is_some()
                        {
                            panic!("{} is already a variable with the datatype {}!", name, datatype);
                        }
                        else if self.strings.get(&name).is_some()
                        {
                            panic!("Variable \"{}\" already exists as a constant {}. (Cannot be overwritten!)", name.clone(), datatype);
                        }
                    }

                    nodes.push(ASTNode::ConstStatement {datatype, name, val} );
                }

                TokenList::EOL =>
                {
                    self.advance();
                }

                TokenList::EOF =>
                {
                    break;
                }

                _ =>
                {
                    panic!("Unexpected token: {:?}!", token);
                }
            }
        }

        nodes
    }
}
