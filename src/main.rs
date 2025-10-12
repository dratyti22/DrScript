mod lexer;
mod ast;

fn main() {
    let x = r#"
        var x = 6;
        ver y = 8;
        print(x*y);
    "#;
    let y = lexer::lex(x);
    println!("{:?}", y);
}
