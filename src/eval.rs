#![feature(advanced_slice_patterns)]
use crate::sexpr::SExprType;

enum EvalErrorKind {
    ExpectedSExpr,
    ExpectedNumber,
    TooFewArguments,
}

enum ValueType {
    SExpr,
    Number,
}

enum ExprType {
    Value(ValueType),
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

// type value =
//   | Symbol of string
//   | Number of int
//   | Bool of bool
//
// type SExpr =
//   | SExpr of SExpr list
//   | Value of value

// type sexpr =
//   | Sexpr of sexpr list
//   | Number of int
//   | Symbol of string

pub fn eval<'a>(ast: &'a SExprType) -> SExprType<'a> {
    match ast {
        SExprType::SExpr(l) => eval_function(l),
        SExprType::Number(i) => SExprType::Number(*i),
        SExprType::Symbol(s) => SExprType::Symbol(s),
    }
}


fn eval_function<'a>(vec: &Vec<SExprType<'a>>) -> SExprType<'a> {
    let maybe_op = vec.get(0);
    match maybe_op {
        Some(SExprType::Symbol("+")) => {
            let sum = &vec[1..].iter().fold(0, |acc, item| {
                if let SExprType::Number(i) = eval(item) {
                    acc + i
                } else {
                    panic!("oops");
                }
            });
            SExprType::Number(*sum)
        },
        _ => panic!("uhh")
    }
}