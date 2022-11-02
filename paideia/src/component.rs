mod inline;
mod block;
mod alt;

use crate::{location::InternalPath, render_format::RenderFormat};
use std::{fmt, rc::Rc, sync::Arc};

pub use block::BlockComponent;
pub use inline::InlineComponent;
use katalogos::{
    colist::{Cocons, Conil},
    list::{Cons, Nil},
};

#[derive(Debug)]
pub struct Renderer<'render_fmt, 'fmtr, R>
where
    R: RenderFormat + ?Sized,
{
    render_format: &'render_fmt mut R,
    formatter: &'fmtr mut fmt::Formatter<'fmtr>,
}

impl<'render_fmt, 'fmtr, R> Renderer<'render_fmt, 'fmtr, R>
where
    R: RenderFormat + ?Sized,
{
    pub fn format(&self) -> &R {
        &self.render_format
    }

    pub fn format_mut(&mut self) -> &mut R {
        &mut self.render_format
    }

    pub fn with<'fmt_s, 'fmtr_this, S>(
        &'fmtr_this mut self,
        render_format: &'fmt_s mut S,
    ) -> Renderer<'fmt_s, 'fmtr_this, S>
    where
        S: RenderFormat + ?Sized,
    {
        Renderer { render_format, formatter: &mut *self.formatter }
    }
}

impl<'render_fmt, 'fmtr, R> fmt::Write for Renderer<'render_fmt, 'fmtr, R>
where
    R: RenderFormat + ?Sized,
{
    fn write_str(&mut self, input: &str) -> fmt::Result {
        self.render_format.write_str(input, self.formatter)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Context<'loc, 'kind, K>
where
    K: ComponentKind + ?Sized,
{
    location: &'loc InternalPath,
    level: u32,
    kind: &'kind K,
}

impl<'loc, 'kind, K> Context<'loc, 'kind, K>
where
    K: ComponentKind + ?Sized,
{
    pub fn new(
        location: &'loc InternalPath,
        level: u32,
        kind: &'kind K,
    ) -> Self {
        Self { location, level, kind }
    }

    pub fn location(&self) -> &'loc InternalPath {
        self.location
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn kind(&self) -> &'kind K {
        self.kind
    }
}

pub trait ComponentKind {}

pub trait Component {
    type Kind: ComponentKind + ?Sized;
}

pub trait Render<R>: Component
where
    R: RenderFormat + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<R>,
        ctx: Context<Self::Kind>,
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
pub struct HtmlRendering;

impl RenderFormat for HtmlRendering {
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        target.write_str(input)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MdRendering {
    level: u32,
    needs_newline: bool,
}

impl MdRendering {
    pub fn new() -> Self {
        Self { level: 0, needs_newline: false }
    }
}

impl RenderFormat for MdRendering {
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        target.write_str(input)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TextRendering;

impl RenderFormat for TextRendering {
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        target.write_str(input)
    }
}

impl<'this, T> Component for &'this T
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;
}

impl<'this, T> Component for &'this mut T
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;
}

impl<T> Component for Box<T>
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;
}

impl<T> Component for Rc<T>
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;
}

impl<T> Component for Arc<T>
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;
}

impl<C> Component for Nil<C>
where
    C: ComponentKind,
{
    type Kind = C;
}

impl<H, T> Component for Cons<H, T>
where
    H: Component,
    T: Component<Kind = H::Kind>,
{
    type Kind = H::Kind;
}

impl<C> Component for Conil<C>
where
    C: ComponentKind,
{
    type Kind = C;
}

impl<H, T> Component for Cocons<H, T>
where
    H: Component,
    T: Component<Kind = H::Kind>,
{
    type Kind = H::Kind;
}

impl<C, R> Render<R> for Conil<C>
where
    C: ComponentKind,
    R: RenderFormat,
{
    fn render(
        &self,
        _renderer: &mut Renderer<R>,
        _ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.coerce()
    }
}

impl<R, H, T> Render<R> for Cocons<H, T>
where
    R: RenderFormat,
    H: Render<R>,
    T: Render<R, Kind = H::Kind>,
{
    fn render(
        &self,
        renderer: &mut Renderer<R>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        match self {
            Cocons::Head(head) => head.render(renderer, ctx),
            Cocons::Tail(tail) => tail.render(renderer, ctx),
        }
    }
}

impl<'this, T, R> Render<R> for &'this T
where
    R: RenderFormat,
    T: Render<R> + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<R>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl<'this, T, R> Render<R> for &'this mut T
where
    R: RenderFormat,
    T: Render<R> + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<R>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl<T, R> Render<R> for Box<T>
where
    R: RenderFormat,
    T: Render<R> + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<R>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl<T, R> Render<R> for Rc<T>
where
    R: RenderFormat,
    T: Render<R> + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<R>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl<T, R> Render<R> for Arc<T>
where
    R: RenderFormat,
    T: Render<R> + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<R>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl<'this, R> RenderFormat for &'this R where R: RenderFormat + ?Sized {}

impl<'this, R> RenderFormat for &'this mut R where R: RenderFormat + ?Sized {}

impl<R> RenderFormat for Box<R> where R: RenderFormat + ?Sized {}

impl<R> RenderFormat for Rc<R> where R: RenderFormat + ?Sized {}

impl<R> RenderFormat for Arc<R> where R: RenderFormat + ?Sized {}

impl<'this, K> ComponentKind for &'this K where K: ComponentKind + ?Sized {}

impl<'this, K> ComponentKind for &'this mut K where K: ComponentKind + ?Sized {}

impl<K> ComponentKind for Box<K> where K: ComponentKind + ?Sized {}

impl<K> ComponentKind for Rc<K> where K: ComponentKind + ?Sized {}

impl<K> ComponentKind for Arc<K> where K: ComponentKind + ?Sized {}
