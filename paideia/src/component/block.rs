use super::{Component, ComponentKind, InlineComponent};
use crate::render::{Context, Html, Markdown, Render, Renderer, Text};
use std::fmt::{self, Write};

pub mod text;
pub mod list;

#[derive(Debug)]
pub struct BlockComponent {
    _priv: (),
}

impl BlockComponent {
    pub(crate) fn new() -> Self {
        Self { _priv: () }
    }
}

impl ComponentKind for BlockComponent {}

#[derive(Debug, Clone, Copy)]
pub struct InlineBlock<C>(pub C)
where
    C: Component<Kind = InlineComponent>;

impl<C> Component for InlineBlock<C>
where
    C: Component<Kind = InlineComponent>,
{
    type Kind = BlockComponent;
}

impl<C> Render<Html> for InlineBlock<C>
where
    C: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<span class=\"paideia-inline-block\">")?;
        self.0.render(renderer, ctx.with_kind(&InlineComponent::new()))?;
        renderer.write_str("</span>")?;
        Ok(())
    }
}

impl<'sess, C> Render<Markdown> for InlineBlock<C>
where
    C: Render<Markdown, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.0.render(
            renderer,
            Context::new(ctx.location(), ctx.level(), &InlineComponent::new()),
        )
    }
}

impl<'sess, C> Render<Text> for InlineBlock<C>
where
    C: Render<Text, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.0.render(
            renderer,
            Context::new(ctx.location(), ctx.level(), &InlineComponent::new()),
        )
    }
}
