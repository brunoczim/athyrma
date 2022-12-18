use crate::{
    source::{Span, Symbol},
    token::Token,
};
use std::{iter, sync::Arc};

#[derive(Debug, Clone, Copy)]
pub enum LexError {
    Unrecognized(Symbol<char>),
    UnfinishedQuote(Symbol<char>),
    UnfinishedSpecial(Symbol<char>),
    UnclosedComment(Symbol<char>),
}

#[derive(Debug, Clone)]
pub struct Lexer<I>
where
    I: Iterator<Item = Symbol<char>>,
{
    source: iter::Peekable<I>,
}

impl<I> Lexer<I>
where
    I: Iterator<Item = Symbol<char>>,
{
    pub fn from_source(source: I) -> Self {
        Self { source: source.peekable() }
    }

    fn next_non_whitespace(&mut self) -> Option<Symbol<char>> {
        loop {
            let current = self.source.next()?;
            if !current.data.is_whitespace() {
                break Some(current);
            }
        }
    }

    fn lex_comma(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        Ok(Symbol { data: Token::Comma, span: first.span })
    }

    fn lex_semicolon(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        Ok(Symbol { data: Token::Semicolon, span: first.span })
    }

    fn lex_equal(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        Ok(Symbol { data: Token::Equal, span: first.span })
    }

    fn lex_pipe(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        Ok(Symbol { data: Token::Pipe, span: first.span })
    }

    fn lex_open_paren(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        match self.source.peek().copied() {
            Some(second) if second.data == '*' => {
                self.lex_comment(first, second)
            },
            _ => Ok(Symbol { data: Token::OpenParen, span: first.span }),
        }
    }

    fn lex_close_paren(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        Ok(Symbol { data: Token::CloseParen, span: first.span })
    }

    fn lex_open_square(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        Ok(Symbol { data: Token::OpenSquare, span: first.span })
    }

    fn lex_close_square(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        Ok(Symbol { data: Token::CloseSquare, span: first.span })
    }

    fn lex_open_curly(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        Ok(Symbol { data: Token::OpenCurly, span: first.span })
    }

    fn lex_close_curly(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        Ok(Symbol { data: Token::CloseCurly, span: first.span })
    }

    fn lex_terminal(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        self.lex_quoted(
            first,
            |data| Token::Terminal(Arc::from(data)),
            LexError::UnfinishedQuote,
        )
    }

    fn lex_special(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        self.lex_quoted(
            first,
            |data| Token::Special(Arc::from(data)),
            LexError::UnfinishedSpecial,
        )
    }

    fn lex_non_term(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        let mut span = first.span;
        let mut ident = String::new();
        let mut whitespace = false;
        loop {
            match self.source.peek() {
                Some(symbol) => {
                    if symbol.data.is_whitespace() {
                        whitespace = true;
                        self.source.next();
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
                            self.next_spanned(span.as_mut());
                        } else {
                            break;
                        }
                    }
                },
                None => break,
            }
        }
        Ok(Symbol { data: Token::NonTerm(Arc::from(ident)), span })
    }

    fn lex_comment(
        &mut self,
        first: Symbol<char>,
        _second: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        let mut stack_count = 1u128;
        let mut content = String::new();
        let mut span = first.span;

        let mut prev = None;
        loop {
            match self.next_spanned(span.as_mut()) {
                Some(symbol) => {
                    if let Some(prev_char) = prev {
                        content.push(prev_char);
                    }
                    if prev == Some('*') && symbol.data == ')' {
                        stack_count -= 1;
                        if stack_count == 0 {
                            break Ok(Symbol {
                                data: Token::Comment(Arc::from(content)),
                                span,
                            });
                        }
                    }
                    if prev == Some('(') && symbol.data == '*' {
                        stack_count += 1;
                    }
                    prev = Some(symbol.data);
                },
                None => break Err(LexError::UnclosedComment(first)),
            }
        }
    }

    fn lex_quoted<T, E>(
        &mut self,
        quote: Symbol<char>,
        make_token: T,
        make_error: E,
    ) -> Result<Symbol<Token>, LexError>
    where
        T: FnOnce(String) -> Token,
        E: FnOnce(Symbol<char>) -> LexError,
    {
        let mut quoted = String::new();
        let mut span = quote.span;
        let mut escape = false;
        loop {
            match self.next_spanned(span.as_mut()) {
                Some(symbol) => {
                    if escape {
                        escape = false;
                        quoted.push(match symbol.data {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            _ => symbol.data,
                        });
                    } else if symbol.data == quote.data {
                        break Ok(Symbol { data: make_token(quoted), span });
                    } else if symbol.data == '\\' {
                        escape = true;
                    } else {
                        quoted.push(symbol.data);
                    }
                },
                None => break Err(make_error(quote)),
            }
        }
    }

    fn next_spanned(
        &mut self,
        maybe_span: Option<&mut Span>,
    ) -> Option<Symbol<char>> {
        let symbol = self.source.next();
        if let Some(span) = maybe_span.filter(|_| symbol.is_some()) {
            span.length += 1;
        }
        symbol
    }
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = Symbol<char>>,
{
    type Item = Result<Symbol<Token>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.next_non_whitespace()?;

        let result = match first.data {
            ',' => self.lex_comma(first),
            ';' => self.lex_semicolon(first),
            '=' => self.lex_equal(first),
            '|' => self.lex_pipe(first),
            '(' => self.lex_open_paren(first),
            ')' => self.lex_close_paren(first),
            '[' => self.lex_open_square(first),
            ']' => self.lex_close_square(first),
            '{' => self.lex_open_curly(first),
            '}' => self.lex_close_curly(first),
            '"' | '\'' => self.lex_terminal(first),
            '?' => self.lex_special(first),
            '$' => self.lex_non_term(first),
            _ if first.data.is_alphabetic() => self.lex_non_term(first),
            _ => Err(LexError::Unrecognized(first)),
        };

        Some(result)
    }
}
