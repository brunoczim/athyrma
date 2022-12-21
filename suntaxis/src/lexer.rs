use crate::{
    source::{Span, Symbol},
    token::Token,
};
use core::fmt;
use std::{iter, sync::Arc};

#[derive(Debug, Clone, Copy)]
pub enum LexError {
    Unrecognized(Symbol<char>),
    UnfinishedQuote(Symbol<char>),
    UnfinishedSpecial(Symbol<char>),
    UnclosedComment(Symbol<char>),
    NumberTooBig(Span),
}

impl fmt::Display for LexError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unrecognized(symbol) => {
                write!(fmtr, "unrecognized character {:?}", symbol.data)?;
                if let Some(span) = symbol.span {
                    write!(fmtr, ", {}", span)?;
                }
                Ok(())
            },

            Self::UnfinishedQuote(symbol) => {
                write!(
                    fmtr,
                    "unfinished quoting wtih starting character {:?}",
                    symbol.data
                )?;
                if let Some(span) = symbol.span {
                    write!(fmtr, ", {}", span)?;
                }
                Ok(())
            },

            Self::UnfinishedSpecial(symbol) => {
                write!(
                    fmtr,
                    "unfinished special symbol wtih starting character {:?}",
                    symbol.data
                )?;
                if let Some(span) = symbol.span {
                    write!(fmtr, ", {}", span)?;
                }
                Ok(())
            },

            Self::UnclosedComment(symbol) => {
                write!(
                    fmtr,
                    "unfinished comment wtih starting character {:?}",
                    symbol.data
                )?;
                if let Some(span) = symbol.span {
                    write!(fmtr, ", {}", span)?;
                }
                Ok(())
            },

            Self::NumberTooBig(span) => {
                write!(fmtr, "number too big")?;
                if let Some(span) = symbol.span {
                    write!(fmtr, ", {}", span)?;
                }
                Ok(())
            },
        }
    }
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

    fn lex_times(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        Ok(Symbol { data: Token::Times, span: first.span })
    }

    fn lex_except(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        Ok(Symbol { data: Token::Except, span: first.span })
    }

    fn lex_open_paren(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        match self.source.peek().copied() {
            Some(second) if second.data == '*' => {
                self.source.next();
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
        let mut ident = String::from(first.data);
        let mut whitespace = false;
        let mut span_end = None;

        loop {
            match self.source.peek() {
                Some(symbol) => {
                    if symbol.data.is_whitespace() {
                        whitespace = true;
                        self.source.next();
                    } else {
                        let should_continue = matches!(symbol.data, '_' | '$')
                            || symbol.data.is_alphanumeric();
                        if should_continue {
                            if whitespace {
                                whitespace = false;
                                ident.push(' ');
                            }
                            ident.push(symbol.data);
                            span_end = span_end.max(symbol.span.map(Span::end));
                            self.source.next();
                        } else {
                            break;
                        }
                    }
                },
                None => break,
            }
        }

        Ok(Symbol {
            data: Token::NonTerm(Arc::from(ident)),
            span: first
                .span
                .map(Span::start)
                .zip(span_end)
                .map(|(start, end)| Span::new_inclusive(start, end)),
        })
    }

    fn lex_number(
        &mut self,
        first: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        let mut number = u128::from(first.data.to_digit(10).unwrap());
        let mut exponent = 1;

        loop {
            match self.source.peek().copied() {
                Some(symbol) => match symbol.data.to_digit(10) {
                    Some(digit) => {
                        number += 10u128.pow(exponent) * u128::from(digit);
                        exponent += 1;
                        self.source.next();
                    },
                    None => break,
                },
                None => break,
            }
        }

        Ok(Symbol { data: Token::Number(number), span: first.span })
    }

    fn lex_comment(
        &mut self,
        first: Symbol<char>,
        _second: Symbol<char>,
    ) -> Result<Symbol<Token>, LexError> {
        let mut stack_count = 1u128;
        let mut content = String::new();
        let mut prev = None;
        let mut span_end = None;

        loop {
            match self.source.next() {
                Some(symbol) => {
                    span_end = span_end.max(symbol.span.map(Span::end));
                    if prev == Some('*') && symbol.data == ')' {
                        stack_count -= 1;
                        if stack_count == 0 {
                            break Ok(Symbol {
                                data: Token::Comment(Arc::from(content)),
                                span: first
                                    .span
                                    .map(Span::start)
                                    .zip(span_end)
                                    .map(|(start, end)| {
                                        Span::new_inclusive(start, end)
                                    }),
                            });
                        }
                    }
                    if prev == Some('(') && symbol.data == '*' {
                        stack_count += 1;
                    }
                    if let Some(prev_char) = prev {
                        content.push(prev_char);
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
        let mut span_end = None;
        let mut escape = false;

        loop {
            match self.source.next() {
                Some(symbol) => {
                    span_end = span_end.max(symbol.span.map(Span::end));
                    if escape {
                        escape = false;
                        quoted.push(match symbol.data {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            _ => symbol.data,
                        });
                    } else if symbol.data == quote.data {
                        break Ok(Symbol {
                            data: make_token(quoted),
                            span: quote
                                .span
                                .map(Span::start)
                                .zip(span_end)
                                .map(|(start, end)| {
                                    Span::new_inclusive(start, end)
                                }),
                        });
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
            '*' => self.lex_times(first),
            '-' => self.lex_except(first),
            '(' => self.lex_open_paren(first),
            ')' => self.lex_close_paren(first),
            '[' => self.lex_open_square(first),
            ']' => self.lex_close_square(first),
            '{' => self.lex_open_curly(first),
            '}' => self.lex_close_curly(first),
            '"' | '\'' => self.lex_terminal(first),
            '?' => self.lex_special(first),
            '$' => self.lex_non_term(first),
            '_' => self.lex_non_term(first),
            _ if first.data.is_alphabetic() => self.lex_non_term(first),
            _ if first.data.is_digit(10) => self.lex_number(first),
            _ => Err(LexError::Unrecognized(first)),
        };

        Some(result)
    }
}

#[cfg(test)]
mod test {
    use super::Lexer;
    use crate::{source::Source, token::Token};
    use std::sync::Arc;

    #[test]
    fn every_token() {
        let input = "hello \t there = \"double\" | 'single', (* comment *) () \
                     [] {} ?special? ;";
        let tokens = Lexer::from_source(Source::new(input.chars()))
            .map(|result| result.map(|symbol| symbol.data))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(tokens, &[
            Token::NonTerm(Arc::from("hello there")),
            Token::Equal,
            Token::Terminal(Arc::from("double")),
            Token::Pipe,
            Token::Terminal(Arc::from("single")),
            Token::Comma,
            Token::Comment(Arc::from(" comment ")),
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenSquare,
            Token::CloseSquare,
            Token::OpenCurly,
            Token::CloseCurly,
            Token::Special(Arc::from("special")),
            Token::Semicolon,
        ]);
    }

    #[test]
    fn non_terminals() {
        let input = "hello \t there, I am here , bye";
        let tokens = Lexer::from_source(Source::new(input.chars()))
            .map(|result| result.map(|symbol| symbol.data))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(tokens, &[
            Token::NonTerm(Arc::from("hello there")),
            Token::Comma,
            Token::NonTerm(Arc::from("I am here")),
            Token::Comma,
            Token::NonTerm(Arc::from("bye")),
        ]);
    }

    #[test]
    fn terminals() {
        let input = "\"hello \t there\", 'I am \\'\\\"\\\\ here\n'";
        let tokens = Lexer::from_source(Source::new(input.chars()))
            .map(|result| result.map(|symbol| symbol.data))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(tokens, &[
            Token::Terminal(Arc::from("hello \t there")),
            Token::Comma,
            Token::Terminal(Arc::from("I am '\"\\ here\n")),
        ]);
    }

    #[test]
    fn specials() {
        let input = "?hello \t there?, ?I am \\'\\\"\\\\\\? here\n?";
        let tokens = Lexer::from_source(Source::new(input.chars()))
            .map(|result| result.map(|symbol| symbol.data))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(tokens, &[
            Token::Special(Arc::from("hello \t there")),
            Token::Comma,
            Token::Special(Arc::from("I am '\"\\? here\n")),
        ]);
    }
}
