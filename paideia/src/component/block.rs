use crate::render_format::{Html, Markdown, Text};

use super::{
    Component,
    ComponentKind,
    Context,
    InlineComponent,
    Render,
    Renderer,
};
use std::fmt;

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
        write!(
            renderer,
            "<span class=\"paideia-inline-block\">{}</p>",
            Context::new(ctx.location(), ctx.level(), &InlineComponent::new())
                .render(&self.0)
        )
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
        write!(
            renderer,
            "{}",
            Context::new(ctx.location(), ctx.level(), &InlineComponent::new(),)
                .render(&self.0)
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
        write!(
            renderer,
            "{}",
            Context::new(ctx.location(), ctx.level(), &InlineComponent::new(),)
                .render(&self.0)
        )
    }
}
