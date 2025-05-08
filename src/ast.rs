#[derive(Debug)]
pub enum ASTNode
{
    AddStatement { op_one: i32, op_two: i32 },
    ShoutStatement { val: String, ident: bool},
    LetStatement { name: String, val: i32 },
}
