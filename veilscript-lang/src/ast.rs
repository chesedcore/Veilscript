#![allow(non_camel_case_types)]

#[derive(Debug)]
pub struct Ident {
    pub name: String
}

#[derive(Debug)]
pub enum BinOp {
    ADD,
    SUB,
    MULT,
    DIV,
}

#[derive(Debug)]
pub enum Atom {
    LITERAL_FLOAT(f64),
    LITERAL_INT(i64),
    LITERAL_STRING(String),
    IDENTIFIER(Ident),
}

#[derive(Debug)]
pub enum Expr {
    ATOM(Atom),
    BINARY_OP {
        left: Box<Expr>,
        opcode: BinOp,
        right: Box<Expr>,
    }
}

#[derive(Debug)]
pub enum Ast {
    NODE_EXPRESSION(Expr),
}

