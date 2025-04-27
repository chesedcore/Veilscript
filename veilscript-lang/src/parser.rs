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

    pub fn peek_and_extract(&self) -> Result<&Token, String> {
        match self.peek() {
            Some(t) => Ok(t),
            None => return Err("Unexpected end of input!".to_string()),
        }
    }

    pub fn advance(&mut self) -> Option<&Token> {
        let wrapped_token = self.tokens.get(self.pos);
        self.pos += 1;
        wrapped_token
    }

    ///general parsing functions

    pub fn parse_atom(&mut self) -> Result<Atom, String> {

        //peek -> Option<&Token>
        let token = self.peek_and_extract()?;
                
        //token: &Token { &lexeme: &str , &kind: &TokenType }
        //this is where i parse all the "primary" types 
        match token.kind {

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

    pub fn parse_group_or_atom(&mut self) -> Result<Expr, String> { //parses brackets properly
        //look into the next token...
        let token = self.peek_and_extract()?;
        //what is the token made of?
        match token.kind {
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
    }


    pub fn parse_unary_expr(&mut self) -> Result<Expr, String> {

        let token = self.peek_and_extract()?;
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
        let token = self.peek_and_extract()?;
        let mut left = match token.kind {
            TokenType::PLUS | TokenType::MINUS => self.parse_unary_expr()?,
            _ => self.parse_group_or_atom()?
        };

        loop {
            let next_token = self.peek_and_extract()?;
            let next_token_type =  &next_token.kind;
            
            let operator = match next_token_type {
                TokenType::PLUS => BinOp::ADD,
                TokenType::MINUS => BinOp::SUB,
                TokenType::SLASH => BinOp::DIV,
                TokenType::ASTERISK => BinOp::MULT,
                TokenType::EOF | TokenType::RPAREN 
                            | TokenType::SEMICOLON  => break, //break out if the next is EOF or the
                                                              //end of a grouping (indicated by RPAREN)
                                                              //or a semicolon (end of statement) 
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

    pub fn parse_full_expr(&mut self) -> Result<Expr, String> {
        Ok(self.parse_expr(0)?)
    }

    pub fn parse_assignment(&mut self) -> Result<Stmt, String> {
        let ident_token = self.peek_and_extract()?;
        let name = ident_token.lexeme.to_owned();
        self.advance(); //move past the IDENT tokens

        let intermediate_token = self.peek_and_extract()?;

        match intermediate_token.kind {
            TokenType::SEMICOLON => {
                self.advance(); //move past that semicolon
                Ok(Stmt::STATEMENT_ZERO_EFFECT)
            },
            TokenType::COLON => {
                self.advance(); //a type is being specified! capture the type
                let type_token = self.peek_and_extract()?;
                let type_t = Some(type_token.kind.clone());
                self.advance(); //capture the equals 
                let equals_token = self.peek_and_extract()?;
                match equals_token.kind {
                    TokenType::EQUALS => {},
                    _ => return Err(format!("Expected '=', found {:?}!", equals_token.kind)),
                }
                self.advance(); //the next thing is the expr.
                let expr = Box::new(self.parse_full_expr()?);
                self.advance(); //move past the semicolon.

                Ok(Stmt::STATEMENT_ASSIGNMENT(
                        Assignment{ ident: Ident{name}, type_t, expr }
                ))
            }
            _ => return Err("balls".to_string()),
        }
    }

    pub fn parse_statement(&mut self) -> Result<Stmt, String> {
        let token = self.peek_and_extract()?;
        //at this point. we have only four options
        //IDENTIFIER -> COLON -> TOKEN -> EQUALS -> EXPR -> SEMI... Stmt(Assignment)
        //IDENTIFIER -> EQUALS -> EXPR -> SEMI... same shit
        //IDENTIFIER -> SEMI... zero effect path
        //EXPR -> SEMI... zero effect expr
        let statement = match token.kind { //lord save me for this 9000 line match 
            TokenType::IDENTIFIER => self.parse_assignment()?,
            _ => Stmt::STATEMENT_ZERO_EFFECT,
        };
        Ok(statement)
    }
}

