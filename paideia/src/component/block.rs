use super::{
    Component,
    ComponentKind,
    Context,
    HtmlRendering,
    InlineComponent,
    MdRendering,
    Render,
    TextRendering,
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

impl<C> Render<HtmlRendering> for InlineBlock<C>
where
    C: Render<HtmlRendering, Kind = InlineComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<HtmlRendering, Self::Kind>,
    ) -> fmt::Result {
        write!(
            fmtr,
            "<span class=\"paideia-inline-block\">{}</p>",
            Context::new(
                ctx.location(),
                ctx.level(),
                ctx.render_format(),
                &InlineComponent::new(),
            )
            .render(&self.0)
        )
    }
}

impl<C> Render<MdRendering> for InlineBlock<C>
where
    C: Render<MdRendering, Kind = InlineComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<MdRendering, Self::Kind>,
    ) -> fmt::Result {
        write!(
            fmtr,
            "{}",
            Context::new(
                ctx.location(),
                ctx.level(),
                ctx.render_format(),
                &InlineComponent::new(),
            )
            .render(&self.0)
        )
    }
}

impl<C> Render<TextRendering> for InlineBlock<C>
where
    C: Render<TextRendering, Kind = InlineComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<TextRendering, Self::Kind>,
    ) -> fmt::Result {
        write!(
            fmtr,
            "{}",
            Context::new(
                ctx.location(),
                ctx.level(),
                ctx.render_format(),
                &InlineComponent::new(),
            )
            .render(&self.0)
        )
    }
}
