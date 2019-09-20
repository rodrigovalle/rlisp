use crate::lex::{
    expect, skip_space, take_while, Parse, ParseErrorKind, ParseResult,
};

#[derive(Debug, PartialEq)]
pub enum SExprType<'a> {
    SExpr(Vec<SExprType<'a>>),
    Symbol(&'a str),
    Number(i64),
}

impl<'a> SExprType<'a> {
    fn match_number(input: &str) -> ParseResult<SExprType> {
        let input = skip_space(input);
        let (digits, input) =
            take_while(input, "number", |c| c.is_ascii_digit())?;

        if let Ok(int) = digits.parse() {
            Ok((SExprType::Number(int), input))
        } else {
            Err(ParseErrorKind::Integer)
        }
    }

    fn match_symbol(input: &str) -> ParseResult<SExprType> {
        let input = skip_space(input);
        let (symbol, input) = take_while(input, "symbol", |c| {
            c.is_ascii_alphanumeric()
                || (c.is_ascii_punctuation()
                    && !match c {
                        '(' | ')' | '\'' => true,
                        _ => false,
                    })
        })?;

        Ok((SExprType::Symbol(symbol), input))
    }
}

impl<'a> Parse<'a> for SExprType<'a> {
    fn parse(input: &'a str) -> ParseResult<SExprType<'a>> {
        let input = skip_space(input);
        let mut input = expect("(", input)?;
        let mut children = Vec::new();

        while let Some(c) = input.chars().next() {
            if c == ')' {
                break;
            }

            let result = SExprType::parse(input)
                .or_else(|_| SExprType::match_number(input))
                .or_else(|_| SExprType::match_symbol(input));

            if let Ok((sexpr, rest)) = result {
                children.push(sexpr);
                input = rest;
            } else {
                return Err(ParseErrorKind::SExpr);
            }
        }
        let input = expect(")", input)?;

        Ok((SExprType::SExpr(children), input))
    }
}

#[cfg(test)]
mod parser_test {
    use super::*;

    fn parse_expect(input: &'static str, expect: SExprType) {
        let (result, _) = SExprType::parse(input).expect("parse error");
        assert_eq!(result, expect);
    }

    #[test]
    fn test_empty() {
        parse_expect("()", SExprType::SExpr(vec![]))
    }

    #[test]
    fn test_number() {
        parse_expect("(1)", SExprType::SExpr(vec![SExprType::Number(1)]))
    }

    #[test]
    fn test_symbol() {
        parse_expect(
            "(test)",
            SExprType::SExpr(vec![SExprType::Symbol("test")]),
        )
    }

    #[test]
    fn test_number_symbol() {
        parse_expect(
            "(test 1)",
            SExprType::SExpr(vec![
                SExprType::Symbol("test"),
                SExprType::Number(1),
            ]),
        )
    }

    #[test]
    fn test_nested() {
        parse_expect(
            "(test (test 1 2) 3)",
            SExprType::SExpr(vec![
                SExprType::Symbol("test"),
                SExprType::SExpr(vec![
                    SExprType::Symbol("test"),
                    SExprType::Number(1),
                    SExprType::Number(2),
                ]),
                SExprType::Number(3),
            ]),
        )
    }
}
