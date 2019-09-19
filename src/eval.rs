use crate::sexpr::SExpr;

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


trait Eval {
    fn eval(&self) -> EvalResult<>;
}

impl<'a> Eval for SExpr<'a> {
    fn eval(expr: SExpr) -> EvalResult<>;
}