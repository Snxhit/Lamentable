use crate::lexer::TokenList;
use crate::ast::ASTNode;
//use crate::parser::Parser as NestedParser;
use crate::interpreter::Interpreter;
use std::collections::HashMap;

pub struct Parser
{
    tokens: Vec<TokenList>,
    pos: usize,
    strings: HashMap<String, (String, bool)>,
    floats: HashMap<String, (f64, bool)>,
    bools: HashMap<String, (String, bool)>,
}

impl Parser
{
    pub fn new(tokens: Vec<TokenList>, strings: HashMap<String, (String, bool)>, floats: HashMap<String, (f64, bool)>, bools: HashMap<String, (String, bool)>) -> Self
    {
        Self
        {
            tokens,
            pos: 0,
            strings,
            floats,
            bools,
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
//       println!("Parsing session has started -------------");
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
                        "str" =>
                        {
                            val = if let Some(TokenList::String(val)) = self.current()
                            {
                                let t = val.clone();
                                self.advance();
                                t
                            }
                            else
                            {
                                panic!("String, integer or boolean expected after equals!");
                            }
                        }

                        "num" =>
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

                        "bool" =>
                        {
                            val = if let Some(TokenList::Boolean(x)) = self.current()
                            {
                                let t = x.clone();
                                self.advance();
                                match t
                                {
                                    true => "True".to_string(),
                                    false => "False".to_string(),
                                }
                            }
                            else
                            {
                                panic!("String, integer or boolean expected after equals!");
                            }
                        }

                        _ => panic!("Unknown datatype"),
                    }

                    if let Ok(num) = val.parse::<f64>()
                    {
                        if self.strings.get(&name).is_none() && self.bools.get(&name).is_none()
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
                            panic!("{} is already a variable with another datatype!", name);
                        }
                    }

                    if let Err(_) = val.parse::<f64>()
                    {
                        if self.floats.get(&name).is_none() && self.bools.get(&name).is_none()
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
                                if val == "True".to_string() || val == "False"
                                {
                                    self.bools.insert(name.clone(), (val.clone(), false));
                                }
                                else
                                {
                                    self.strings.insert(name.clone(), (val.clone(), false));
                                }
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
                        panic!("Expected type of constant after 'let'!");
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
                        "str" =>
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

                        "num" =>
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

                        "bool" =>
                        {
                            val = if let Some(TokenList::Boolean(x)) = self.current()
                            {
                                let t = x.clone();
                                self.advance();
                                match t
                                {
                                    true =>
                                    {
                                        self.bools.insert(name.clone(), ("True".to_string(), true));
                                        "True".to_string()
                                    }
                                    false =>
                                    {
                                        self.bools.insert(name.clone(), ("False".to_string(), true));
                                        "False".to_string()
                                    }
                                }
                            }
                            else
                            {
                                panic!("String, integer or boolean expected after equals!");
                            }

                        }
                        _ => panic!("Unknown datatype"),
                    }

                    if let Ok(num) = val.parse::<f64>()
                    {
                        if self.strings.get(&name).is_none() && self.floats.get(&name).is_none() && self.bools.get(&name).is_none()
                        {
                            self.floats.insert(name.clone(), (num, true));
                        }
                        else if self.strings.get(&name).is_some() || self.bools.get(&name).is_some()
                        {
                            panic!("{} is already a variable with a different datatype!", name);
                        }
                        else if self.floats.get(&name).is_some()
                        {
                            panic!("Variable \"{}\" already exists as a constant {}. (Cannot be overwritten!)", name.clone(), datatype);
                        }
                    }

                    if let Err(_) = val.parse::<f64>()
                    {
                        if self.floats.get(&name).is_none() && self.strings.get(&name).is_none() && self.strings.get(&name).is_none()
                        {
                            self.strings.insert(name.clone(), (val.clone(), true));
                        }
                        else if self.floats.get(&name).is_some() || self.bools.get(&name).is_some()
                        {
                            panic!("{} is already a variable with a different datatype!", name);
                        }
                        else if self.strings.get(&name).is_some()
                        {
                            panic!("Variable \"{}\" already exists as a constant {}. (Cannot be overwritten!)", name.clone(), datatype);
                        }
                    }

