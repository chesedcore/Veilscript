use logos::Logos;

#[allow(non_camel_case_types)]
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum TokenType {
    //keywords
    #[token("fn")]
    #[token("rite")]
    FN,

    #[token("return")]
    #[token("ret")]
    RETURN,

    //types
    #[token("float")]
    #[token("num")]
    #[token("numeric")]
    TYPE_FLOAT,

    #[token("int")]
    EXPERIMENTAL_TYPE_INT,

    #[token("rune")]
    #[token("string")]
    TYPE_STRING,

    #[token("void")]
    #[token("nothing")]
    #[token("null")]
    TYPE_VOID,
    
    //punctuation
    #[token("=")]
    EQUALS,

    #[token(":")]
    COLON,

    #[token("::")]
    DOUBLE_COLON,

    #[token(".")]
    DOT,

    #[token("->")]
    ARROW,

    #[token(",")]
    COMMA,

    #[token(";")]
    SEMICOLON,

    #[token("(")]
    LPAREN,

    #[token(")")]
    RPAREN,

    #[token("{")]
    LBRACE,

    #[token("}")]
    RBRACE,

    #[token("+")]
    PLUS,

    #[token("-")]
    MINUS,

    #[token("/")]
    SLASH,

    #[token("*")]
    ASTERISK,


    //literals
    #[regex(r#""([^"\\]|\\.)*""#)]
    LITERAL_STRING,

    #[regex(r"[0-9]+\.[0-9]+")]
    LITERAL_FLOAT,

    #[regex(r"[0-9]+")]
    LITERAL_INT,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    IDENTIFIER,

    #[regex(r"//[^\n]*", logos::skip)]
    COMMENT,

    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    WHITESPACE,

    //fallback
    #[regex(r".", priority=0)]
    ERROR,
    
    //last reached
    EOF
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token<'src> {
    pub lexeme: &'src str,
    pub kind: TokenType,
}

pub fn tokenise(source_string: &str) -> Vec<Token> {
    let mut result_vector= Vec::<Token>::new();
    let mut lexer = TokenType::lexer(source_string);

    while let Some(Ok(kind)) = lexer.next() {
        let lexeme = lexer.slice();
        result_vector.push( Token{lexeme, kind} );
    }
    result_vector.push( Token{lexeme:"STOP", kind:TokenType::EOF});
    result_vector
}

pub fn print_tokens_from_string(source_string: &str){
    let tok_vec = tokenise(source_string);
    for token in tok_vec.iter() {
        println!("{:?} -> {}", token.kind, token.lexeme);
    }
}
