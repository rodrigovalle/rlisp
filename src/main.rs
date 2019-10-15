mod ast;
mod env;
mod eval;
mod lex;
use eval::Eval;
use ast::AstNode;
use lex::Parse;

fn main() {
    run("(+ 1 (+ 2 3))");
    run("(+ (+ 1 2) 3 4 (- 2 1))");
    run("(def! a 10)");
    //run("(+ (+ 1 2) ())");
}

fn run(program: &str) {
    let mut eval = Eval::new();
    match AstNode::parse(program) {
        Ok((sexpr, _)) => {
            println!("parsed: {:?}", sexpr);
            println!("eval: {:?}", eval.eval(&sexpr));
        }
        Err(err) => {
            println!("failure");
            println!("{:?}", err);
        }
    }
}
