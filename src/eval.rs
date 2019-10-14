use crate::sexpr::SExprType;
use crate::lex::Parse;

enum EvalErrorKind {
    ExpectedSExpr,
    ExpectedNumber,
    TooFewArguments,
}

enum FunctionType<'a> {
    SExpr(SExprType<'a>),
}

type EvalResult<T> = Result<T, EvalErrorKind>;

pub fn eval<'a>(ast: &'a SExprType) -> SExprType<'a> {
    match ast {
        SExprType::SExpr(l) => {
            if !l.is_empty() {
                // functional application
                if let SExprType::Symbol(fn_name) = l[0] {
                    let args = evlis(&l[1..]);
                    eval_builtin(fn_name, &args)
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

fn evlis<'a>(list: &'a [SExprType]) -> Vec<SExprType<'a>> {
    list.iter().map(eval).collect()
}

fn eval_builtin<'a>(fn_name: &'a str, args: &[SExprType<'a>]) -> SExprType<'a> {
    match (fn_name, args) {
        ("+", args) => builtin_addition(args),
        ("-", args) => builtin_subtraction(args),
        (op, args) => panic!("unrecognized operator {}", op),
    }
}

fn unwrap_number(item: &SExprType) -> i64 {
    if let SExprType::Number(i) = item {
        *i
    } else {
        panic!("{:?} is not a number", item)
    }
}

fn builtin_addition<'a>(args: &[SExprType<'a>]) -> SExprType<'a> {
    let sum = args
        .iter()
        .map(unwrap_number)
        .sum();
    SExprType::Number(sum)
}

fn builtin_subtraction<'a>(args: &[SExprType<'a>]) -> SExprType<'a> {
    match args.split_at(1) {
        ([SExprType::Number(head)], []) => SExprType::Number(-head),
        ([SExprType::Number(head)], tail) => {
            let result = tail
                .iter()
                .map(unwrap_number)
                .fold(*head, |acc, i| acc - i);
            SExprType::Number(result)
        },
        _ => panic!("'-' Received unexpected arguments")
    }
}

#[cfg(test)]
mod eval_test {
    use super::*;

    fn eval_expect(input: &'static str, expect: SExprType) {
        let (result, _) = SExprType::parse(input).expect("parse error");
        assert_eq!(eval(&result), expect, "input: {}", input);
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