use std::iter::Peekable;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
}

impl From<String> for ParseError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl<'s> From<&'s str> for ParseError {
    fn from(message: &'s str) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tok {
    Literal(String),
    ORenderBlock { clear_whitespace: bool },
    CRenderBlock { clear_whitespace: bool },
    OControlBlock { clear_whitespace: bool },
    CControlBlock { clear_whitespace: bool },
}

pub struct Lexer<'lex> {
    chars: std::str::Chars<'lex>,
    peek1: Option<char>,
    peek2: Option<char>,
    peek3: Option<char>,
    in_block: bool,
}

impl<'lex> Lexer<'lex> {
    pub fn new(src: &'lex str) -> Self {
        let mut chars = src.chars();
        let peek1 = chars.next();
        let peek2 = chars.next();
        let peek3 = chars.next();
        Self {
            chars,
            peek1,
            peek2,
            peek3,
            in_block: false,
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
        let next = self.peek1;
        self.peek1 = self.peek2;
        self.peek2 = self.peek3;
        self.peek3 = self.chars.next();
        next
    }

    pub fn has_next_char(&mut self, c: char) -> bool {
        self.peek1 == Some(c)
    }

    pub fn has_next_char2(&mut self, c1: char, c2: char) -> bool {
        return self.peek1 == Some(c1) && self.peek2 == Some(c2);
    }

    pub fn has_next_char3(&mut self, c1: char, c2: char, c3: char) -> bool {
        return self.peek1 == Some(c1) && self.peek2 == Some(c2) && self.peek3 == Some(c3);
    }

    pub fn with_next_char(&mut self, c: char) -> bool {
        if self.has_next_char(c) {
            self.next_char().unwrap();
            return true;
        }
        false
    }
    pub fn with_next_chars(&mut self, c1: char, c2: char) -> bool {
        if self.has_next_char2(c1, c2) {
            self.next_char().unwrap();
            self.next_char().unwrap();
            return true;
        }
        false
    }
}

impl<'lex> Iterator for Lexer<'lex> {
    type Item = Tok;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_char()? {
            '{' if !self.in_block && self.with_next_char('{') => {
                self.in_block = true;
                let clear_whitespace = self.with_next_char('-');
                Some(Tok::ORenderBlock { clear_whitespace })
            }
            '{' if !self.in_block && self.with_next_char('%') => {
                self.in_block = true;
                let clear_whitespace = self.with_next_char('-');
                Some(Tok::OControlBlock { clear_whitespace })
            }
            '}' if self.in_block && self.with_next_char('}') => {
                self.in_block = false;
                let clear_whitespace = false;
                Some(Tok::CRenderBlock { clear_whitespace })
            }
            '-' if self.in_block && self.with_next_chars('}', '}') => {
                self.in_block = false;
                let clear_whitespace = true;
                Some(Tok::CRenderBlock { clear_whitespace })
            }
            '%' if self.in_block && self.with_next_char('}') => {
                self.in_block = false;
                let clear_whitespace = false;
                Some(Tok::CControlBlock { clear_whitespace })
            }
            '-' if self.in_block && self.with_next_chars('%', '}') => {
                self.in_block = false;
                let clear_whitespace = true;
                Some(Tok::CControlBlock { clear_whitespace })
            }
            c => {
                let mut literal = String::new();
                literal.push(c);

                loop {
                    if self.in_block
                        && (self.has_next_char2('%', '}')
                            || self.has_next_char2('}', '}')
                            || self.has_next_char3('-', '}', '}')
                            || self.has_next_char3('-', '%', '}'))
                    {
                        break;
                    }

                    if !self.in_block
                        && (self.has_next_char2('{', '%') || self.has_next_char2('{', '{'))
                    {
                        break;
                    }

                    match self.next_char() {
                        Some(c) => literal.push(c),
                        None => break,
                    }
                }

                Some(Tok::Literal(literal))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Render(String),
    Control(String),
    Content(String),
}

pub struct Parser<'p> {
    lexer: Peekable<Lexer<'p>>,
}

impl<'p> Parser<'p> {
    pub fn new(s: &'p str) -> Self {
        Self::new_with_lexer(Lexer::new(s))
    }

    pub fn new_with_lexer(lexer: Lexer<'p>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
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
