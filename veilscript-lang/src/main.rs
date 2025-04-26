mod lexer;
mod parser;
mod ast;

use lexer::*;
use parser::Parser;

fn main() {
    let source = r#"
    3 + 2*5 + 8*4 /3@
    //              ^ remove this @ to make code function properly
    "#;

    //trial run
    let tokens = tokenise(&source);
    print_tokens_from_string(&source);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expr(0).expect("Something exploded!");
    let node = ast.to_pretty_string();

    println!("{}",node);
}
