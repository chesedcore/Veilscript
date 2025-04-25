use crate::lexer::Token;
use crate::ast::{Ast, Expr, Assignment, Variable, BinOp};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { 
            tokens, 
            pos: 0
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn advance(&mut self) -> Option<&Token> {
        let wrapped_token = self.tokens.get(self.pos);
        self.pos += 1;
        wrapped_token
    }

    pub fn expect(&mut self, expected: TokenType) -> Result<(), String>{
        match self.advance() {
            Some(token) => {
                if token.kind == expected {
                    return Ok();
                }else{
                    return String::from("Token not matched!");
                }
            }

            _ => { return String::from("Nothing found... probably reached EOF?"); }
        }
    }
}


