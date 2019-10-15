mod ast;
mod env;
mod eval;
mod lex;
use ast::AstNode;
use eval::Eval;
use lex::Parse;
use std::collections::HashMap;

fn main() {
    run("(+ 1 (+ 2 3))");
    run("(+ (+ 1 2) 3 4 (- 2 1))");
    //run("(+ (+ 1 2) ())");
}

fn run(program: &str) {
    let evaluator = Eval::new(HashMap::new());
    match AstNode::parse(program) {
        Ok((sexpr, _)) => {
            println!("parsed: {:?}", sexpr);
            println!("eval: {:?}", evaluator.eval(&sexpr));
        }
        Err(err) => {
            println!("failure");
            println!("{:?}", err);
        }
    }
}
