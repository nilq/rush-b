extern crate colored;

mod rush;
use rush::{lexer::*, source::Source};

fn main() {
    let test = r#"
public static fib =
    | 0 => 0
    | 1 => 1
    | n => fib (n - 2) + fib (n - 1)

fib 100s
    "#;
    
    let source = Source::from(
        "<main>",
        test.lines().map(|x| x.into()).collect::<Vec<String>>(),
    );
    let lexer = Lexer::default(test.chars().collect(), &source);

    let mut tokens = Vec::new();

    for token_result in lexer {
        if let Ok(token) = token_result {
            tokens.push(token)
        } else {
            return;
        }
    }

    println!("{:#?}", tokens)
}