                    nodes.push(ASTNode::ConstStatement {datatype, name, val} );
                }

                TokenList::If =>
                {
                    self.advance();

                    let temptoken = self.current().cloned();
                    let eval = if let Some(TokenList::BracketR) = temptoken
                    {
                        self.advance();

                        let temptoken = self.current().cloned();
                        if let Some(TokenList::Boolean(x)) = temptoken
                        {
                            self.advance();

                            let temptoken = self.current().cloned();
                            if let Some(TokenList::BracketL) = temptoken
                            {
                                self.advance();
                                x
                            }
                            else
                            {
                                panic!("Expected ')' after opening '(' brackets for if statement conditionals!");
                            }
                        }
                        else if let Some(TokenList::Identifier(name)) = temptoken
                        {
                            self.advance();
                            if self.bools.get(&name).is_some()
                            {
                                let temptoken = self.current().cloned();
                                if Some(TokenList::Comparison) == temptoken
                                {
                                    self.advance();
                                    let temptoken = self.current().cloned();
                                    if let Some(TokenList::Boolean(b)) = temptoken
                                    {
                                        self.advance();
                                        self.advance();
                                        self.bools.get(&name).unwrap().0.to_lowercase() == b.to_string()
                                    }
                                    else
                                    {
                                        panic!("The provided value in the boolean field is not a valid boolean!");
                                    }
                                }
                                else if Some(TokenList::NEComparison) == temptoken
                                {
                                    self.advance();
                                    let temptoken = self.current().cloned();
                                    if let Some(TokenList::Boolean(b)) = temptoken
                                    {
                                        self.advance();
                                        self.advance();
                                        self.bools.get(&name).unwrap().0.to_lowercase() != b.to_string()
                                    }
                                    else
                                    {
                                        panic!("The provided value in the boolean field is not a valid boolean!");
                                    }
                                }
                                else
                                {
                                    panic!("Comparison operator expected after identifier!");
                                }
                            }
                            else if self.floats.get(&name).is_some()
                            {
                                let temptoken = self.current().cloned();
                                if Some(TokenList::Comparison) == temptoken
                                {
                                    self.advance();
                                    let temptoken = self.current().cloned();
                                    if let Some(TokenList::Number(n)) = temptoken
                                    {
                                        self.replace_identifiers();
                                        let val = self.parse_math_expr();
                                        while true
                                        {
                                            let temptoken = self.current().cloned();
                                            if temptoken == Some(TokenList::BracketR)
                                            {
                                                break;
                                            }
                                            else
                                            {
                                                self.advance();
                                            }
                                        }
                                        self.floats.get(&name).unwrap().0 == val
                                    }
                                    else if let Some(TokenList::Identifier(x)) = temptoken
                                    {
                                        self.replace_identifiers();
                                        let val = self.parse_math_expr();
                                        while true
                                        {
                                            let temptoken = self.current().cloned();
                                            if temptoken == Some(TokenList::BracketR)
                                            {
                                                break;
                                            }
                                            else
                                            {
                                                self.advance();
                                            }
                                        }
                                        self.floats.get(&name).unwrap().0 == val
                                    }
                                    else if let Some(TokenList::BracketR) = temptoken
                                    {
                                        self.replace_identifiers();
                                        let val = self.parse_math_expr();
                                        while true
                                        {
                                            let temptoken = self.current().cloned();
                                            if temptoken == Some(TokenList::BracketR)
                                            {
                                                break;
                                            }
                                            else
                                            {
                                                self.advance();
                                            }
                                        }
                                        self.floats.get(&name).unwrap().0 == val
                                    }
                                    else
                                    {
                                        panic!("The provided value in the integer field is not a valid integer!")
                                    }
                                }
                                else if Some(TokenList::NEComparison) == temptoken
                                {
                                    self.advance();
                                    let temptoken = self.current().cloned();
                                    if let Some(TokenList::Number(n)) = temptoken
                                    {
                                        self.advance();
                                        self.advance();
                                        self.floats.get(&name).unwrap().0 != n
                                    }
                                    else
                                    {
                                        panic!("The provided value in the integer field is not a valid integer!")
                                    }
                                }
                                else
                                {
                                    panic!("Comparison operator expected after identifier");
                                }
                            }
                            else if self.strings.get(&name).is_some()
                            {
                                let temptoken = self.current().cloned();
                                if Some(TokenList::Comparison) == temptoken
                                {
                                    self.advance();
                                    let temptoken = self.current().cloned();
                                    if let Some(TokenList::String(s)) = temptoken
                                    {
                                        self.advance();
                                        self.advance();
                                        self.strings.get(&name).unwrap().0 == s
                                    }
                                    else
                                    {
                                        panic!("The provided value is not a valid string!");
                                    }
                                }
                                else if Some(TokenList::NEComparison) == temptoken
                                {
                                    self.advance();
                                    let temptoken = self.current().cloned();
                                    if let Some(TokenList::String(s)) = temptoken
                                    {
                                        self.advance();
                                        self.advance();
                                        self.strings.get(&name).unwrap().0 != s
                                    }
                                    else
                                    {
                                        panic!("The provided value is not a valid string!");
                                    }
                                }
                                else
                                {
                                    panic!("Comparison expected after if statement!");
                                }
                            }
                            else
                            {
                                panic!("Provided identifier is not a valid variable name!");
                            }
                        }
                        else
                        {
                            panic!("Boolean value expected as if statement conditional!");
                        }
                    }
                    else
                    {
                        panic!("Opening bracket '(' expected after if keyword!");
                    };
                    if eval
                    {
                        let temptoken = self.current().cloned();
                        let loop_tokens = if let Some(TokenList::BracketR) = temptoken
                        {
                            let mut nest_number = 0;
                            let mut loop_data = Vec::new();
                            self.advance();
                            while true
                            {
                                let temptoken = self.current().cloned();
//                                println!("{:?}", temptoken.clone().unwrap());
                                match temptoken
                                {
                                    Some(TokenList::BracketR) =>
                                    {
                                        loop_data.push(TokenList::BracketR);
                                        nest_number = nest_number + 1;
                                    }
                                    Some(TokenList::BracketL) =>
                                    {
                                        if nest_number != 0
                                        {
                                            nest_number = nest_number - 1;
                                            loop_data.push(TokenList::BracketL);
                                        }
                                        else if nest_number == 0
                                        {
//                                            self.advance();
//                                            loop_data.push(TokenList::BracketL);
                                            break;
                                        }
                                    },
                                    Some(token) =>
                                    {
                                        loop_data.push(token);
                                    },
                                    None => {}
                                }
                                self.advance();
                            }

                            self.advance();
                            loop_data
                        }
                        else
                        {
                            panic!("Opening bracket '(' expected at the start of if block!");
                        };

                        use crate::parser::Parser as NestedParser;
                        for token in &loop_tokens
                        {
//                            println!("{:?}", token);
                        }
                        let mut parser = NestedParser::new(loop_tokens, self.strings.clone(), self.floats.clone(), self.bools.clone());
                        let loop_AST = parser.parse();

                        nodes.push(ASTNode::IfStatement{ tree: loop_AST });

/*                        use crate::interpreter::Interpreter;
                        let mut interpreter = Interpreter::new();
                        interpreter.interpret(loop_AST);*/

                    }
                    else
                    {
                        while true
                        {
                            if let Some(TokenList::BracketL) = self.current()
                            {
                                break;
                            }
                            self.advance();
                        }
                        self.advance();
                    }
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

//        println!("Parsing session has ended -------------");
        nodes
    }
}
