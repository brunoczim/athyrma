use std::{
    fmt,
    rc::Rc,
    sync::{Arc, Mutex},
};

pub mod html;
pub mod markdown;
pub mod text;

mod common_text;

pub use html::Html;
pub use markdown::Markdown;
pub use text::Text;

use katalogos::coproduct::{Cocons, Conil};

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

impl<C, W> Render<W> for Conil<C>
where
    C: ComponentKind,
    W: Format + ?Sized,
{
    fn render(
        &self,
        _renderer: &mut Renderer<W>,
        _ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        Ok(())
    }
}

impl<W, H, T> Render<W> for Cocons<H, T>
where
    W: Format + ?Sized,
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
    W: Format + ?Sized,
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
    W: Format + ?Sized,
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
    W: Format + ?Sized,
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
    W: Format + ?Sized,
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
    W: Format + ?Sized,
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

impl<T, R, const N: usize> Render<R> for [T; N]
where
    T: Render<R>,
    R: Format + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<R>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        for element in self {
            element.render(renderer, ctx)?;
        }
        Ok(())
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
    format: &'format mut W,
    target: &'target mut (dyn fmt::Write + 'obj),
}

impl<'format, 'target, 'obj, W> fmt::Debug
    for Renderer<'format, 'target, 'obj, W>
where
    W: Format + ?Sized + fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_struct("Renderer")
            .field("render_format", &self.format)
            .field("formatter", &(self.target as *const _))
            .finish()
    }
}

impl<'format, 'target, 'obj, W> Renderer<'format, 'target, 'obj, W>
where
    W: Format + ?Sized,
{
    pub fn new(
        format: &'format mut W,
        target: &'target mut (dyn fmt::Write + 'obj),
    ) -> Self {
        Self { format, target }
    }

    pub fn scoped<S, F, T>(&mut self, scope: S, consumer: F) -> T
    where
        F: FnOnce(&mut Renderer<W>) -> T,
        S: Scope<Format = W>,
    {
        let render_format = &mut *self.format;
        let formatter = &mut *self.target;

        scope.enter(render_format, |render_format| {
            consumer(&mut Renderer { format: render_format, target: formatter })
        })
    }
}

impl<'format, 'target, 'obj, W> fmt::Write
    for Renderer<'format, 'target, 'obj, W>
where
    W: Format + ?Sized,
{
    fn write_str(&mut self, input: &str) -> fmt::Result {
        self.format.write_str(input, self.target)
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
    pub fn new(location: &'loc InternalPath, kind: &'kind K) -> Self {
        Self { location, level: 0, kind }
    }

    pub fn with_kind<Q>(self, kind: &'kind Q) -> Context<'loc, 'kind, Q>
    where
        Q: ComponentKind + ?Sized,
    {
        Context { location: self.location, level: self.level, kind }
    }

    pub fn location(self) -> &'loc InternalPath {
        self.location
    }

    pub fn level(self) -> u32 {
        self.level
    }

    pub fn enter(self) -> Self {
        Self { level: self.level + 1, ..self }
    }

    pub fn kind(self) -> &'kind K {
        self.kind
    }
}

#[derive(Debug)]
pub struct RenderAsDisplay<'loc, 'kind, 'format, C, W>
where
    C: Render<W>,
    W: Format + ?Sized,
{
    component: C,
    format: Mutex<&'format mut W>,
    context: Context<'loc, 'kind, C::Kind>,
}

impl<'loc, 'kind, 'format, C, W> RenderAsDisplay<'loc, 'kind, 'format, C, W>
where
    C: Render<W>,
    W: Format + ?Sized,
{
    pub fn new(
        component: C,
        format: &'format mut W,
        context: Context<'loc, 'kind, C::Kind>,
    ) -> Self {
        Self { component, format: Mutex::new(format), context }
    }
}

impl<'loc, 'kind, 'format, C, W> fmt::Display
    for RenderAsDisplay<'loc, 'kind, 'format, C, W>
where
    C: Render<W>,
    W: Format + ?Sized,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut format = self.format.lock().unwrap();
        self.component
            .render(&mut Renderer::new(&mut **format, fmtr), self.context)?;
        Ok(())
    }
}
