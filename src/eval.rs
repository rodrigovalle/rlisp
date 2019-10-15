use crate::ast::AstNode;
use crate::env::Env;
use std::collections::HashMap;

pub struct Eval<'a> {
    env: Env<&'a str, AstNode<'a>>,
}

impl<'a> Eval<'a> {
    pub fn new(init_env: HashMap<&'a str, AstNode<'a>>) -> Eval {
        Eval {
            env: Env::new(init_env),
        }
    }

    pub fn eval(&self, ast: &'a AstNode) -> AstNode<'a> {
        match ast {
            AstNode::SExpr(l) => {
                if !l.is_empty() {
                    // functional application
                    if let AstNode::Symbol(fn_name) = l[0] {
                        let args = self.evlis(&l[1..]);
                        self.eval_builtin(fn_name, &args)
                    } else {
                        panic!("no such function {:?}", l[0])
                    }
                } else {
                    // leave nil alone
                    AstNode::SExpr(vec![])
                }
            }
            AstNode::Number(i) => AstNode::Number(*i),
            AstNode::Symbol(s) => AstNode::Symbol(s),
        }
    }

    fn evlis(&self, list: &'a [AstNode]) -> Vec<AstNode<'a>> {
        list.iter().map(|i| self.eval(i)).collect()
    }

    fn eval_builtin(
        &self,
        fn_name: &'a str,
        args: &[AstNode<'a>],
    ) -> AstNode<'a> {
        match (fn_name, args) {
            ("+", args) => Self::builtin_addition(args),
            ("-", args) => Self::builtin_subtraction(args),
            (op, _args) => panic!("unrecognized operator {}", op),
        }
    }

    fn builtin_addition(args: &[AstNode<'a>]) -> AstNode<'a> {
        let sum = args.iter().map(Self::unwrap_number).sum();
        AstNode::Number(sum)
    }

    fn builtin_subtraction(args: &[AstNode<'a>]) -> AstNode<'a> {
        match args.split_at(1) {
            ([AstNode::Number(head)], []) => AstNode::Number(-head),
            ([AstNode::Number(head)], tail) => {
                let result = tail
                    .iter()
                    .map(Self::unwrap_number)
                    .fold(*head, |acc, i| acc - i);
                AstNode::Number(result)
            }
            _ => panic!("'-' Received unexpected arguments"),
        }
    }

    fn unwrap_number(item: &AstNode) -> i64 {
        if let AstNode::Number(i) = item {
            *i
        } else {
            panic!("{:?} is not a number", item)
        }
    }
}

#[cfg(test)]
mod eval_test {
    use super::*;
    use crate::lex::Parse;
    use std::collections::HashMap;

    fn eval_expect(input: &'static str, expect: AstNode) {
        let (result, _) = AstNode::parse(input).expect("parse error");
        let eval = Eval::new(HashMap::new());
        assert_eq!(eval.eval(&result), expect, "input: {}", input);
    }

    #[test]
    fn test_add() {
        let tests = vec![
            ("(+ 1 1)", AstNode::Number(2)),
            ("(+ 2 1)", AstNode::Number(3)),
            ("(+ 0)", AstNode::Number(0)),
            ("(+ -1 0)", AstNode::Number(-1)),
            ("(+ 1 -0)", AstNode::Number(1)),
            ("(+ 1 2 3 4)", AstNode::Number(10)),
        ];
        for (expr, result) in tests {
            eval_expect(expr, result);
        }
    }

    #[test]
    fn test_sub() {
        let tests = vec![
            ("(- 1 1)", AstNode::Number(0)),
            ("(- 2 1)", AstNode::Number(1)),
            ("(- 1 2)", AstNode::Number(-1)),
            ("(- 2)", AstNode::Number(-2)),
            ("(- 1 2 3 4)", AstNode::Number(-8)),
        ];
        for (expr, result) in tests {
            eval_expect(expr, result);
        }
    }
}
