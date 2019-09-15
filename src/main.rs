mod grammar;
mod lex;
use grammar::SExpr;
use lex::Parse;

fn main() {
    //let tokenizer = Lexer::new("(define hello (3 4))");
    //for token in tokenizer {
    //    print!("{:?} ", token);
    //}
    match SExpr::parse("(+ (- 1 2) () 3 4)") {
        Ok((sexpr, _)) => {
            println!("success");
            println!("{:?}", sexpr);
        }
        Err(err) => {
            println!("failure");
            println!("{:?}", err);
        }
    }
}
