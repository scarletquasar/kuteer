use lexer::Lexer;
use crate::lexer::Token;

pub mod lexer;

fn main() {
    let input = "let x = 42 + 10;";
    let mut lexer = Lexer::new(input);

    loop {
        let token = lexer.get_next_token();
        println!("{:?}", token);

        if token == Token::EOF {
            break;
        }

        if token == Token::Unknown {
            panic!("Unknown token detected")
        }
    }
}