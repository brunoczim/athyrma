mod inline;
mod block;

use crate::location::InternalPath;
use std::{fmt, rc::Rc, sync::Arc};

pub use block::BlockComponent;
pub use inline::InlineComponent;
use katalogos::{
    colist::{Cocons, Conil},
    list::{Cons, Nil},
};

#[derive(Debug, Clone, Copy)]
pub struct Context<'loc, 'fmt, 'kind, R, K>
where
    R: RenderFormat + ?Sized,
    K: ComponentKind + ?Sized,
{
    location: &'loc InternalPath,
    level: u32,
    render_format: &'fmt R,
    kind: &'kind K,
}

impl<'loc, 'fmt, 'kind, R, K> Context<'loc, 'fmt, 'kind, R, K>
where
    R: RenderFormat + ?Sized,
    K: ComponentKind + ?Sized,
{
    pub fn new(
        location: &'loc InternalPath,
        level: u32,
        render_format: &'fmt R,
        kind: &'kind K,
    ) -> Self {
        Self { location, level, render_format, kind }
    }

    pub fn location(&self) -> &'loc InternalPath {
        self.location
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn render_format(&self) -> &'fmt R {
        self.render_format
    }

    pub fn kind(&self) -> &'kind K {
        self.kind
    }

    pub fn render<'this, T>(
        &'this self,
        component: T,
    ) -> Renderer<'this, 'loc, 'fmt, 'kind, T, R>
    where
        T: Render<R, Kind = K>,
    {
        Renderer { context: self, component }
    }
}

pub trait RenderFormat {}

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
        fmtr: &mut fmt::Formatter,
        ctx: &Context<R, Self::Kind>,
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

/// A renderer over a component. The `Display` trait can be used on the
/// renderer.
#[derive(Debug, Clone, Copy)]
pub struct Renderer<'ctx, 'loc, 'fmt, 'kind, T, R>
where
    R: RenderFormat + ?Sized,
    T: Render<R>,
{
    /// The component being rendered.
    pub component: T,
    /// The context at which the component will be rendered.
    pub context: &'ctx Context<'loc, 'fmt, 'kind, R, T::Kind>,
}

impl<'ctx, 'loc, 'fmt, 'kind, T, R> fmt::Display
    for Renderer<'ctx, 'loc, 'fmt, 'kind, T, R>
where
    R: RenderFormat,
    T: Render<R>,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.component.render(fmt, self.context)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HtmlRendering;

impl RenderFormat for HtmlRendering {}

#[derive(Debug, Clone, Copy)]
pub struct MdRendering;

impl RenderFormat for MdRendering {}

#[derive(Debug, Clone, Copy)]
pub struct TextRendering;

impl RenderFormat for TextRendering {}

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
        _fmtr: &mut fmt::Formatter,
        _ctx: &Context<R, Self::Kind>,
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
        fmtr: &mut fmt::Formatter,
        ctx: &Context<R, Self::Kind>,
    ) -> fmt::Result {
        match self {
            Cocons::Head(head) => head.render(fmtr, ctx),
            Cocons::Tail(tail) => tail.render(fmtr, ctx),
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
        fmtr: &mut fmt::Formatter,
        ctx: &Context<R, Self::Kind>,
    ) -> fmt::Result {
        (**self).render(fmtr, ctx)
    }
}

impl<'this, T, R> Render<R> for &'this mut T
where
    R: RenderFormat,
    T: Render<R> + ?Sized,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<R, Self::Kind>,
    ) -> fmt::Result {
        (**self).render(fmtr, ctx)
    }
}

impl<T, R> Render<R> for Box<T>
where
    R: RenderFormat,
    T: Render<R> + ?Sized,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<R, Self::Kind>,
    ) -> fmt::Result {
        (**self).render(fmtr, ctx)
    }
}

impl<T, R> Render<R> for Rc<T>
where
    R: RenderFormat,
    T: Render<R> + ?Sized,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<R, Self::Kind>,
    ) -> fmt::Result {
        (**self).render(fmtr, ctx)
    }
}

impl<T, R> Render<R> for Arc<T>
where
    R: RenderFormat,
    T: Render<R> + ?Sized,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<R, Self::Kind>,
    ) -> fmt::Result {
        (**self).render(fmtr, ctx)
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
