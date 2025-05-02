#![allow(non_camel_case_types)]
//because i'm coming from python and gdscript. enum variants are SCREAMING_SNAKE_CASE AND YOU CAN'T
//CONVINCE ME OTHERWISE!!! GRRAHHH

use crate::lexer::TokenType;

///TOKENS, EXPRESSIONS AND IDENTS 


///IDENTIFIER section
//this here is an IDENTIFIER. It serves one purpose: to hold the name to variables and function
//calls (and more in the future). Right now, it is only a type wrapper around a string, which is
//fine for the purpose it serves.
#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String
}

///OPERATOR section
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

#[derive(Debug)]
pub enum MonOp {
    POS,
    NEG
}

impl MonOp {
    pub fn get_precedence(&self) -> u8 {3}
    pub fn to_string(&self) -> String {
        match self {
            MonOp::POS => "+".to_string(),
            MonOp::NEG => "-".to_string(),
        }
    }
}

///ATOM section
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

///FN CALL section 
//my ass is NOT explaining this 
#[derive(Debug)]
pub struct FnCall {
    pub ident: Ident,
    pub args: Box<Vec<Expr>>
}

impl FnCall {
    pub fn to_pretty_string(&self) -> String {
        let mut ret = String::from(&self.ident.name);
        ret += "(";
        for arg in self.args.as_ref() {
            ret += &format!("[{}]", arg.to_pretty_string());
        }
        ret += ")";
        ret
    }
}

///EXPR section
//this here is an EXPR(expression) enum. It represents either an ATOMIC EXPRESSION (an expression
//that cannot be divided anymore) or a BINARY OPERATION (like 2+3 or 1-var) or a SCOPE
#[derive(Debug)]
pub enum Expr {
    ATOM(Atom),
    GROUPED_EXPR(Box<Expr>),
    BINARY_EXPR {
        left: Box<Expr>,
        opcode: BinOp,
        right: Box<Expr>,
    },
    UNARY_EXPR {
        opcode: MonOp,
        expr: Box<Expr>
    },
    SCOPE(Scope),
    FUNCTION_CALL(FnCall),
}

impl Expr {
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
            Expr::UNARY_EXPR{opcode,expr} => format!("({}{})",opcode.to_string(),expr.to_pretty_string()),
            Expr::SCOPE(scope) => scope.to_pretty_string(),
            Expr::FUNCTION_CALL(fncall) => fncall.to_pretty_string(),
        }
    }
}

///ASSIGNMENT section
//An assignment assigns the EXPR on the RIGHT into the IDENT on the LEFT
#[derive(Debug)]
pub struct Assignment {
    pub ident: Ident,
    pub type_t: Option<TokenType>, //there may not be a type given! in which case, infer it
    pub expr: Box<Expr>
}

///PARAMETER section
//A parameter dictates an IDENTIFIER associated with a method/function along with its TYPE 
#[derive(Debug)]
pub struct Parameter {
    pub ident: Ident,
    pub type_t: TokenType, //declaring a type in parameters is an absolute must.
}
impl Parameter {
    pub fn to_pretty_string(params: &[Parameter]) -> String {
        let mut ret = String::new();
        for param in params {
            ret += &format!("[{}: {:?}]", param.ident.name, param.type_t)
        }
        ret
    }
}

///FUNCTION DECLARATION section
//a function declaration DECLARES that the next scope is a reusable block of statements 
#[derive(Debug)]
pub struct FnDeclaration {
    pub ident: Ident, 
    pub type_t: TokenType, //the return type. unassigned implies TYPE_VOID
    pub params: Vec<Parameter>
}
impl FnDeclaration {
    pub fn to_pretty_string(&self) -> String {
        format!("{}({}) -> {:?}", self.ident.name, Parameter::to_pretty_string(&self.params), self.type_t)
    }
}

///RETURN section 
//do i really need to explain tf this is :sob:
#[derive(Debug)]
pub struct ReturnStmt {
    pub expr: Box<Expr>
}

///STATEMENT section
//a statement is a full, higher level constructs that include ASSIGNMENTS, FUNCTION CALLS or
//CONTROL statements.
#[derive(Debug)]
pub enum Stmt {
    STATEMENT_ASSIGNMENT(Assignment),
    STATEMENT_FUNCTION_DECLARATION(FnDeclaration),
    STATEMENT_ZERO_EFFECT,
    STATEMENT_RETURN(ReturnStmt),
    STATEMENT_FUNCTION_CALL(FnCall),
    SCOPE(Scope)
}

impl Stmt {
    pub fn to_pretty_string(&self) -> String {
        match self {
            Stmt::STATEMENT_ZERO_EFFECT => "ZERO-EFFECT".to_string(),
            Stmt::STATEMENT_FUNCTION_DECLARATION(decl) => decl.to_pretty_string(),
            Stmt::STATEMENT_ASSIGNMENT(Assignment{ident,type_t, expr}) => {
                format!("{}:{:?} = {}",ident.name, type_t, expr.to_pretty_string())
            },
            Stmt::STATEMENT_RETURN(ret) => ret.expr.to_pretty_string(),
            Stmt::SCOPE(scope) => scope.to_pretty_string(),
            Stmt::STATEMENT_FUNCTION_CALL(fncall) => fncall.to_pretty_string(),
        }
    }
}

///SCOPE section
//A SCOPE defines a collective lifetime for all variables defined within itself
#[derive(Debug)]
pub struct Scope {
    pub stmts: Vec<Stmt>
}
impl Scope {
    pub fn to_pretty_string(&self) -> String {
        self.to_pretty_string_with_indent(0)
    }

    fn to_pretty_string_with_indent(&self, indent_level: usize) -> String {
        let indent = "   ".repeat(indent_level);
        let mut ret = format!("{indent}Scope{{\n");

        for stmt in &self.stmts {
            match stmt {
                Stmt::SCOPE(scope) => {
                    //recurse with deeper indent
                    ret += &scope.to_pretty_string_with_indent(indent_level + 1);
                },
                _ => {
                    //just recurse lol
                    let stmt_str = stmt.to_pretty_string();
                    ret += &format!("{}   {}\n", indent, stmt_str);
                }
            }
        }

        ret += &format!("{indent}}}\n");
        ret
    }
}

