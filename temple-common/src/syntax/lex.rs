use crate::syntax::util::Lookahead;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tok {
    Literal(String),
    ORenderBlock { clear_whitespace: bool },
    CRenderBlock { clear_whitespace: bool },
    OControlBlock { clear_whitespace: bool },
    CControlBlock { clear_whitespace: bool },
}

pub struct Lexer<'lex> {
    chars: Lookahead<std::str::Chars<'lex>, 3>,
    in_block: bool,
}

impl<'lex> Lexer<'lex> {
    pub fn new(src: &'lex str) -> Self {
        Self {
            chars: Lookahead::new(src.chars()),
            in_block: false,
        }
    }
}

impl<'lex> Iterator for Lexer<'lex> {
    type Item = Tok;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next()? {
            '{' if !self.in_block && self.chars.with_next(&['{']) => {
                self.in_block = true;
                let clear_whitespace = self.chars.with_next(&['-']);
                Some(Tok::ORenderBlock { clear_whitespace })
            }
            '{' if !self.in_block && self.chars.with_next(&['%']) => {
                self.in_block = true;
                let clear_whitespace = self.chars.with_next(&['-']);
                Some(Tok::OControlBlock { clear_whitespace })
            }
            '}' if self.in_block && self.chars.with_next(&['}']) => {
                self.in_block = false;
                let clear_whitespace = false;
                Some(Tok::CRenderBlock { clear_whitespace })
            }
            '-' if self.in_block && self.chars.with_next(&['}', '}']) => {
                self.in_block = false;
                let clear_whitespace = true;
                Some(Tok::CRenderBlock { clear_whitespace })
            }
            '%' if self.in_block && self.chars.with_next(&['}']) => {
                self.in_block = false;
                let clear_whitespace = false;
                Some(Tok::CControlBlock { clear_whitespace })
            }
            '-' if self.in_block && self.chars.with_next(&['%', '}']) => {
                self.in_block = false;
                let clear_whitespace = true;
                Some(Tok::CControlBlock { clear_whitespace })
            }
            c => {
                let mut literal = String::new();
                literal.push(c);

                loop {
                    if self.in_block
                        && (self.chars.has_next(&['%', '}'])
                            || self.chars.has_next(&['}', '}'])
                            || self.chars.has_next(&['-', '}', '}'])
                            || self.chars.has_next(&['-', '%', '}']))
                    {
                        break;
                    }

                    if !self.in_block
                        && (self.chars.has_next(&['{', '%']) || self.chars.has_next(&['{', '{']))
                    {
                        break;
                    }

                    match self.chars.next() {
                        Some(c) => literal.push(c),
                        None => break,
                    }
                }

                if self.in_block {
                    literal = literal.trim().into();
                }

                Some(Tok::Literal(literal))
            }
        }
    }
}
