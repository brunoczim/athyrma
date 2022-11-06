use super::{Format, Scope};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Text {
    needs_newline: bool,
    level: u32,
    indent_size: u32,
}

impl Default for Text {
    fn default() -> Self {
        Self::new(4)
    }
}

impl Text {
    pub fn new(indent_size: u32) -> Self {
        Self { needs_newline: false, level: 0, indent_size }
    }
}

impl Format for Text {
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        for line in input.split_inclusive('\n') {
            if line.trim().len() > 0 {
                if self.needs_newline {
                    target.write_str("\n")?;
                }
                self.needs_newline = false;
                let space_count =
                    self.level.saturating_sub(1) * self.indent_size;
                for _ in 0 .. space_count {
                    target.write_str(" ")?;
                }
                target.write_str(line)?;
            } else {
                self.needs_newline = line.ends_with('\n');
                if !self.needs_newline {
                    target.write_str(line)?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Nest;

impl Scope for Nest {
    type Format = Text;

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
