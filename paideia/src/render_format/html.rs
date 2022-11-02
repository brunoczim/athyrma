use super::RenderFormat;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Html;

impl RenderFormat for Html {
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        target.write_str(input)
    }
}
