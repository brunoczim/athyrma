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
}

impl PartialEq<Token> for TokenKind {
    fn eq(&self, other: &Token) -> bool {
        *self == other.kind()
    }
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
    type Item = Symbol<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = loop {
            let current = self.input_stream.peek().copied()?;
            if !current.data.is_whitespace() {
                break current;
            }
        };

        match current {
            ',' => {},
            ';' => {},
            '=' => {},
            '"' | '\'' => {},
            _ if current.data.is_alphabetic() => {},
        }
    }
}
