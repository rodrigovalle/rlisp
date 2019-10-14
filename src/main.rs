mod eval;
mod lex;
mod sexpr;
use lex::Parse;
use sexpr::SExprType;
use eval::eval;

fn main() {
    run("(+ 1 (+ 2 3))");
    run("(+ (+ 1 2) 3 4 (- 2 1))");
    //run("(+ (+ 1 2) ())");
}

fn run(program: &str) {
    match SExprType::parse(program) {
        Ok((sexpr, _)) => {
            println!("parsed: {:?}", sexpr);
            println!("eval: {:?}", eval(&sexpr));
        }
        Err(err) => {
            println!("failure");
            println!("{:?}", err);
        }
    }
}
