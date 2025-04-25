mod lexer;

fn main() {
    let source = r#"
    func greet() -> string {
        message = "my balls itch";
        return message;
    }
    "#;

    //trial run
    lexer::print_tokens_from_string(source);
}
