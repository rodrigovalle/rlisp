mod grammar;
mod lex;
use grammar::SExpr;
use lex::Parse;

fn main() {
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
