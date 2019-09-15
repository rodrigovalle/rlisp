use crate::lex::{
    expect, skip_space, take_while, Parse, ParseErrorKind, ParseResult,
};

#[derive(Debug, PartialEq)]
pub enum SExpr<'a> {
    SExpr(Vec<SExpr<'a>>),
    Symbol(&'a str),
    Number(i64),
}

impl<'a> SExpr<'a> {
    fn match_number(input: &str) -> ParseResult<SExpr> {
        let input = skip_space(input);
        let (digits, input) =
            take_while(input, "number", |c| c.is_ascii_digit())?;

        if let Ok(int) = digits.parse() {
            Ok((SExpr::Number(int), input))
        } else {
            Err(ParseErrorKind::Integer)
        }
    }

    fn match_symbol(input: &str) -> ParseResult<SExpr> {
        let input = skip_space(input);
        let (symbol, input) = take_while(input, "symbol", |c| {
            c.is_ascii_alphanumeric()
                || (c.is_ascii_punctuation()
                    && !match c {
                        '(' | ')' | '\'' => true,
                        _ => false,
                    })
        })?;

        Ok((SExpr::Symbol(symbol), input))
    }
}

impl<'a> Parse<'a> for SExpr<'a> {
    fn parse(input: &'a str) -> ParseResult<SExpr<'a>> {
        let input = skip_space(input);
        let mut input = expect("(", input)?;
        let mut children = Vec::new();

        while let Some(c) = input.chars().next() {
            if c == ')' {
                break;
            }

            let result = SExpr::parse(input)
                .or_else(|_| SExpr::match_number(input))
                .or_else(|_| SExpr::match_symbol(input));

            if let Ok((sexpr, rest)) = result {
                children.push(sexpr);
                input = rest;
            } else {
                return Err(ParseErrorKind::SExpr);
            }
        }
        let input = expect(")", input)?;

        Ok((SExpr::SExpr(children), input))
    }
}

#[cfg(test)]
mod parser_test {
    use super::*;

    fn parse_expect(input: &'static str, expect: SExpr) {
        let (result, _) = SExpr::parse(input).expect("parse error");
        assert_eq!(result, expect);
    }

    #[test]
    fn test_empty() {
        parse_expect("()", SExpr::SExpr(vec![]))
    }

    #[test]
    fn test_number() {
        parse_expect("(1)", SExpr::SExpr(vec![SExpr::Number(1)]))
    }

    #[test]
    fn test_symbol() {
        parse_expect("(test)", SExpr::SExpr(vec![SExpr::Symbol("test")]))
    }

    #[test]
    fn test_number_symbol() {
        parse_expect(
            "(test 1)",
            SExpr::SExpr(vec![SExpr::Symbol("test"), SExpr::Number(1)]),
        )
    }

    #[test]
    fn test_nested() {
        parse_expect(
            "(test (test 1 2) 3)",
            SExpr::SExpr(vec![
                SExpr::Symbol("test"),
                SExpr::SExpr(vec![
                    SExpr::Symbol("test"),
                    SExpr::Number(1),
                    SExpr::Number(2),
                ]),
                SExpr::Number(3),
            ]),
        )
    }
}
