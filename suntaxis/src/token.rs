use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
    Comment(Arc<str>),
    Terminal(Arc<str>),
    NonTerm(Arc<str>),
    Special(Arc<str>),
    Number(u128),
    Equal,
    Comma,
    Semicolon,
    Pipe,
    Except,
    Times,
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
            Self::Comment(_) => TokenKind::Comment,
            Self::Terminal(_) => TokenKind::Terminal,
            Self::NonTerm(_) => TokenKind::NonTerm,
            Self::Special(_) => TokenKind::Special,
            Self::Number(_) => TokenKind::Number,
            Self::Equal => TokenKind::Equal,
            Self::Comma => TokenKind::Comma,
            Self::Semicolon => TokenKind::Semicolon,
            Self::Pipe => TokenKind::Pipe,
            Self::Except => TokenKind::Except,
            Self::Times => TokenKind::Times,
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
    Comment,
    Terminal,
    NonTerm,
    Number,
    Special,
    Equal,
    Comma,
    Semicolon,
    Pipe,
    Except,
    Times,
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
