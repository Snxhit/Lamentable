use crate::lexer::TokenList;
use crate::ast::ASTNode;

pub struct Parser
{
    tokens: Vec<TokenList>,
    pos: usize,
}

impl Parser
{
    pub fn new(tokens: Vec<TokenList>) -> Self
    {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&TokenList>
    {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self)
    {
        self.pos += 1;
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

                    let text = if let Some(TokenList::Identifier(text)) = self.current()
                    {
                        let t = text.clone();
                        self.advance();
                        t
                    }
                    else
                    {
                        panic!("Expected identifier (Text) after 'shout'!");
                    };

                    nodes.push(ASTNode::ShoutStatement { text } );
                }

                TokenList::Let =>
                {
                    self.advance();

                    let name = if let Some(TokenList::Identifier(name)) = self.current()
                    {
                        let n = name.clone();
                        self.advance();
                        n
                    }
                    else
                    {
                        panic!("Expected identifier (Text) after 'let'!");
                    };

                    if self.current() != Some(&TokenList::Equals)
                    {
                        panic!("Equals expected after identifier in let statement!");
                    }
                    self.advance();

                    let val = if let Some(TokenList::Number(val)) = self.current()
                    {
                        let v = *val;
                        self.advance();
                        v
                    }
                    else
                    {
                        panic!("Number expected after equals sign in let statement!");
                    };

                    nodes.push(ASTNode::LetStatement {name, val} );
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
