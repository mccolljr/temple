use crate::syntax::{
    error::ParseError,
    lex::{Lexer, Tok},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Render(String),
    Control(String),
    Content(String),
}

pub struct Parser<'p> {
    lexer: Lexer<'p>,
}

impl<'p> Parser<'p> {
    pub fn new(s: &'p str) -> Self {
        Self::new_with_lexer(Lexer::new(s))
    }

    pub fn new_with_lexer(lexer: Lexer<'p>) -> Self {
        Self { lexer }
    }

    pub fn parse_nodes(&mut self) -> Result<Vec<Node>, ParseError> {
        let mut nodes = Vec::new();

        let mut clearing_whitespace = false;
        loop {
            match self.lexer.next() {
                Some(Tok::Literal(mut s)) => {
                    if clearing_whitespace {
                        clearing_whitespace = false;
                        s = s.trim_start().into();
                    }
                    if !s.is_empty() {
                        nodes.push(Node::Content(s))
                    }
                }
                Some(Tok::ORenderBlock { clear_whitespace }) => {
                    if clear_whitespace {
                        let mut remove_it = false;
                        if let Some(Node::Content(ref mut s)) = nodes.last_mut() {
                            *s = s.trim_end().into();
                            remove_it = s.is_empty();
                        }
                        if remove_it {
                            nodes.pop().unwrap();
                        }
                    }
                    match self.lexer.next() {
                        Some(Tok::Literal(s)) => nodes.push(Node::Render(s)),
                        _ => return Err("expected render expression".into()),
                    }
                    match self.lexer.next() {
                        Some(Tok::CRenderBlock { clear_whitespace }) => {
                            clearing_whitespace = clear_whitespace
                        }
                        _ => return Err("expected '}}'".into()),
                    }
                }
                Some(Tok::OControlBlock { clear_whitespace }) => {
                    if clear_whitespace {
                        let mut remove_it = false;
                        if let Some(Node::Content(ref mut s)) = nodes.last_mut() {
                            *s = s.trim_end().into();
                            remove_it = s.is_empty();
                        }
                        if remove_it {
                            nodes.pop().unwrap();
                        }
                    }
                    match self.lexer.next() {
                        Some(Tok::Literal(s)) => nodes.push(Node::Control(s)),
                        _ => return Err("expected control statement".into()),
                    }
                    match self.lexer.next() {
                        Some(Tok::CControlBlock { clear_whitespace }) => {
                            clearing_whitespace = clear_whitespace
                        }
                        _ => return Err("expected '%}'".into()),
                    }
                }
                Some(Tok::CRenderBlock { .. }) => return Err("unexpected '}}'".into()),
                Some(Tok::CControlBlock { .. }) => return Err("unexpected '%}'".into()),
                None => break,
            }
        }

        Ok(nodes)
    }
}
