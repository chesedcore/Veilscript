mod lexer;

fn main() {
    let source = r#"
    func greet() -> void {
        message = "my balls itch";
    }
    "#;

    //trial run
    lexer::print_tokens_from_string(source);
}
