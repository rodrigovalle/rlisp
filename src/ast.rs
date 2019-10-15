use crate::lex::{
    expect, skip_space, take_while, Parse, ParseErrorKind, ParseResult,
};

#[derive(Debug, PartialEq)]
pub enum AstNode<'a> {
    SExpr(Vec<AstNode<'a>>),
    Symbol(&'a str),
    Number(i64),
}

impl<'a> AstNode<'a> {
    fn match_number(input: &str) -> ParseResult<AstNode> {
        let input = skip_space(input);
        let (digits, input) =
            take_while(input, "number", |c| c.is_ascii_digit() || *c == '-')?;

        if let Ok(int) = digits.parse() {
            Ok((AstNode::Number(int), input))
        } else {
            Err(ParseErrorKind::Integer)
        }
    }

    fn match_symbol(input: &str) -> ParseResult<AstNode> {
        let input = skip_space(input);
        let (symbol, input) = take_while(input, "symbol", |c| {
            c.is_ascii_alphanumeric()
                || (c.is_ascii_punctuation()
                    && !match c {
                        '(' | ')' | '\'' => true,
                        _ => false,
                    })
        })?;

        Ok((AstNode::Symbol(symbol), input))
    }
}

impl<'a> Parse<'a> for AstNode<'a> {
    fn parse(input: &'a str) -> ParseResult<AstNode<'a>> {
        let input = skip_space(input);
        let mut input = expect("(", input)?;
        let mut children = Vec::new();

        while let Some(c) = input.chars().next() {
            if c == ')' {
                break;
            }

            let result = AstNode::parse(input)
                .or_else(|_| AstNode::match_number(input))
                .or_else(|_| AstNode::match_symbol(input));

            if let Ok((sexpr, rest)) = result {
                children.push(sexpr);
                input = rest;
            } else {
                return Err(ParseErrorKind::SExpr);
            }
        }
        let input = expect(")", input)?;

        Ok((AstNode::SExpr(children), input))
    }
}

#[cfg(test)]
mod parser_test {
    use super::*;

    fn parse_expect(input: &'static str, expect: AstNode) {
        let (result, _) = AstNode::parse(input).expect("parse error");
        assert_eq!(result, expect);
    }

    #[test]
    fn test_empty() {
        parse_expect("()", AstNode::SExpr(vec![]))
    }

    #[test]
    fn test_number() {
        parse_expect("(1)", AstNode::SExpr(vec![AstNode::Number(1)]))
    }

    #[test]
    fn test_symbol() {
        parse_expect("(test)", AstNode::SExpr(vec![AstNode::Symbol("test")]))
    }

    #[test]
    fn test_number_symbol() {
        parse_expect(
            "(test 1)",
            AstNode::SExpr(vec![AstNode::Symbol("test"), AstNode::Number(1)]),
        )
    }

    #[test]
    fn test_nested() {
        parse_expect(
            "(test (test 1 2) 3)",
            AstNode::SExpr(vec![
                AstNode::Symbol("test"),
                AstNode::SExpr(vec![
                    AstNode::Symbol("test"),
                    AstNode::Number(1),
                    AstNode::Number(2),
                ]),
                AstNode::Number(3),
            ]),
        )
    }
}
