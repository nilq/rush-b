extern crate colored;

mod rush;
use rush::{lexer::*, parser::*, source::Source};

fn main() {
    let test = r#"
@swag
let a =
    "this will print automatically"

let b =
    10 + 10

a = +a + -b
goto 'swag'
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

    println!("Stuck.");

    let mut parser = Parser::new(tokens, &source);

    match parser.parse() {
        Ok(ref ast) => {
            println!("{:#?}", ast)
        }
        _ => ()
    }
}
