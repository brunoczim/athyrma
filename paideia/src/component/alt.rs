use crate::location::InternalPath;
use std::fmt;

pub trait RenderFormat {
    fn write_str(
        &self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result;
}

pub trait ComponentKind {}

pub trait Component {
    type Kind: ComponentKind;
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

pub struct Renderer<'render_fmt, 'fmtr, R>
where
    R: RenderFormat + ?Sized,
{
    render_format: &'render_fmt R,
    formatter: &'fmtr mut fmt::Formatter<'fmtr>,
}

pub trait Render<R>: Component
where
    R: RenderFormat + ?Sized,
{
    fn render(
        &self,
        renderer: Renderer<R>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result;
}
