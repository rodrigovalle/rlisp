mod lex;
use lex::Lexer;

fn main() {
    let tokenizer = Lexer::new("(define hello (3 4))");
    for token in tokenizer {
        print!("{:?} ", token);
    }
}
