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
        match self.lexer.peek().unwrap() {
            Token::Identifier(_) => self.parse_selector_or_ruleset(),
            token if matches!(token, Token::Dot | Token::Hash) => self.parse_selector(None),
            _ => panic!("Unexpected token"),
        }
    }

    fn parse_selector_or_ruleset(&mut self) -> ASTNode {
        let identifier = match self.lexer.next().unwrap() {
            Token::Identifier(identifier) => Some(identifier),
            _ => panic!("Unexpected token"),
        };

        match self.lexer.peek() {
            Some(Token::Colon) => self.parse_ruleset(identifier),
            _ => self.parse_selector(identifier),
        }
    }

    fn parse_selector(&mut self, mut tag: Option<String>) -> ASTNode {
        let mut id = None;
        let mut classes = Vec::new();

        while let Some(token) = self.lexer.next() {
            match token {
                Token::Identifier(i) => {
                    if tag.is_some() {
                        panic!("Unexpected identifier");
                    }

                    tag = Some(i);
                }
                Token::Dot => {
                    if let Token::Identifier(class) = self.lexer.next().unwrap() {
                        classes.push(class);
                    } else {
                        panic!("Expected identifier after .");
                    }
                }
                Token::Hash => {
                    if let Token::Identifier(i) = self.lexer.next().unwrap() {
                        id = Some(i);
                    } else {
                        panic!("Expected identifier after #");
                    }
                }
                Token::LeftBrace => {
                    let children: Vec<ASTNode> = self.parse_selector_body();

                    return ASTNode::Selector {
                        tag,
                        id,
                        classes,
                        children,
                    };
                }
                _ => panic!("Unexpected token"),
            }
        }

        panic!("Unexpected end of input");
    }

    fn parse_selector_body(&mut self) -> Vec<ASTNode> {
        let mut nodes = Vec::new();

        while let Some(token) = self.lexer.peek() {
            match token {
                Token::RightBrace => {
                    self.lexer.next();
                    return nodes;
                }
                _ => nodes.push(self.parse_node()),
            }
        }

        panic!("Unexpected end of input");
    }

    fn parse_ruleset(&mut self, first_declaration_propery: Option<String>) -> ASTNode {
        let mut declarations = Vec::new();

        if let Some(first_declaration_propery) = first_declaration_propery {
            declarations.push(self.parse_ruleset_first_declaration(first_declaration_propery));
        }

        while let Some(token) = self.lexer.peek() {
            match token {
                Token::RightBrace => {
                    self.lexer.next();
                    return ASTNode::StyleRuleset { declarations };
                }
                _ => {
                    let (property, value) = self.parse_ruleset_declaration();
                    declarations.push((property, value));
                }
            }
        }

        ASTNode::StyleRuleset { declarations }
    }

    fn parse_ruleset_first_declaration(
        &mut self,
        first_declaration_propery: String,
    ) -> RulesetDeclaration {
        match self.lexer.next().unwrap() {
            Token::Colon => (),
            _ => panic!("Expected colon"),
        }

        let value = match self.lexer.next().unwrap() {
            Token::Identifier(value) => value,
            Token::Text(value) => value,
            _ => panic!("Expected value"),
        };

        (first_declaration_propery, value)
    }

    fn parse_ruleset_declaration(&mut self) -> RulesetDeclaration {
        let property = match self.lexer.next().unwrap() {
            Token::Identifier(property) => property,
            _ => panic!("Expected property identifier"),
        };

        match self.lexer.next().unwrap() {
            Token::Colon => (),
            _ => panic!("Expected colon"),
        }

        let value = match self.lexer.next().unwrap() {
            Token::Identifier(value) => value,
            Token::Text(value) => value,
            _ => panic!("Expected value"),
        };

        (property, value)
    }
}
