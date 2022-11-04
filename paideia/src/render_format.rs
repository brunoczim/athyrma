use std::fmt;

mod html;
mod markdown;
mod text;

pub use html::Html;
pub use markdown::Markdown;
pub use text::Text;

pub trait RenderFormat {
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result;
}

impl<'this, T> RenderFormat for &'this mut T
where
    T: RenderFormat + ?Sized,
{
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        (**self).write_str(input, target)
    }
}

impl<T> RenderFormat for Box<T>
where
    T: RenderFormat + ?Sized,
{
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        (**self).write_str(input, target)
    }
}
