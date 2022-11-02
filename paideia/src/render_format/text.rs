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
pub struct Text<'sess> {
    session: &'sess mut Session,
}

impl<'sess> Text<'sess> {
    pub fn new(session: &'sess mut Session) -> Self {
        Self { session }
    }

    pub fn enter(&mut self) -> Text {
        self.session.level += 1;
        Text::new(&mut *self.session)
    }
}

impl<'sess> Drop for Text<'sess> {
    fn drop(&mut self) {
        self.session.level = self.session.level.saturating_sub(1);
    }
}

impl<'sess> RenderFormat for Text<'sess> {
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        for line in input.split_inclusive('\n') {
            if line.trim().len() > 0 {
                if self.session.needs_newline {
                    target.write_str("\n")?;
                }
                self.session.needs_newline = false;
                for _ in 0 .. self.session.level * self.session.indent_size {
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
