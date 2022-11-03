use super::RenderFormat;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Session {
    needs_newline: bool,
    level: u32,
    indent_size: u32,
}

impl Default for Session {
    fn default() -> Self {
        Self::new(4)
    }
}

impl Session {
    pub fn new(indent_size: u32) -> Self {
        Self { needs_newline: false, level: 0, indent_size }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Markdown<'sess> {
    session: &'sess mut Session,
}

impl<'sess> Markdown<'sess> {
    pub fn new(session: &'sess mut Session) -> Self {
        Self { session }
    }

    pub fn enter(&mut self) -> Markdown {
        self.session.level += 1;
        Markdown::new(&mut *self.ssession)
    }
}

impl<'sess> Drop for Markdown<'sess> {
    fn drop(&mut self) {
        self.session.level = self.session.level.saturating_sub(1);
    }
}

impl<'sess> RenderFormat for Markdown<'sess> {
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
