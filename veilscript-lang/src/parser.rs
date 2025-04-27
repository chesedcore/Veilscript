use crate::lexer::TokenType;
use crate::lexer::Token;
use crate::ast::*;

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

    pub fn advance(&mut self) -> Option<&Token> {
        let wrapped_token = self.tokens.get(self.pos);
        self.pos += 1;
        wrapped_token
    }

    /*pub fn expect(&mut self, expected: TokenType) -> Result<(), String>{
        match self.advance() {
            Some(token) => {
                if token.kind == expected{
                    Ok(())
                }else{
                    Err(format!("Expected {:?}, but found {:?}", expected, token.kind))
                }
            }

            _ => Err("Nothing found... probably reached EOF?".to_string())
        }
    }*/

    ///general parsing functions

    pub fn parse_atom(&mut self) -> Result<Atom, String> {

        //peek -> Option<&Token>
        match self.peek() {

            None => Err("Unexpected end of input... eh, EOF, probably?".to_string()),
            Some(token) => {
                //token: &Token { &lexeme: &str , &kind: &TokenType }
                //this is where i parse all the "primary" types 
                match &token.kind {

                    //parse INTEGERS
                    TokenType::LITERAL_INT => {
                        let lexeme = token.lexeme;
                        let parsed_value = lexeme.parse::<i64>()
                            .map_err(|_| "Invalid int! Too big for an i64, perhaps?".to_string())?;
                        self.advance(); 
                        Ok(Atom::LITERAL_INT(parsed_value))
                    },

                    //parse FLOATS
                    TokenType::LITERAL_FLOAT => {
                        let lexeme = token.lexeme;
                        let parsed_value = lexeme.parse::<f64>()
                            .map_err(|_| "Invalid float! Too wonky for an f64, perhaps?".to_string())?;
                        self.advance();
                        Ok(Atom::LITERAL_FLOAT(parsed_value))
                    },

                    //parse STRINGS
                    TokenType::LITERAL_STRING => {
                        let lexeme = token.lexeme;
                        let parsed_value = lexeme.to_owned();
                        self.advance();
                        Ok(Atom::LITERAL_STRING(parsed_value))
                    },
                    
                    //parse IDENTIFIERS
                    TokenType::IDENTIFIER => {
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

    pub fn parse_group_or_atom(&mut self) -> Result<Expr, String> { //parses brackets properly

        //look into the next token...
        match self.peek() {
            //is it a valid token?
            //if it is, then...
            Some(token) => {
                //what is the token made of?
                match &token.kind {
                    //if it's a left parenthesis -> (
                    TokenType::LPAREN => {
                        //start parsing the next expression inside it!
                        self.advance(); //move past the '('
                        let expr = self.parse_expr(0)?;
                        //after we find the expr, time to check if the right paren exists...
                        match self.peek() {
                            //hit! rparen found!
                            Some(token) if token.kind == TokenType::RPAREN => {
                                self.advance(); //get past the rparen 
                                Ok(Expr::GROUPED_EXPR(Box::new(expr)))
                            },
                            //no hit. malformed brackets.
                            _ => Err("Malformed brackets... no ')' found!".to_string())
                        }
                    },
                    
                    //anything but the lparen -> proceed normally.
                    _ => {
                        let atom = self.parse_atom()?;
                        Ok(Expr::ATOM(atom))
                    }
                }
            },

            //if not a valid token, error
            _ => Err("Unexpected end of input!".to_string())
        }
    }

    pub fn parse_unary_expr(&mut self) -> Result<Expr, String> {

        let token = match self.peek() {
            Some(t) => t,
            None => return Err("Unexpected end of input!".to_string()),
        }; //grab a valid token

        match token.kind { 
            TokenType::PLUS | TokenType::MINUS => {
                let op = if token.kind == TokenType::PLUS {MonOp::POS} else {MonOp::NEG};
                self.advance(); //move past the unary

                //grab the next expr 
                let expr = self.parse_expr(op.get_precedence()+1)?;
                Ok(Expr::UNARY_EXPR{
                    opcode: op,
                    expr: Box::new(expr)
                })
            },

            _ => Err("Not a valid unary! You insane or what?".to_string())
        }

    }

    pub fn parse_expr(&mut self, current_precedence: u8) -> Result<Expr, String> {
        
        //first, handle the unary side of things
        let mut left = match self.peek() {
            Some(token) => match token.kind {
                TokenType::PLUS | TokenType::MINUS => self.parse_unary_expr()?,
                _ => self.parse_group_or_atom()?
            },
            None => return Err("Unexpected end of input while parsing expression!".to_string())
        };

        loop {
            let next_token = match self.peek(){
                Some(token) => token,
                None => return Err("Failed to unwrap next token...EOF?".to_string()),
            };

            let next_token_type =  &next_token.kind;
            
            let operator = match next_token_type {
                TokenType::PLUS => BinOp::ADD,
                TokenType::MINUS => BinOp::SUB,
                TokenType::SLASH => BinOp::DIV,
                TokenType::ASTERISK => BinOp::MULT,
                TokenType::EOF | TokenType::RPAREN => break, //break out if the next is EOF or the
                                                             //end of a grouping (indicated by RPAREN)
                _ => return Err(format!("Invalid token type detected: {:?} Fix your shit, dumbass.", next_token)),
            };

            let precedence = operator.get_precedence();
            if precedence < current_precedence {
                break; //stop building
            }

            self.advance(); //look at the token on the right
            let right = self.parse_expr(precedence+1)?; //recurse lol

            left = Expr::BINARY_EXPR {
                left: Box::new(left),
                opcode: operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }
}

