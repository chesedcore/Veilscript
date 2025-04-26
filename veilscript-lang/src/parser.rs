use crate::lexer::Token;
use crate::ast::*;

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
                    Ok(())
                }else{
                    Err(format!("Expected {:?}, but found {:?}", expected, token.kind))
                }
            }

            _ => Err("Nothing found... probably reached EOF?".to_string())
        }
    }

    ///general parsing functions

    pub fn parse_atom(&mut self) -> Result<Atom, String> {

        //peek -> Option<&Token>
        match self.peek() {

            None => Err("Unexpected end of input... eh, EOF, probably?".to_string()),
            Some(token) => {
                //token: &Token { &lexeme: &str , &kind: &Result<TokenType, String> }
                //this is where i parse all the "primary" types 
                match &token.kind {

                    //parse INTEGERS
                    Ok(TokenType::LITERAL_INT) => {
                        let lexeme = token.lexeme;
                        let parsed_value = lexeme.parse::<i64>()
                            .map_err(|_| "Invalid int! Too big for an i64, perhaps?".to_string())?;
                        self.advance(); 
                        Ok(Atom::LITERAL_INT(parsed_value))
                    },

                    //parse FLOATS
                    Ok(TokenType::LITERAL_FLOAT) => {
                        let lexeme = token.lexeme;
                        let parsed_value = lexeme.parse::<f64>()
                            .map_err(|_| "Invalid float! Too wonky for an f64, perhaps?".to_string())?;
                        self.advance();
                        Ok(Atom::LITERAL_FLOAT(parsed_value))
                    },

                    //parse STRINGS
                    Ok(TokenType::LITERAL_STRING) => {
                        let lexeme = token.lexeme;
                        let parsed_value = lexeme.to_owned();
                        self.advance();
                        Ok(Atom::LITERAL_STRING(parsed_value))
                    },
                    
                    //parse IDENTIFIERS
                    Ok(TokenType::IDENTIFIER) => {
                        let lexeme = token.lexeme;
                        let parsed_value = lexeme.to_owned();
                        self.advance();
                        Ok(Atom::IDENTIFIER(Ident{name:parsed_value}))
                    },

                    _ => Err("Unrecognised token... what the hell? Fix this shit.".to_string()),
                }
            }
        }

    }
}


