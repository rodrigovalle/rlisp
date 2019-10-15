#[derive(Debug)]
pub enum ParseErrorKind {
    Expected(&'static str),
    SExpr,
    Integer,
}

pub type ParseResult<'a, T> = Result<(T, &'a str), ParseErrorKind>;

pub trait Parse<'a>: Sized {
    fn parse(input: &'a str) -> ParseResult<Self>;
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
