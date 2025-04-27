#![allow(non_camel_case_types)]
//because i'm coming from python and gdscript. enum variants are SCREAMING_SNAKE_CASE AND YOU CAN'T
//CONVINCE ME OTHERWISE!!! GRRAHHH


///TOKENS, EXPRESSIONS AND IDENTS 

//this here is an IDENTIFIER. It serves one purpose: to hold the name to variables and function
//calls (and more in the future). Right now, it is only a type wrapper around a string, which is
//fine for the purpose it serves.
#[derive(Debug)]
pub struct Ident {
    pub name: String
}

//this here is a BINARY OPERATOR enum. It represents these operators: +,-,*,/. Support for more
//operators is intended to be added in the future, if i can get my lazy ass to push further, lmao 
#[derive(Debug)]
pub enum BinOp {
    ADD,
    SUB,
    MULT,
    DIV,
}

impl BinOp {
    pub fn get_precedence(&self) -> u8 { //decides operator precedence to be utilised in parser.rs
        match self {
            BinOp::ADD | BinOp::SUB => 1,
            BinOp::MULT | BinOp::DIV => 2,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            BinOp::ADD => "+".to_string(),
            BinOp::SUB => "-".to_string(),
            BinOp::MULT => "*".to_string(),
            BinOp::DIV => "/".to_string(),
        }
    }
}


//this here is an ATOM enum. It represents the smallest, most indivisible part of the source, and is
//comprised of a LITERAL (like strings or floats) or an IDENTIFIER(like a variable or function name)
#[derive(Debug)]
pub enum Atom {
    LITERAL_FLOAT(f64),
    LITERAL_INT(i64),
    LITERAL_STRING(String),
    IDENTIFIER(Ident),
}

impl Atom {
    pub fn to_string(&self) -> String {
        match self {
            Atom::LITERAL_INT(val) => val.to_string(),
            Atom::LITERAL_FLOAT(val) => val.to_string(),
            Atom::LITERAL_STRING(val) => format!(r"{}", val),
            Atom::IDENTIFIER(ident) => ident.name.clone(),
        }
    }
}

//this here is an EXPR(expression) enum. It represents either an ATOMIC EXPRESSION (an expression
//that cannot be divided anymore) or a BINARY OPERATION (like 2+3 or 1-var)
#[derive(Debug)]
pub enum Expr {
    ATOM(Atom),
    GROUPED_EXPR(Box<Expr>),
    BINARY_EXPR {
        left: Box<Expr>,
        opcode: BinOp,
        right: Box<Expr>,
    }
}

impl Expr {
    pub fn is_operator(&self) -> bool { //used for parser.rs 
        match self {
            Expr::BINARY_EXPR{left:_, opcode:_, right:_} => true,
            _ => false,
        }
    }

    pub fn to_pretty_string(&self) -> String {
        match self {
            Expr::ATOM(atom) => atom.to_string(),
            Expr::BINARY_EXPR { left, opcode, right } => {
                format!(
                    "({} {} {})",
                    left.to_pretty_string(),
                    opcode.to_string(),
                    right.to_pretty_string()
                )
            }
            Expr::GROUPED_EXPR(inner) => format!("({})", inner.to_pretty_string()), 
        }
    }
}

///ABSTRACT SYNTAX TREE (AST)

//this here is an AST NODE. when our parser fully finishes parsing our code, it spits out an AST NODE
//that will be later interpreted and executed. right now, we assume the built tree is syntactically
//valid.
#[derive(Debug)]
pub enum Ast {
    NODE_EXPRESSION(Expr),
}






