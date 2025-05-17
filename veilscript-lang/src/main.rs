mod lexer;
mod parser;
mod ast;
mod libparse;

use lexer::*;
use parser::Parser;

fn main() {
    let source = r#"
    {
        balls = 2;
        cock: int = balls;
    }
    "#;

    //trial run
    let tokens = tokenise(&source);
    println!("{}", &source);
    //print_tokens_from_string(&source);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_statement().expect("Something exploded");
    let node = ast.to_pretty_string();
    println!("--parse-results--");
    println!("{}",node);
}
