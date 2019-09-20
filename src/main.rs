mod eval;
mod lex;
mod sexpr;
use lex::Parse;
use sexpr::SExprType;
use eval::eval;

fn main() {
    match SExprType::parse("(+ (+ 1 2) 3 4)") {
        Ok((sexpr, _)) => {
            println!("success");
            println!("{:?}", sexpr);
            println!("{:?}", eval(&sexpr));
        }
        Err(err) => {
            println!("failure");
            println!("{:?}", err);
        }
    }
}
