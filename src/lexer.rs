use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Colon,
    Semicolon,
    Dot,
    Hash,
    Text(String),
}

impl Token {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '{' => Some(Self::LeftBrace),
            '}' => Some(Self::RightBrace),
            '(' => Some(Self::LeftParen),
            ')' => Some(Self::RightParen),
            ':' => Some(Self::Colon),
            ';' => Some(Self::Semicolon),
            '.' => Some(Self::Dot),
            '#' => Some(Self::Hash),
            _ => None,
        }
    }

    fn uniquely_determined_by(c: char) -> bool {
        Self::from_char(c).is_some()
    }
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        match self.input.next() {
            Some(c) => Token::from_char(c).or_else(|| {
                if c.is_alphabetic() {
                    Some(self.read_identifier(c))
                } else {
                    Some(self.read_text(c))
                }
            }),
            None => None,
        }
    }

    fn read_identifier(&mut self, first_char: char) -> Token {
        let mut identifier = String::from(first_char);

        while let Some(&c) = self.input.peek() {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                identifier.push(self.input.next().unwrap());
            } else {
                break;
            }
        }

        Token::Identifier(identifier)
    }

    fn read_text(&mut self, first_char: char) -> Token {
        let mut text = String::from(first_char);

        while let Some(&c) = self.input.peek() {
            if !c.is_whitespace() && !Token::uniquely_determined_by(c) {
                text.push(self.input.next().unwrap());
            } else {
                break;
            }
        }

        Token::Text(text.trim().to_string())
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if !c.is_whitespace() {
                break;
            }

            self.input.next();
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl<'a> Clone for Lexer<'a> {
    fn clone(&self) -> Self {
        Lexer {
            input: self.input.clone(),
        }
    }
}
