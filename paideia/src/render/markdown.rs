use super::{Format, Scope};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Markdown {
    needs_newline: bool,
    level: u32,
    indent_size: u32,
}

impl Default for Markdown {
    fn default() -> Self {
        Self::new(4)
    }
}

impl Markdown {
    pub fn new(indent_size: u32) -> Self {
        Self { needs_newline: false, level: 0, indent_size }
    }
}

impl Format for Markdown {
    fn write_str(
        &mut self,
        mut input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        for line in input.split_inclusive('\n') {
            if line.trim().len() > 0 {
                if self.session.needs_newline {
                    target.write_str("\n")?;
                }
                self.session.needs_newline = false;
                let indent_spaces = self.session.level.saturating_sub(1)
                    * self.session.indent_size;
                for _ in 0 .. indent_spaces {
                    target.write_str(" ")?;
                }
                target.write_str(line)?;
            } else {
                self.session.needs_newline = line.ends_with('\n');
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Nest;

impl Scope for Nest {
    fn enter<F, T>(&self, format: &mut Self::Format, consumer: F) -> T
    where
        F: FnOnce(&mut Self::Format) -> T,
    {
        format.level += 1;
        let output = consumer(format);
        format.level -= 1;
        output
    }
}
