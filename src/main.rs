mod lex;
use lex::Lexer;

fn main() {
    let input = "(define hello (3 4))";
    let mut tokenizer = Lexer::new(input);
    loop {
        match tokenizer.next_token() {
            Some(token) => println!("{:?}", token),
            _ => break,
        }
    }
}
