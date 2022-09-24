use std::fmt;

use super::BlockComponent;
use crate::component::{
    Component,
    Context,
    HtmlRendering,
    MdRendering,
    Render,
};

#[derive(Debug, Clone, Copy)]
pub struct Bold<C>(pub C)
where
    C: Component<Kind = BlockComponent>;

impl<C> Component for Bold<C>
where
    C: Component<Kind = BlockComponent>,
{
    type Kind = BlockComponent;
}

impl<C> Render<HtmlRendering> for Bold<C>
where
    C: Render<HtmlRendering, Kind = BlockComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<HtmlRendering, Self::Kind>,
    ) -> std::fmt::Result {
        write!(
            fmtr,
            "<div class=\"paideia-bold\">{}</div>",
            ctx.renderer(&self.0)
        )
    }
}

impl<C> Render<MdRendering> for Bold<C>
where
    C: Render<MdRendering, Kind = BlockComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<MdRendering, Self::Kind>,
    ) -> std::fmt::Result {
        write!(fmtr, "<b>{}</b>", ctx.renderer(&self.0))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Italic<C>(pub C)
where
    C: Component<Kind = BlockComponent>;

impl<C> Component for Italic<C>
where
    C: Component<Kind = BlockComponent>,
{
    type Kind = BlockComponent;
}

impl<C> Render<HtmlRendering> for Italic<C>
where
    C: Render<HtmlRendering, Kind = BlockComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<HtmlRendering, Self::Kind>,
    ) -> std::fmt::Result {
        write!(
            fmtr,
            "<div class=\"paideia-italic\">{}</div>",
            ctx.renderer(&self.0)
        )
    }
}

impl<C> Render<MdRendering> for Italic<C>
where
    C: Render<MdRendering, Kind = BlockComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<MdRendering, Self::Kind>,
    ) -> std::fmt::Result {
        write!(fmtr, "<i>{}</i>", ctx.renderer(&self.0))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Preformatted<C>(pub C)
where
    C: Component<Kind = BlockComponent>;

impl<C> Component for Preformatted<C>
where
    C: Component<Kind = BlockComponent>,
{
    type Kind = BlockComponent;
}

impl<C> Render<HtmlRendering> for Preformatted<C>
where
    C: Render<HtmlRendering, Kind = BlockComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<HtmlRendering, Self::Kind>,
    ) -> std::fmt::Result {
        write!(
            fmtr,
            "<div class=\"paideia-preformatted\">{}</div>",
            ctx.renderer(&self.0)
        )
    }
}

impl<C> Render<MdRendering> for Preformatted<C>
where
    C: Render<MdRendering, Kind = BlockComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<MdRendering, Self::Kind>,
    ) -> std::fmt::Result {
        write!(fmtr, "<pre>{}</pre>", ctx.renderer(&self.0))
    }
}
