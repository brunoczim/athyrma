//mod location;

use std::fmt;

pub trait Context {}

pub trait RenderFormat {}

pub trait Component {
    type Context: Context + ?Sized;
}

pub trait Render<R>: Component
where
    R: RenderFormat + ?Sized,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        render_format: &R,
        ctx: &Self::Context,
    ) -> fmt::Result;
}

pub trait FullRender: Render<HtmlRendering> + Render<TextRendering> {}

impl<T> FullRender for T where
    T: Render<HtmlRendering> + Render<TextRendering> + ?Sized
{
}

#[derive(Debug, Clone, Copy)]
pub struct InlineContext;

impl Context for InlineContext {}

#[derive(Debug, Clone, Copy)]
pub struct BlockContext;

impl Context for BlockContext {}

#[derive(Debug, Clone, Copy)]
pub struct HtmlRendering;

impl RenderFormat for HtmlRendering {}

#[derive(Debug, Clone, Copy)]
pub struct TextRendering;

impl RenderFormat for TextRendering {}

fn html_escape(ch: char) -> Option<&'static str> {
    match ch {
        '&' => Some("&amp;"),
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '"' => Some("&quot;"),
        '\'' => Some("&#39;"),
        '\\' => Some("&#92;"),
        _ => None,
    }
}

impl Component for str {
    type Context = InlineContext;
}

impl Render<HtmlRendering> for str {
    fn render(
        &self,
        fmt: &mut fmt::Formatter,
        _render_format: &HtmlRendering,
        _ctx: &Self::Context,
    ) -> fmt::Result {
        let mut start = 0;
        let iter = self
            .char_indices()
            .filter_map(|(i, ch)| html_escape(ch).map(|s| (i, s)));

        for (end, escape) in iter {
            fmt.write_str(&self[start .. end])?;
            fmt.write_str(escape)?;
            start = end + 1;
        }

        fmt.write_str(&self[start ..])?;
        Ok(())
    }
}

impl Render<TextRendering> for str {
    fn render(
        &self,
        fmt: &mut fmt::Formatter,
        _render_format: &TextRendering,
        _ctx: &Self::Context,
    ) -> fmt::Result {
        fmt.write_str(self)
    }
}
