#[derive(Debug)]
pub enum ASTNode
{
    ShoutStatement { val: String, ident: bool},
    LetStatement { datatype: String, name: String, val: String }, // val is converted to f64 later on.
    ConstStatement { datatype: String, name: String, val: String },
    IfStatement { tree: Vec<ASTNode> },
}
