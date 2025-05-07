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
                ASTNode::ShoutStatement { text } =>
                {
                    if let Some(val) = self.variables.get(&text)
                    {
                        println!("{}", val);
                    }
                    else
                    {
                        println!("{}", text);
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


