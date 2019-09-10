use std::iter::Iterator;
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug)]
pub enum Token<'a> {
    Paren(char),
    Operator(char),
    Number(i64),
    Symbol(&'a str),
}

pub struct Lexer<'a> {
    input: &'a str,
    input_iter: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            input,
            input_iter: input.char_indices().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if let Some((i, c)) = self.input_iter.next() {
            match c {
                '(' | ')' => Some(Token::Paren(c)),
                '+' | '-' | '*' | '/' => Some(Token::Operator(c)),
                '0'...'9' => Some(self.tokenize_number(i)),
                'a'...'z' | 'A'...'Z' | '_' => Some(self.tokenize_symbol(i)),
                _ => None,
            }
        } else {
            None
        }
    }

    fn peek_char(&mut self) -> Option<&char> {
        match self.input_iter.peek() {
            Some((_, c)) => Some(c),
            None => None,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        match self.input_iter.next() {
            Some((_, c)) => Some(c),
            None => None,
        }
    }

    fn tokenize_symbol(&mut self, start: usize) -> Token {
        while let Some(&(i, c)) = self.input_iter.peek() {
            if c.is_ascii_alphanumeric() {
                self.input_iter.next();
                continue;
            } else {
                return Token::Symbol(&self.input[start..i]);
            }
        }
        Token::Symbol(&self.input[start..])
    }

    fn tokenize_number(&mut self, start: usize) -> Token {
        while let Some(&(i, c)) = self.input_iter.peek() {
            if c.is_ascii_digit() {
                self.input_iter.next();
                continue;
            } else {
                let val: i64 = self.input[start..i].parse().unwrap();
                return Token::Number(val);
            }
        }

        let val: i64 = self.input[start..].parse().unwrap();
        Token::Number(val)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_ascii_whitespace() {
                self.next_char();
            } else {
                return;
            }
        }
    }
}

/*
pub fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let mut tokens = Vec::new();
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => tokens.push(Token::Par),
            ')' => tokens.push(Token::RParen),
            '0'...'9' => tokens.push(tokenize_number(&input[i..])),
            'a'...'z' | 'A'...'Z' | '_' => tokens.push(tokenize_symbol(&input[i..])),
            _ => {
            }
        }
    }

    tokens
}

fn tokenize_symbol(input: &str) -> Token {
    let mut len: usize = 0;
    for c in input.chars() {
        if c.is_ascii_alphanumeric() || c.is_ascii_punctuation() {
            len += 1;
        }
    }

    Token::Symbol(&input[..len])
}

fn tokenize_number(input: &str) -> Token {
    let mut collector = String::new();
    for c in input.chars() {
        if c.is_ascii_digit() {
            collector.push(c);
        }
    }

    Token::Number(collector.parse().unwrap())
}
*/
