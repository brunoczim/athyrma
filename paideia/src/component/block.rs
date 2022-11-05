use crate::render::{Html, Markdown, Text};

use super::{
    Component,
    ComponentKind,
    Context,
    InlineComponent,
    Render,
    Renderer,
};
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

impl<'sess, C> Render<Markdown<'sess>> for InlineBlock<C>
where
    C: Render<Markdown<'sess>, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.0.render(
            renderer,
            Context::new(ctx.location(), ctx.level(), &InlineComponent::new()),
        )
    }
}

impl<'sess, C> Render<Text<'sess>> for InlineBlock<C>
where
    C: Render<Text<'sess>, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.0.render(
            renderer,
            Context::new(ctx.location(), ctx.level(), &InlineComponent::new()),
        )
    }
}
