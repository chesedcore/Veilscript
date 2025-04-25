#[derive(Debug)]
pub struct Assignment {
    pub name: String,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct Variable {
    pub name: String
}

#[derive(Debug)]
pub enum BinOp {
    add,
    sub,
    mult,
    div
}

#[derive(Debug)]
pub enum Expr {
    FloatLiteral(f64),
    IntLiteral(i64),
    StringLiteral(String),
    Variable(Variable),
    BinaryExpr{
        left: Box<Expr>,
        opcode: BinOp,
        right: Box<Expr>,
    }
}

#[derive(Debug)]
pub enum Ast {
    AssignmentNode(Assignment),
    ExpressionNode(Expr),
}

