#[derive(Debug)]
pub enum ParseErrorKind {
    Expected(&'static str),
    SExpr,
    Integer,
    Symbol,
}

pub type ParseResult<'a, T> = Result<(T, &'a str), ParseErrorKind>;

pub trait Parse<'a>: Sized {
    fn parse(input: &'a str) -> ParseResult<Self>;
}

pub trait ParseWith<'a, C>: Sized {
    fn parse(input: &'a str, context: C) -> ParseResult<Self>;
}

/// Skip spaces at the beginning of a string.
pub fn skip_space(input: &str) -> &str {
    let iter = input.chars();
    let taken = iter.take_while(|c| c.is_ascii_whitespace()).count();
    &input[taken..]
}

/// Assert that the beginning of the string matches what is expected. Return the
/// input with the match removed if the check is successful.
pub fn expect<'a>(
    expected: &'static str,
    input: &'a str,
) -> Result<&'a str, ParseErrorKind> {
    if input.starts_with(expected) {
        Ok(&input[expected.len()..])
    } else {
        Err(ParseErrorKind::Expected(expected))
    }
}

pub fn take_while<'a, F: Fn(&char) -> bool>(
    input: &'a str,
    what: &'static str,
    f: F,
) -> ParseResult<'a, &'a str> {
    let end = input.chars().take_while(f).count();
    if end != 0 {
        let take = &input[..end];
        let rest = &input[end..];
        Ok((take, rest))
    } else {
        Err(ParseErrorKind::Expected(what))
    }
}

/*
 * A lisp tokenizer implementation from before I decided to use parser
 * combinators instead
 */

/*
 * use std::iter::{Iterator, Peekable};
 * use std::str::CharIndices;
 *
 * #[derive(Debug, Eq, PartialEq)]
 * pub enum Token<'a> {
 *     Paren(char),
 *     Operator(char),
 *     Number(i64),
 *     Symbol(&'a str),
 * }
 *
 * pub struct Lexer<'a> {
 *     input: &'a str,
 *     input_iter: Peekable<CharIndices<'a>>,
 * }
 *
 * impl<'a> Lexer<'a> {
 *     pub fn new(input: &str) -> Lexer {
 *         Lexer {
 *             input,
 *             input_iter: input.char_indices().peekable(),
 *         }
 *     }
 *
 *     pub fn next_token(&mut self) -> Option<Token<'a>> {
 *         self.skip_whitespace();
 *         if let Some((i, c)) = self.input_iter.next() {
 *             match c {
 *                 '(' | ')' => Some(Token::Paren(c)),
 *                 '+' | '-' | '*' | '/' => Some(Token::Operator(c)),
 *                 '0'...'9' => Some(self.tokenize_number(i)),
 *                 'a'...'z' | 'A'...'Z' | '_' => Some(self.tokenize_symbol(i)),
 *                 _ => None,
 *             }
 *         } else {
 *             None
 *         }
 *     }
 *
 *     fn peek_char(&mut self) -> Option<&char> {
 *         match self.input_iter.peek() {
 *             Some((_, c)) => Some(c),
 *             None => None,
 *         }
 *     }
 *
 *     fn next_char(&mut self) -> Option<char> {
 *         match self.input_iter.next() {
 *             Some((_, c)) => Some(c),
 *             None => None,
 *         }
 *     }
 *
 *     fn tokenize_symbol(&mut self, start: usize) -> Token<'a> {
 *         while let Some(&(i, c)) = self.input_iter.peek() {
 *             if (c.is_ascii_alphanumeric() || c.is_ascii_punctuation())
 *                 && !(c == '(' || c == ')')
 *             {
 *                 self.input_iter.next();
 *                 continue;
 *             } else {
 *                 return Token::Symbol(&self.input[start..i]);
 *             }
 *         }
 *         Token::Symbol(&self.input[start..])
 *     }
 *
 *     fn tokenize_number(&mut self, start: usize) -> Token<'a> {
 *         while let Some(&(i, c)) = self.input_iter.peek() {
 *             if c.is_ascii_digit() {
 *                 self.input_iter.next();
 *                 continue;
 *             } else {
 *                 let val: i64 = self.input[start..i].parse().unwrap();
 *                 return Token::Number(val);
 *             }
 *         }
 *
 *         let val: i64 = self.input[start..].parse().unwrap();
 *         Token::Number(val)
 *     }
 *
 *     fn skip_whitespace(&mut self) {
 *         while let Some(c) = self.peek_char() {
 *             if c.is_ascii_whitespace() {
 *                 self.next_char();
 *             } else {
 *                 return;
 *             }
 *         }
 *     }
 * }
 *
 * impl<'a> Iterator for Lexer<'a> {
 *     type Item = Token<'a>;
 *     fn next(&mut self) -> Option<Self::Item> {
 *         self.next_token()
 *     }
 * }
 */

#[cfg(test)]
mod lexer_test {
    /* use crate::lex::{Lexer, Token};
    *
    * #[test]
    * fn test_e2e() {
    *     let input = "(some-function? (list a b c d) 1 (+ 2 3))";
    *     let tokenizer = Lexer::new(input);
    *     let expected = vec![
    *         Token::Paren('('),
    *         Token::Symbol("some-function?"),
    *         Token::Paren('('),
    *         Token::Symbol("list"),
    *         Token::Symbol("a"),
    *         Token::Symbol("b"),
    *         Token::Symbol("c"),
    *         Token::Symbol("d"),
    *         Token::Paren(')'),
    *         Token::Number(1),
    *         Token::Paren('('),
    *         Token::Operator('+'),
    *         Token::Number(2),
    *         Token::Number(3),
    *         Token::Paren(')'),
    *         Token::Paren(')'),
    *     ];

    *     for (actual, expected) in Iterator::zip(tokenizer, expected) {
    *         assert_eq!(actual, expected);
    *     }
    * }
    */
}
