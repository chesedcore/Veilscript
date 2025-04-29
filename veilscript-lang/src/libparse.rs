#![allow(unused_doc_comments)]

use crate::parser::Parser;
use crate::lexer::{TokenType};
use crate::ast::*;

impl<'a> Parser<'a> {

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

            _ => Err(format!(
            "What the hell? Expected either LITERAL or IDENT, found {:?} instead... huh??",
            token.kind
            )),
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
    
    //grabbing an ident with the helpers in parser.rs is tricky because they return the tokentype,
    //not the token itself. i made this to make my life a little easier. this advances forward, so
    //be EXTREMELY sure the next tokentype is for sure syntactically an IDENTIFIER.
    pub fn parse_next_ident(&mut self) -> Result<Ident, String> {
        let token = self.advance_and_extract()?;
        Parser::check_for(token.clone(), TokenType::IDENTIFIER)?;
        let name = token.lexeme.to_owned();
        Ok(Ident{name})
    }
    
    ///MATCHES: Vec<Parameter> RPAREN
    pub fn parse_params(&mut self) -> Result<Vec<Parameter>, String> {
        let mut params: Vec<Parameter> = Vec::new();
        
        //in case there aren't any parameters
        match self.check_next_contains(&[TokenType::RPAREN, TokenType::IDENTIFIER])? {
            TokenType::RPAREN => {
                self.advance();
                return Ok(params);
            }
            _ => {}
        }
        
        //keep getting params
        loop {
            let ident = self.parse_next_ident()?;
            self.check_advance(TokenType::COLON)?;
            let type_t = self.advance_and_extract()?.kind.clone();
            params.push(Parameter{
                ident, type_t
            });
            let next = self.check_advance_contains(&[TokenType::COMMA, TokenType::RPAREN])?;
            if next == TokenType::COMMA {
                continue;
            } else {
                return Ok(params);
            }
        }
    }


    ///MATCHES: FN IDENTIFIER LPAREN Vec<Parameter> RPAREN ARROW TYPE_T       // {scope}
    pub fn parse_function_declaration(&mut self) -> Result<FnDeclaration, String> {
        self.check_advance(TokenType::FN)?;
        let ident = self.parse_next_ident()?;
        self.check_advance(TokenType::LPAREN)?;
        let params = self.parse_params()?;

        match self.check_next_contains(&[TokenType::LBRACE, TokenType::ARROW])? {
            TokenType::LBRACE => Ok(FnDeclaration{ident,params,type_t: TokenType::TYPE_VOID}),
            TokenType::ARROW => {
                self.advance();
                let type_t = self.advance_and_extract()?.kind;
                return Ok(FnDeclaration{ident,params,type_t});
            },
            _ => Err("".to_string())
        }
    }


    
    ///FULL PARSER METHODS
    ///these methods will probably be HUGE. 
    ///these allow the parsing of statements

    pub fn parse_statement(&mut self) -> Result<Stmt, String> {
        let token = self.peek_and_extract()?;
        
        let statement = match token.kind { //lord save me for this 9000 line match 
            TokenType::IDENTIFIER => self.parse_assignment()?,
            TokenType::FN => Stmt::STATEMENT_FUNCTION_DECLARATION(self.parse_function_declaration()?),
            _ => Stmt::STATEMENT_ZERO_EFFECT,
        };
        Ok(statement)
    } 

    pub fn parse_rhs_expr(&mut self) -> Result<Expr, String> {
        self.check_advance(TokenType::EQUALS)?;
        let rhs = self.parse_full_expr()?;
        self.check_advance(TokenType::SEMICOLON)?;
        Ok(rhs)
    }

    pub fn parse_assignment(&mut self) -> Result<Stmt, String> {
        let ident_token = self.advance_and_extract()?;
        let name = ident_token.lexeme.to_owned();

        let intermediate_token = self.advance_and_extract()?;

        match intermediate_token.kind {
            
            TokenType::SEMICOLON => {
                Ok(Stmt::STATEMENT_ZERO_EFFECT)
            },

            TokenType::COLON => {

                ///MATCHED PATTERN -> (IDENT COLON) ANYTOKEN EQUALS Expr SEMICOLON;
                ///                    balls    :     int       =   3+2    ;

                let type_token = self.advance_and_extract()?; //grab type
                let type_t = Some(type_token.kind.clone()); //grab tokentype
                let expr = Box::new(self.parse_rhs_expr()?); //grab expr
                Ok(Stmt::STATEMENT_ASSIGNMENT(
                        Assignment{ ident: Ident{name}, type_t, expr }
                ))
            },

            TokenType::EQUALS => {

                ///MATCHED PATTERN -> (IDENT EQUALS) Expr SEMICOLON;
                ///                     balls   =     2+2    ;

                let expr = Box::new(self.parse_full_expr()?);
                self.check_advance(TokenType::SEMICOLON)?;
                Ok(Stmt::STATEMENT_ASSIGNMENT(
                        Assignment{ ident: Ident{name}, type_t: None, expr }
                ))
            }

            _ => return Err("balls".to_string()),
        }
    }

    
}
