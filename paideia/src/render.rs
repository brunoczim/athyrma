use std::{fmt, rc::Rc, sync::Arc};

pub mod html;
pub mod markdown;
pub mod text;

pub use html::Html;
use katalogos::{
    colist::{Cocons, Conil},
    list::{Cons, Nil},
};
pub use markdown::Markdown;
pub use text::Text;

use crate::{
    component::{Component, ComponentKind},
    location::InternalPath,
};

pub trait Format {
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result;
}

impl<'this, W> Format for &'this mut W
where
    W: Format + ?Sized,
{
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        (**self).write_str(input, target)
    }
}

impl<W> Format for Box<W>
where
    W: Format + ?Sized,
{
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        (**self).write_str(input, target)
    }
}

pub trait Scope {
    type Format: Format + ?Sized;

    fn enter<F, T>(&self, format: &mut Self::Format, consumer: F) -> T
    where
        F: FnOnce(&mut Self::Format) -> T;
}

impl<'this, S> Scope for &'this S
where
    S: Scope + ?Sized,
{
    type Format = S::Format;

    fn enter<F, T>(&self, format: &mut Self::Format, consumer: F) -> T
    where
        F: FnOnce(&mut Self::Format) -> T,
    {
        (**self).enter(format, consumer)
    }
}

impl<'this, S> Scope for &'this mut S
where
    S: Scope + ?Sized,
{
    type Format = S::Format;

    fn enter<F, T>(&self, format: &mut Self::Format, consumer: F) -> T
    where
        F: FnOnce(&mut Self::Format) -> T,
    {
        (**self).enter(format, consumer)
    }
}

impl<S> Scope for Box<S>
where
    S: Scope + ?Sized,
{
    type Format = S::Format;

    fn enter<F, T>(&self, format: &mut Self::Format, consumer: F) -> T
    where
        F: FnOnce(&mut Self::Format) -> T,
    {
        (**self).enter(format, consumer)
    }
}

impl<S> Scope for Rc<S>
where
    S: Scope + ?Sized,
{
    type Format = S::Format;

    fn enter<F, T>(&self, format: &mut Self::Format, consumer: F) -> T
    where
        F: FnOnce(&mut Self::Format) -> T,
    {
        (**self).enter(format, consumer)
    }
}

impl<S> Scope for Arc<S>
where
    S: Scope + ?Sized,
{
    type Format = S::Format;

    fn enter<F, T>(&self, format: &mut Self::Format, consumer: F) -> T
    where
        F: FnOnce(&mut Self::Format) -> T,
    {
        (**self).enter(format, consumer)
    }
}

pub trait Render<W>: Component
where
    W: Format + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result;
}

impl<C, W> Render<W> for Nil<C>
where
    C: ComponentKind,
    W: Format,
{
    fn render(
        &self,
        _renderer: &mut Renderer<W>,
        _ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        Ok(())
    }
}

impl<W, H, T> Render<W> for Cons<H, T>
where
    W: Format,
    H: Render<W>,
    T: Render<W, Kind = H::Kind>,
{
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.head.render(renderer, ctx)?;
        self.tail.render(renderer, ctx)?;
        Ok(())
    }
}

impl<C, W> Render<W> for Conil<C>
where
    C: ComponentKind,
    W: Format,
{
    fn render(
        &self,
        _renderer: &mut Renderer<W>,
        _ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.coerce()
    }
}

impl<W, H, T> Render<W> for Cocons<H, T>
where
    W: Format,
    H: Render<W>,
    T: Render<W, Kind = H::Kind>,
{
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        match self {
            Cocons::Head(head) => head.render(renderer, ctx),
            Cocons::Tail(tail) => tail.render(renderer, ctx),
        }
    }
}

impl<'this, T, W> Render<W> for &'this T
where
    W: Format,
    T: Render<W> + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl<'this, T, W> Render<W> for &'this mut T
where
    W: Format,
    T: Render<W> + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl<T, W> Render<W> for Box<T>
where
    W: Format,
    T: Render<W> + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl<T, W> Render<W> for Rc<T>
where
    W: Format,
    T: Render<W> + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl<T, W> Render<W> for Arc<T>
where
    W: Format,
    T: Render<W> + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

pub trait FullRender: Render<Html> + Render<Markdown> + Render<Text> {}

impl<T> FullRender for T where
    T: Render<Html> + Render<Markdown> + Render<Text> + ?Sized
{
}

pub struct Renderer<'format, 'target, 'obj, W>
where
    W: Format + ?Sized,
{
    render_format: &'format mut W,
    target: &'target mut (dyn fmt::Write + Send + Sync + 'obj),
}

impl<'format, 'target, 'obj, W> fmt::Debug
    for Renderer<'format, 'target, 'obj, W>
where
    W: Format + ?Sized + fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_struct("Renderer")
            .field("render_format", &self.render_format)
            .field("formatter", &(self.target as *const _))
            .finish()
    }
}

impl<'format, 'target, 'obj, W> Renderer<'format, 'target, 'obj, W>
where
    W: Format,
{
    pub fn scoped<S, F, T>(&mut self, scope: S, consumer: F) -> T
    where
        F: FnOnce(&mut Renderer<W>) -> T,
        S: Scope<Format = W>,
    {
        let render_format = &mut *self.render_format;
        let formatter = &mut *self.target;

        scope.enter(render_format, |render_format| {
            consumer(&mut Renderer { render_format, target: formatter })
        })
    }
}

impl<'format, 'target, 'obj, W> fmt::Write
    for Renderer<'format, 'target, 'obj, W>
where
    W: Format + ?Sized,
{
    fn write_str(&mut self, input: &str) -> fmt::Result {
        self.render_format.write_str(input, self.target)
    }
}

#[derive(Debug)]
pub struct Context<'loc, 'kind, K>
where
    K: ComponentKind + ?Sized,
{
    location: &'loc InternalPath,
    level: u32,
    kind: &'kind K,
}

impl<'loc, 'kind, K> Clone for Context<'loc, 'kind, K>
where
    K: ComponentKind + ?Sized,
{
    fn clone(&self) -> Self {
        Self { location: self.location, level: self.level, kind: self.kind }
    }
}

impl<'loc, 'kind, K> Copy for Context<'loc, 'kind, K> where
    K: ComponentKind + ?Sized
{
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

    pub fn with_kind<Q>(self, kind: &'kind Q) -> Context<'loc, 'kind, Q>
    where
        Q: ComponentKind + ?Sized,
    {
        Context::new(self.location(), self.level(), kind)
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
