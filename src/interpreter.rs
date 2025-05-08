use std::collections::HashMap;
use crate::ast::ASTNode;

pub struct Interpreter
{
    variables: HashMap<String, i32>,
}

impl Interpreter
{
    pub fn new() -> Self
    {
        Self
        {
            variables: HashMap::new(),
        }
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
                        if let Some(val) = self.variables.get(&val)
                        {
                            println!("{}", val);
                        }
                        else
                        {
                            panic!("Provided identifier was not a valid variable name, please consider encasing it in double quotes!")
                        }
                    }
                    else
                    {
                        println!("{}", val);
                    }
                }

                ASTNode::LetStatement { name, val } =>
                {
                    self.variables.insert(name, val);
                }

                _ => panic!("Invalid ASTNode"),
            }
        }
    }
}


