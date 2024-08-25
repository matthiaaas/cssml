use std::iter::Peekable;

use crate::{
    ast::ASTNode,
    lexer::{Lexer, Token},
};

type RulesetDeclaration = (String, String);

#[derive(Debug)]
pub enum ParsingError {
    UnexpectedToken(String),
    UnexpectedEndOfInput,
    MissingIdentifier(String),
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            lexer: Lexer::new(input).peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<ASTNode, ParsingError> {
        let mut nodes = Vec::new();

        while self.lexer.peek().is_some() {
            let node = self.parse_node()?;
            nodes.push(node);
        }

        Ok(ASTNode::Document(nodes))
    }

    fn parse_node(&mut self) -> Result<ASTNode, ParsingError> {
        if self.is_selector()? {
            self.parse_selector()
        } else {
            self.parse_ruleset()
        }
    }

    fn is_selector(&mut self) -> Result<bool, ParsingError> {
        let mut peek_iter = self.lexer.clone();

        match peek_iter.next() {
            Some(Token::Identifier(_)) => match peek_iter.next() {
                Some(Token::Dot | Token::Hash) => Ok(true),
                Some(Token::Colon) => Ok(false),
                _ => Ok(true),
            },
            Some(Token::Dot | Token::Hash) => Ok(true),
            _ => Ok(false),
        }
    }

    fn parse_selector(&mut self) -> Result<ASTNode, ParsingError> {
        let mut tag = None;
        let mut id = None;
        let mut classes = Vec::new();
        let mut element = None;

        loop {
            match self.lexer.next() {
                Some(Token::Identifier(identifier)) => {
                    if tag.is_some() {
                        return Err(ParsingError::UnexpectedToken(
                            "Expected only one tag in selector".to_string(),
                        ));
                    }

                    tag = Some(identifier);
                }
                Some(Token::Dot) => {
                    if let Some(Token::Identifier(class)) = self.lexer.next() {
                        classes.push(class);
                    } else {
                        return Err(ParsingError::MissingIdentifier(
                            "Expected class name".to_string(),
                        ));
                    }
                }
                Some(Token::Hash) => {
                    if let Some(Token::Identifier(i)) = self.lexer.next() {
                        id = Some(i);
                    } else {
                        return Err(ParsingError::MissingIdentifier("Expected id".to_string()));
                    }
                }
                Some(Token::LeftParen) => {
                    element = Some(self.parse_element_constructor()?);
                }
                Some(Token::LeftBrace) => {
                    let children = self.parse_selector_body()?;

                    return Ok(ASTNode::Selector {
                        tag,
                        id,
                        classes,
                        element,
                        children,
                    });
                }
                Some(token) => return Err(ParsingError::UnexpectedToken(format!("{:?}", token))),
                None => return Err(ParsingError::UnexpectedEndOfInput),
            }
        }
    }

    fn parse_element_constructor(&mut self) -> Result<String, ParsingError> {
        let mut element = String::new();

        loop {
            match self.lexer.next() {
                Some(Token::Identifier(identifier) | Token::Text(identifier)) => {
                    if !element.is_empty() {
                        element.push_str(" ");
                    }

                    element.push_str(&identifier);
                }
                Some(Token::RightParen) => {
                    return Ok(element);
                }
                Some(token) => {
                    return Err(ParsingError::UnexpectedToken(format!("{:?}", token)));
                }
                None => return Err(ParsingError::UnexpectedEndOfInput),
            }
        }
    }

    fn parse_selector_body(&mut self) -> Result<Vec<ASTNode>, ParsingError> {
        let mut nodes = Vec::new();

        while let Some(token) = self.lexer.peek() {
            match token {
                Token::RightBrace => {
                    self.lexer.next();
                    break;
                }
                _ => nodes.push(self.parse_node()?),
            }
        }

        Ok(nodes)
    }

    fn parse_ruleset(&mut self) -> Result<ASTNode, ParsingError> {
        let mut declarations = Vec::new();

        loop {
            match self.lexer.peek() {
                Some(Token::RightBrace) => {
                    return Ok(ASTNode::StyleRuleset { declarations });
                }
                Some(_) => {
                    if self.is_selector()? {
                        return Ok(ASTNode::StyleRuleset { declarations });
                    } else {
                        let (property, value) = self.parse_ruleset_declaration()?;
                        declarations.push((property, value));
                    }
                }
                None => return Err(ParsingError::UnexpectedEndOfInput),
            }
        }
    }

    fn parse_ruleset_declaration(&mut self) -> Result<RulesetDeclaration, ParsingError> {
        let property = self.parse_ruleset_declaration_property()?;
        self.expect_token(Token::Colon)?;
        let value = self.parse_ruleset_declaration_value()?;
        self.expect_token(Token::Semicolon)?;

        Ok((property, value))
    }

    fn parse_ruleset_declaration_property(&mut self) -> Result<String, ParsingError> {
        match self.lexer.next() {
            Some(Token::Identifier(property)) => Ok(property),
            Some(token) => Err(ParsingError::UnexpectedToken(format!(
                "Expected property, got {:?}",
                token
            ))),
            None => Err(ParsingError::UnexpectedEndOfInput),
        }
    }

    fn parse_ruleset_declaration_value(&mut self) -> Result<String, ParsingError> {
        match self.lexer.next() {
            Some(Token::Identifier(value) | Token::Text(value)) => Ok(value),
            Some(token) => Err(ParsingError::UnexpectedToken(format!(
                "Expected value, got {:?}",
                token
            ))),
            None => Err(ParsingError::UnexpectedEndOfInput),
        }
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), ParsingError> {
        match self.lexer.next() {
            Some(token) if token == expected => Ok(()),
            Some(token) => Err(ParsingError::UnexpectedToken(format!(
                "Expected {:?}, got {:?}",
                expected, token
            ))),
            None => Err(ParsingError::UnexpectedEndOfInput),
        }
    }
}
