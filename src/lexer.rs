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
            Some('{') => Some(Token::LeftBrace),
            Some('}') => Some(Token::RightBrace),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            Some(':') => Some(Token::Colon),
            Some(';') => Some(Token::Semicolon),
            Some('.') => Some(Token::Dot),
            Some('#') => Some(Token::Hash),
            Some(c) if c.is_alphabetic() => Some(self.read_identifier(c)),
            Some(c) => Some(self.read_text(c)),
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
            if c != '{' && c != '}' && c != '(' && c != ')' {
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
