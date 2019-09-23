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

// -- GADTs with explicit type constructor kinds could make this kind (haha) of
// -- thing much easier!
//
// type _ value =
//   | Number: int -> int value
//   | Symbol: str -> str value
//
// type _ expr =
//   | Value: 'a value -> 'a expr
//   | If: bool expr * 'a expr * 'a expr -> 'a expr
//   | Eq: 'a expr * 'a expr -> bool expr
//   | Lt: int expr * int expr -> bool expr
//
// let rec eval : type a. a expr -> a = function
//   | Value (Bool b) -> b
//   | Value (Int i) -> i
//   | If (b, l, r) -> if eval b then eval l else eval r
//   | Eq (a, b) -> (eval a) = (eval b)
//   | Lt (a,b) -> (eval a) < (eval b)
//
// (if (n < 0) ((print "a")) ((print "b")))

pub fn eval<'a>(ast: &'a SExprType) -> SExprType<'a> {
    match ast {
        SExprType::SExpr(l) => {
            if let Some(SExprType::Symbol(fn_name)) = l.get(0) {
                eval_function(fn_name, &l[1..])
            } else {
                panic!("implementation does not recognize nil yet")
            }
        }
        SExprType::Number(i) => SExprType::Number(*i),
        SExprType::Symbol(s) => SExprType::Symbol(s),
    }
}

fn eval_function<'a>(
    fn_name: &'a str,
    args: &[SExprType<'a>],
) -> SExprType<'a> {
    match (fn_name, args) {
        ("+", args) => eval_addition(args),
        ("-", args) => eval_subtraction(args),
        (op, args) => panic!("unrecognized operator {}", op),
    }
}

fn unwrap_number(op: &'static str) -> impl Fn(&SExprType) -> i64 {
    move |item| {
        match item {
            SExprType::Number(i) => *i,
            expr => {
                if let SExprType::Number(i) = eval(expr) {
                    i
                } else {
                    println!("{:?}", item);
                    panic!("'{}' recieved unexpected arguments", op)
                }
            }
        }
    }
}

fn eval_addition<'a>(args: &[SExprType<'a>]) -> SExprType<'a> {
    let sum = args
        .iter()
        .map(unwrap_number("+"))
        .fold(0, |acc, i| acc + i);
    SExprType::Number(sum)
}

fn eval_subtraction<'a>(args: &[SExprType<'a>]) -> SExprType<'a> {
    match args.split_at(1) {
        ([SExprType::Number(head)], []) => SExprType::Number(-head),
        ([SExprType::Number(head)], tail) => {
            let result = tail
                .iter()
                .map(unwrap_number("-"))
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