use std::{iter, str, sync::Arc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    pub line: u128,
    pub column: u128,
}

impl Default for Location {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub location: Location,
    pub length: u128,
}

impl Default for Span {
    fn default() -> Self {
        Self::from(Location::default())
    }
}

impl From<Location> for Span {
    fn from(location: Location) -> Self {
        Self { location, length: 1 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol<T> {
    pub data: T,
    pub span: Option<Span>,
}

#[derive(Debug, Clone)]
pub struct InputStream<I>
where
    I: Iterator<Item = char>,
{
    location: Location,
    input: I,
}

impl<I> InputStream<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(input: I) -> Self {
        Self { location: Location::default(), input }
    }
}

impl<I> Iterator for InputStream<I>
where
    I: Iterator<Item = char>,
{
    type Item = Symbol<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let character = self.input.next()?;
        let span = self.location.into();
        if character == '\n' {
            self.location.line += 1;
            self.location.column = 1;
        } else {
            self.location.column += 1;
        }
        Some(Symbol { data: character, span: Some(span) })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
    Terminal(Arc<str>),
    NonTerm(Arc<str>),
    Special(Arc<str>),
    Equal,
    Comma,
    Semicolon,
    Pipe,
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        match self {
            Self::Terminal(_) => TokenKind::Terminal,
            Self::NonTerm(_) => TokenKind::NonTerm,
            Self::Special(_) => TokenKind::Special,
            Self::Equal => TokenKind::Equal,
            Self::Comma => TokenKind::Comma,
            Self::Semicolon => TokenKind::Semicolon,
            Self::Pipe => TokenKind::Pipe,
            Self::OpenParen => TokenKind::OpenParen,
            Self::CloseParen => TokenKind::CloseParen,
            Self::OpenSquare => TokenKind::OpenSquare,
            Self::CloseSquare => TokenKind::CloseSquare,
            Self::OpenCurly => TokenKind::OpenCurly,
            Self::CloseCurly => TokenKind::CloseCurly,
        }
    }
}

impl PartialEq<TokenKind> for Token {
    fn eq(&self, other: &TokenKind) -> bool {
        self.kind() == *other
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    Terminal,
    NonTerm,
    Special,
    Equal,
    Comma,
    Semicolon,
    Pipe,
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
}

impl PartialEq<Token> for TokenKind {
    fn eq(&self, other: &Token) -> bool {
        *self == other.kind()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LexError {
    Unrecognized(Symbol<char>),
    UnfinishedQuote(Symbol<char>),
    UnfinishedSpecial(Symbol<char>),
}

#[derive(Debug, Clone)]
pub struct Lexer<I>
where
    I: Iterator<Item = Symbol<char>>,
{
    input_stream: iter::Peekable<I>,
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = Symbol<char>>,
{
    type Item = Result<Symbol<Token>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = loop {
            let current = self.input_stream.peek().copied()?;
            if !current.data.is_whitespace() {
                break current;
            }
        };

        Some(match current.data {
            ',' => Ok(Symbol { data: Token::Comma, span: current.span }),
            ';' => Ok(Symbol { data: Token::Semicolon, span: current.span }),
            '=' => Ok(Symbol { data: Token::Equal, span: current.span }),
            '|' => Ok(Symbol { data: Token::Pipe, span: current.span }),
            '(' => Ok(Symbol { data: Token::OpenParen, span: current.span }),
            ')' => Ok(Symbol { data: Token::CloseParen, span: current.span }),
            '[' => Ok(Symbol { data: Token::OpenSquare, span: current.span }),
            ']' => Ok(Symbol { data: Token::CloseSquare, span: current.span }),
            '{' => Ok(Symbol { data: Token::OpenCurly, span: current.span }),
            '}' => Ok(Symbol { data: Token::CloseCurly, span: current.span }),
            '"' | '\'' => {
                let mut terminal = String::new();
                let mut escape = false;
                loop {
                    match self.input_stream.next() {
                        Some(character) => {
                            if escape {
                                escape = false;
                                terminal.push(match character.data {
                                    'n' => '\n',
                                    't' => '\t',
                                    'r' => '\r',
                                    _ => character.data,
                                })
                            } else if character.data == current.data {
                                break Ok(Symbol {
                                    data: Token::Terminal(Arc::from(terminal)),
                                    span: current.span,
                                });
                            } else if character.data == '\\' {
                                escape = true;
                            } else {
                                terminal.push(character.data);
                            }
                        },
                        None => break Err(LexError::UnfinishedQuote(current)),
                    }
                }
            },
            '?' => {
                let mut special = String::new();
                let mut escape = false;
                loop {
                    match self.input_stream.next() {
                        Some(character) => {
                            if escape {
                                escape = false;
                                special.push(match character.data {
                                    'n' => '\n',
                                    't' => '\t',
                                    'r' => '\r',
                                    _ => character.data,
                                })
                            } else if character.data == current.data {
                                break Ok(Symbol {
                                    data: Token::Special(Arc::from(special)),
                                    span: current.span,
                                });
                            } else if character.data == '\\' {
                                escape = true;
                            } else {
                                special.push(character.data);
                            }
                        },
                        None => {
                            break Err(LexError::UnfinishedSpecial(current))
                        },
                    }
                }
            },
            _ if current.data.is_alphabetic() || current.data == '$' => {
                let mut ident = String::new();
                let mut whitespace = false;
                loop {
                    match self.input_stream.peek() {
                        Some(symbol) => {
                            if symbol.data.is_whitespace() {
                                whitespace = true;
                                self.input_stream.next();
                            } else {
                                let should_continue =
                                    matches!(symbol.data, '_' | '-' | '$')
                                        || symbol.data.is_alphanumeric();
                                if should_continue {
                                    if whitespace {
                                        whitespace = false;
                                        ident.push(' ');
                                    }
                                    ident.push(symbol.data);
                                    self.input_stream.next();
                                } else {
                                    break;
                                }
                            }
                        },
                        None => break,
                    }
                }
                let length = u128::try_from(ident.chars().count())
                    .expect("token length is too big");
                Ok(Symbol {
                    data: Token::NonTerm(Arc::from(ident)),
                    span: current
                        .span
                        .map(|span| Span { location: span.location, length }),
                })
            },
            _ => Err(LexError::Unrecognized(current)),
        })
    }
}
