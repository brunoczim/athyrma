use super::{
    text_commons::{self, TextCommons},
    Format,
    Scope,
};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Markdown {
    inner: TextCommons,
}

impl Markdown {
    pub fn new(indent_size: u32) -> Self {
        Self { inner: TextCommons::new(indent_size) }
    }
}

impl Format for Markdown {
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        self.inner.write_str(input, target)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Nest;

impl Scope for Nest {
    type Format = Markdown;

    fn enter<F, T>(&self, format: &mut Self::Format, consumer: F) -> T
    where
        F: FnOnce(&mut Self::Format) -> T,
    {
        text_commons::Nest.enter(&mut format.inner, |inner| {
            let mut copy = Markdown { inner: *inner };
            let output = consumer(&mut copy);
            *inner = copy.inner;
            output
        })
    }
}
