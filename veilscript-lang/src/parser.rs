use crate::lexer::TokenType;
use crate::lexer::Token;

pub struct Parser<'a> {
    pub tokens: Vec<Token<'a>>,
    pub pos: usize
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser { 
            tokens, 
            pos: 0
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    
    //converts the Option from peek into a Result<&Token,String>
    pub fn peek_and_extract(&self) -> Result<Token, String> {
        match self.peek() {
            Some(t) => Ok(t.clone()),
            None => Err("Unexpected end of input!".to_string()),
        }
    }
    
    //peeks, moves a step ahead, then returns a token.
    ///why it's useful:
    //what you do is...
    //peek -> check if correct token -> move a step ahead -> repeat
    //what you can do is...
    //check if advance() is correct -> repeat
    pub fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }
    
    //converts the option from advance into a result. recommended use this over advance
    pub fn advance_and_extract(&mut self) -> Result<Token, String> {
        match self.advance() {
            Some(t) => Ok(t.clone()),
            None => Err("Unexpected end of input!".to_string()),
        }
    }
    
    //check if the token thrown into it matches the tokenkind. mostly used as a helper func, dont
    //need to call it.
    pub fn check_for(comparator: Token, expected: TokenType) -> Result<TokenType,String> {
        if comparator.kind == expected {
            Ok(expected)
        } else {
            Err(format!("Expected {:?}, but found {:?} instead!", expected, comparator.kind))
        }
    } 
    
    //grouped version of the previous func. checks if the token thrown in is part of the slice.
    pub fn check_contains(comparator: Token, expected: &[TokenType]) -> Result<TokenType, String> {
        if expected.contains(&comparator.kind) {
            Ok(comparator.kind.clone())
        } else {
            Err(format!("Expected one of {:?}, but found {:?} instead!", expected, comparator.kind))
        }
    } 
    
    //checks if the next token is the expected tokentype. good for lookaheads. use this often
    pub fn check_next(&self, expected: TokenType) -> Result<TokenType, String> {
        Parser::check_for(self.peek_and_extract()?, expected)
    }
    
    //checks if the next token is the expected tokentype while stepping forward.
    ///great function, and you will use this all the time.
    ///why is that?:
    //you can relegate the peek -> check -> advance cycle almost COMPLETELY to this method.
    pub fn check_advance(&mut self, expected: TokenType) -> Result<TokenType, String> {
        let token = self.advance_and_extract()?;
        Parser::check_for(token, expected)
    }

    //grouped version of the method before. 
    ///as great as the previous function, but a bit more limited in usage. used to decide branches.
    pub fn check_advance_contains(&mut self, expected: &[TokenType]) -> Result<TokenType, String> {
        let token = self.advance_and_extract()?;
        Parser::check_contains(token, expected)
    }

}

