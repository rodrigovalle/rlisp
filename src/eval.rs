use crate::sexpr::SExprType;
use crate::env::Env;
use std::collections::HashMap;

pub struct Eval<'a> {
    env: Env<&'a str, SExprType<'a>>
}

impl<'a> Eval<'a> {
    pub fn new(init_env: HashMap<&'a str, SExprType<'a>>) -> Eval {
        Eval { env: Env::new(init_env) }
    }

    pub fn eval(&self, ast: &'a SExprType) -> SExprType<'a> {
        match ast {
            SExprType::SExpr(l) => {
                if !l.is_empty() {
                    // functional application
                    if let SExprType::Symbol(fn_name) = l[0] {
                        let args = self.evlis(&l[1..]);
                        self.eval_builtin(fn_name, &args)
                    } else {
                        panic!("no such function {:?}", l[0])
                    }
                } else {
                    // leave nil alone
                    SExprType::SExpr(vec![])
                }
            },
            SExprType::Number(i) => SExprType::Number(*i),
            SExprType::Symbol(s) => SExprType::Symbol(s),
        }
    }

    fn evlis(&self, list: &'a [SExprType]) -> Vec<SExprType<'a>> {
        list.iter().map(|i| self.eval(i)).collect()
    }

    fn eval_builtin(&self, fn_name: &'a str, args: &[SExprType<'a>]) -> SExprType<'a> {
        match (fn_name, args) {
            ("+", args) => Self::builtin_addition(args),
            ("-", args) => Self::builtin_subtraction(args),
            (op, args) => panic!("unrecognized operator {}", op),
        }
    }

    fn builtin_addition(args: &[SExprType<'a>]) -> SExprType<'a> {
        let sum = args
            .iter()
            .map(Self::unwrap_number)
            .sum();
        SExprType::Number(sum)
    }

    fn builtin_subtraction(args: &[SExprType<'a>]) -> SExprType<'a> {
        match args.split_at(1) {
            ([SExprType::Number(head)], []) => SExprType::Number(-head),
            ([SExprType::Number(head)], tail) => {
                let result = tail
                    .iter()
                    .map(Self::unwrap_number)
                    .fold(*head, |acc, i| acc - i);
                SExprType::Number(result)
            },
            _ => panic!("'-' Received unexpected arguments")
        }
    }

    fn unwrap_number(item: &SExprType) -> i64 {
        if let SExprType::Number(i) = item {
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

    fn eval_expect(input: &'static str, expect: SExprType) {
        let (result, _) = SExprType::parse(input).expect("parse error");
        let eval = Eval::new(HashMap::new());
        assert_eq!(eval.eval(&result), expect, "input: {}", input);
    }

    #[test]
    fn test_add() {
        let tests = vec![
            ("(+ 1 1)", SExprType::Number(2)),
            ("(+ 2 1)", SExprType::Number(3)),
            ("(+ 0)", SExprType::Number(0)),
            ("(+ -1 0)", SExprType::Number(-1)),
            ("(+ 1 -0)", SExprType::Number(1)),
            ("(+ 1 2 3 4)", SExprType::Number(10)),
        ];
        for (expr, result) in tests {
            eval_expect(expr, result);
        }
    }

    fn test_sub() {
        let tests = vec![
            ("(- 1 1)", SExprType::Number(0)),
            ("(- 2 1)", SExprType::Number(1)),
            ("(- 1 2)", SExprType::Number(-1)),
            ("(- 2)", SExprType::Number(-2)),
            ("(- 1 2 3 4)", SExprType::Number(-8)),
        ];
        for (expr, result) in tests {
            eval_expect(expr, result);
        }
    }
}