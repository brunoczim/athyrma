#[macro_use]
mod macros;
mod location;

use std::{fmt, rc::Rc, sync::Arc};

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
        ctx: &Self::Context,
        render_format: &R,
    ) -> fmt::Result;
}

pub trait FullRender:
    Render<HtmlRendering> + Render<MdRendering> + Render<TextRendering>
{
}

impl<T> FullRender for T where
    T: Render<HtmlRendering>
        + Render<MdRendering>
        + Render<TextRendering>
        + ?Sized
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
pub struct MdRendering;

impl RenderFormat for MdRendering {}

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

fn md_escape(ch: char) -> Option<&'static str> {
    match ch {
        '*' => Some("\\*"),
        '-' => Some("\\-"),
        '`' => Some("\\`"),
        '_' => Some("\\_"),
        '(' => Some("\\("),
        ')' => Some("\\)"),
        '[' => Some("\\["),
        ']' => Some("\\]"),
        _ => html_escape(ch),
    }
}

impl Component for str {
    type Context = InlineContext;
}

impl Render<HtmlRendering> for str {
    fn render(
        &self,
        fmt: &mut fmt::Formatter,
        _ctx: &Self::Context,
        _render_format: &HtmlRendering,
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

impl Render<MdRendering> for str {
    fn render(
        &self,
        fmt: &mut fmt::Formatter,
        _ctx: &Self::Context,
        _render_format: &MdRendering,
    ) -> fmt::Result {
        let mut start = 0;
        let iter = self
            .char_indices()
            .filter_map(|(i, ch)| md_escape(ch).map(|s| (i, s)));

        for (end, escape) in iter {
            fmt.write_str(&self[start .. end])?;
            fmt.write_str(escape)?;
            start = end + 1;
        }

        fmt.write_str(&self[start ..])?;
        Ok(())
    }
}

impl<'this, T> Component for &'this T
where
    T: Component + ?Sized,
{
    type Context = T::Context;
}

impl<'this, T, R> Render<R> for &'this T
where
    R: RenderFormat + ?Sized,
    T: Render<R> + ?Sized,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Self::Context,
        render_format: &R,
    ) -> fmt::Result {
        (**self).render(fmtr, ctx, render_format)
    }
}

impl<'this, T> Component for &'this mut T
where
    T: Component + ?Sized,
{
    type Context = T::Context;
}

impl<'this, T, R> Render<R> for &'this mut T
where
    R: RenderFormat + ?Sized,
    T: Render<R> + ?Sized,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Self::Context,
        render_format: &R,
    ) -> fmt::Result {
        (**self).render(fmtr, ctx, render_format)
    }
}

impl<T> Component for Box<T>
where
    T: Component + ?Sized,
{
    type Context = T::Context;
}

impl<T, R> Render<R> for Box<T>
where
    R: RenderFormat + ?Sized,
    T: Render<R> + ?Sized,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Self::Context,
        render_format: &R,
    ) -> fmt::Result {
        (**self).render(fmtr, ctx, render_format)
    }
}

impl<T> Component for Rc<T>
where
    T: Component + ?Sized,
{
    type Context = T::Context;
}

impl<T, R> Render<R> for Rc<T>
where
    R: RenderFormat + ?Sized,
    T: Render<R> + ?Sized,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Self::Context,
        render_format: &R,
    ) -> fmt::Result {
        (**self).render(fmtr, ctx, render_format)
    }
}

impl<T> Component for Arc<T>
where
    T: Component + ?Sized,
{
    type Context = T::Context;
}

impl<T, R> Render<R> for Arc<T>
where
    R: RenderFormat + ?Sized,
    T: Render<R> + ?Sized,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Self::Context,
        render_format: &R,
    ) -> fmt::Result {
        (**self).render(fmtr, ctx, render_format)
    }
}
