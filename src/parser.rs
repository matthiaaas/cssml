use std::iter::Peekable;

use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum ASTNode {
    Document(Vec<ASTNode>),
    Element {
        tag: String,
        id: Option<String>,
        classes: Vec<String>,
        content: Option<String>,
        children: Vec<ASTNode>,
    },
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

    pub fn parse(&mut self) -> ASTNode {
        let mut nodes = Vec::new();

        while self.lexer.peek().is_some() {
            let node = self.parse_node();
            nodes.push(node);
        }

        ASTNode::Document(nodes)
    }

    fn parse_node(&mut self) -> ASTNode {
        println!("parse_node {:?}", self.lexer.peek());
        match self.lexer.next().unwrap() {
            Token::Identifier(tag) => self.parse_element_or_style_rule(tag),
            _ => panic!("Unexpected token"),
        }
    }

    fn parse_element_or_style_rule(&mut self, tag: String) -> ASTNode {
        let mut id = None;
        let mut classes = Vec::new();
        let mut content = None;

        while let Some(token) = self.lexer.peek() {
            match token {
                Token::Dot => {
                    self.lexer.next();
                    if let Token::Identifier(class) = self.lexer.next().unwrap() {
                        classes.push(class);
                    } else {
                        panic!("Expected identifier after .");
                    }
                }
                Token::Hash => {
                    self.lexer.next();
                    if let Token::Identifier(i) = self.lexer.next().unwrap() {
                        id = Some(i);
                    } else {
                        panic!("Expected identifier after #");
                    }
                }
                Token::LeftParen => {
                    self.lexer.next();
                    content = self.parse_element_constructor();
                }
                Token::LeftBrace => {
                    self.lexer.next();
                    return self.parse_element(tag, id, classes, content);
                }
                _ => {
                    self.lexer.next();
                }
            }
        }

        panic!("Unexpected end of input");
    }

    fn parse_element_constructor(&mut self) -> Option<String> {
        let mut content = None;

        while let Some(token) = self.lexer.next() {
            match token {
                Token::RightParen => {
                    return content;
                }
                Token::Identifier(t) => {
                    if content.is_none() {
                        content = Some(t);
                    } else {
                        content = Some(format!("{} {}", content.unwrap(), t));
                    }
                }
                _ => panic!("Unexpected token"),
            }
        }

        panic!("Unexpected end of input");
    }

    fn parse_element(
        &mut self,
        tag: String,
        id: Option<String>,
        classes: Vec<String>,
        content: Option<String>,
    ) -> ASTNode {
        let mut children = Vec::new();

        while let Some(token) = self.lexer.peek() {
            match token {
                Token::RightBrace => {
                    self.lexer.next();
                    break;
                }
                _ => children.push(self.parse_node()),
            }
        }

        ASTNode::Element {
            tag,
            id,
            classes,
            content,
            children,
        }
    }
}
