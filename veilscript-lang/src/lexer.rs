use logos::Logos;

#[allow(non_camel_case_types)]
#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    //keywords
    #[token("func")]
    #[token("rite")]
    FUNC,

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
    #[token("nothingness")]
    #[token("null")]
    TYPE_VOID,
    
    //punctuation
    #[token("=")]
    EQUALS,

    #[token(":")]
    COLON,

    #[token("->")]
    ARROW,

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
}

pub fn tokenise(source_string: &str) -> Vec<Token> {
    let mut result_vector= Vec::<Token>::new();
    for result in Token::lexer(source_string){
        if let Ok(token) = result {
            result_vector.push(token)
        }
    }
    result_vector
}

pub fn print_tokens_from_string(source_string: &str){
    let tok_vec = tokenise(source_string);
    for token in tok_vec.iter() {
        println!("{:?}", token);
    }
}
