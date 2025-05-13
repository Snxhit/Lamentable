use std::collections::HashMap;
use crate::ast::ASTNode;

pub struct Interpreter
{
    strings: HashMap<String, String>,
    floats: HashMap<String, f64>,
}

impl Interpreter
{
    pub fn new() -> Self
    {
        Self
        {
            strings: HashMap::new(),
            floats: HashMap::new(),
        }
    }

    pub fn get_strings(&self) -> &HashMap<String, String>
    {
        &self.strings
    }
    pub fn get_floats(&self) -> &HashMap<String, f64>
    {
        &self.floats
    }

    pub fn interpret(&mut self, nodes: Vec<ASTNode>)
    {
        for node in nodes
        {
            match node
            {
                ASTNode::ShoutStatement { val, ident } =>
                {
                    if ident
                    {
                        if let Some(val) = self.strings.get(&val)
                        {
                            println!("{}", val);
                        }
                        else if let Some(val) = self.floats.get(&val)
                        {
                            println!("{}", val);
                        }
                        else
                        {
                            panic!("Provided identifier was not a valid variable name, please consider encasing it in double quotes!");
                        }
                    }
                    else
                    {
                        println!("{}", val);
                    }
                }

                ASTNode::LetStatement { datatype, name, val } =>
                {
                    if let Ok(num) = val.parse::<f64>()
                    {
                        if self.strings.get(&name).is_none()
                        {
                            self.floats.insert(name.clone(), num);
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
                            self.strings.insert(name, val);
                        }
                        else
                        {
                            panic!("{} is already a variable with the datatype {}!", name, datatype);
                        }
                    }
                }

                ASTNode::ConstStatement { datatype, name, val } =>
                {
                    if let Ok(num) = val.parse::<f64>()
                    {
                        if self.strings.get(&name).is_none() && self.floats.get(&name).is_none()
                        {
                            self.floats.insert(name.clone(), num);
                        }
                        else
                        {
                            panic!("{} is a constant (non-reassignable) with the datatype {}!", name, datatype);
                        }
                    }
                    if let Err(_) = val.parse::<f64>()
                    {
                        if self.floats.get(&name).is_none() && self.strings.get(&name).is_none()
                        {
                            self.strings.insert(name, val);
                        }
                        else
                        {
                            panic!("{} is a constant (non-reassignable) with the datatype {}!", name, datatype);
                        }
                    }
                }


                _ => panic!("Invalid ASTNode"),
            }
        }
    }
}


