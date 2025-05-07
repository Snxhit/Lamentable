#[derive(Debug)]
pub enum ASTNode
{
    AddStatement { op_one: i32, op_two: i32 },
    ShoutStatement { text: String },
    LetStatement { name: String, val: i32 },
}
