use std::iter::Peekable;

use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum ASTNode {
    Document(Vec<ASTNode>),
    Selector {
        tag: Option<String>,
        id: Option<String>,
        classes: Vec<String>,
        children: Vec<ASTNode>,
    },
    StyleRuleset {
        declarations: Vec<(String, String)>,
    },
}

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
        match self.lexer.peek() {
            Some(Token::Identifier(_)) => self.parse_selector_or_ruleset(),
            Some(token) if matches!(token, Token::Dot | Token::Hash) => self.parse_selector(None),
            Some(token) => Err(ParsingError::UnexpectedToken(format!("{:?}", token))),
            None => Err(ParsingError::UnexpectedEndOfInput),
        }
    }

    fn parse_selector_or_ruleset(&mut self) -> Result<ASTNode, ParsingError> {
        let identifier = match self.lexer.next() {
            Some(Token::Identifier(identifier)) => Some(identifier),
            _ => {
                return Err(ParsingError::UnexpectedToken(
                    "Expected identifier".to_string(),
                ))
            }
        };

        match self.lexer.peek() {
            Some(Token::Colon) => self.parse_ruleset(identifier),
            _ => self.parse_selector(identifier),
        }
    }

    fn parse_selector(&mut self, mut tag: Option<String>) -> Result<ASTNode, ParsingError> {
        let mut id = None;
        let mut classes = Vec::new();

        loop {
            match self.lexer.next() {
                Some(Token::Identifier(identifier)) => {
                    if tag.is_some() {
                        panic!("Unexpected identifier");
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
                Some(Token::LeftBrace) => {
                    let children = self.parse_selector_body()?;

                    return Ok(ASTNode::Selector {
                        tag,
                        id,
                        classes,
                        children,
                    });
                }
                Some(token) => return Err(ParsingError::UnexpectedToken(format!("{:?}", token))),
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

    fn parse_ruleset(
        &mut self,
        first_declaration_property: Option<String>,
    ) -> Result<ASTNode, ParsingError> {
        let mut declarations = Vec::new();

        if let Some(first_declaration_property) = first_declaration_property {
            declarations.push(self.parse_ruleset_first_declaration(first_declaration_property)?);
        }

        while let Some(token) = self.lexer.peek() {
            match token {
                Token::RightBrace => {
                    self.lexer.next();
                    return Ok(ASTNode::StyleRuleset { declarations });
                }
                _ => {
                    let (property, value) = self.parse_ruleset_declaration()?;
                    declarations.push((property, value));
                }
            }
        }

        Ok(ASTNode::StyleRuleset { declarations })
    }

    fn parse_ruleset_first_declaration(
        &mut self,
        first_declaration_property: String,
    ) -> Result<RulesetDeclaration, ParsingError> {
        self.expect_token(Token::Colon)?;

        let value = self.parse_ruleset_declaration_value()?;

        self.expect_token(Token::Semicolon)?;

        Ok((first_declaration_property, value))
    }

    fn parse_ruleset_declaration(&mut self) -> Result<RulesetDeclaration, ParsingError> {
        let property = match self.lexer.next().unwrap() {
            Token::Identifier(property) => property,
            _ => panic!("Expected property identifier"),
        };

        self.expect_token(Token::Colon)?;

        let value = self.parse_ruleset_declaration_value()?;

        self.expect_token(Token::Semicolon)?;

        Ok((property, value))
    }

    fn parse_ruleset_declaration_value(&mut self) -> Result<String, ParsingError> {
        match self.lexer.next() {
            Some(Token::Identifier(value) | Token::Text(value)) => Ok(value),
            _ => Err(ParsingError::UnexpectedToken("Expected value".to_string())),
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
