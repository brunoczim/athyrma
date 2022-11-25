use core::fmt;

use crate::{
    component::{block::BlockComponent, Component, ComponentKind},
    render::{Context, Format, Html, Markdown, Render, Renderer, Text},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EntryComponent;

impl ComponentKind for EntryComponent {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntryAttrs {
    pub header: bool,
    pub rows: u32,
    pub columns: u32,
}

impl Default for EntryAttrs {
    fn default() -> Self {
        Self { header: false, rows: 1, columns: 1 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Entry<T>
where
    T: Component<Kind = BlockComponent>,
{
    pub child: T,
    pub attrs: EntryAttrs,
}

impl<T> Entry<T>
where
    T: Component<Kind = BlockComponent>,
{
    fn render_generic_html<R>(
        &self,
        renderer: &mut Renderer<R>,
        ctx: Context<<Self as Component>::Kind>,
    ) -> fmt::Result
    where
        R: Format + ?Sized,
        T: Render<R>,
    {
        todo!()
    }
}

impl<T> Component for Entry<T>
where
    T: Component<Kind = BlockComponent>,
{
    type Kind = EntryComponent;
}

impl<T> Render<Html> for Entry<T>
where
    T: Render<Html, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        self.render_generic_html(renderer, ctx)
    }
}

impl<T> Render<Markdown> for Entry<T>
where
    T: Render<Markdown, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        self.render_generic_html(renderer, ctx)
    }
}

impl<T> Render<Text> for Entry<T>
where
    T: Render<Text, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        self.child.render(renderer, ctx.with_kind(&BlockComponent))
    }
}
