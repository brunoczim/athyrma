use crate::location::InternalPath;
use std::{fmt, rc::Rc, sync::Arc};

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
    pub(crate) fn new(
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

    pub fn renderer<'this, T>(
        &'this self,
        component: T,
    ) -> Renderer<'this, 'loc, 'fmt, 'kind, T, R>
    where
        T: Component<Kind = K>,
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
    R: RenderFormat,
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

#[derive(Debug)]
pub struct InlineComponent {
    _priv: (),
}

#[derive(Debug)]
pub struct BlockComponent {
    _priv: (),
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
    type Kind = InlineComponent;
}

impl Render<HtmlRendering> for str {
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<HtmlRendering, Self::Kind>,
    ) -> fmt::Result {
        let mut start = 0;
        let iter = self
            .char_indices()
            .filter_map(|(i, ch)| html_escape(ch).map(|s| (i, s)));

        for (end, escape) in iter {
            fmtr.write_str(&self[start .. end])?;
            fmtr.write_str(escape)?;
            start = end + 1;
        }

        fmtr.write_str(&self[start ..])?;
        Ok(())
    }
}

impl Render<MdRendering> for str {
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<MdRendering, Self::Kind>,
    ) -> fmt::Result {
        let mut start = 0;
        let iter = self
            .char_indices()
            .filter_map(|(i, ch)| md_escape(ch).map(|s| (i, s)));

        for (end, escape) in iter {
            fmtr.write_str(&self[start .. end])?;
            fmtr.write_str(escape)?;
            start = end + 1;
        }

        fmtr.write_str(&self[start ..])?;
        Ok(())
    }
}

impl Render<TextRendering> for str {
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        _ctx: &Context<TextRendering, Self::Kind>,
    ) -> fmt::Result {
        write!(fmtr, "{}", self)
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
