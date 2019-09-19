mod lex;
mod sexpr;
use lex::Parse;
use sexpr::SExpr;

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
