use std::fmt;

use super::InlineComponent;
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
    C: Component<Kind = InlineComponent>;

impl<C> Component for Bold<C>
where
    C: Component<Kind = InlineComponent>,
{
    type Kind = InlineComponent;
}

impl<C> Render<HtmlRendering> for Bold<C>
where
    C: Render<HtmlRendering, Kind = InlineComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<HtmlRendering, Self::Kind>,
    ) -> std::fmt::Result {
        write!(
            fmtr,
            "<span class=\"paideia-bold\">{}</span>",
            ctx.render(&self.0)
        )
    }
}

impl<C> Render<MdRendering> for Bold<C>
where
    C: Render<MdRendering, Kind = InlineComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<MdRendering, Self::Kind>,
    ) -> std::fmt::Result {
        write!(fmtr, "**{}**", ctx.render(&self.0))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Italic<C>(pub C)
where
    C: Component<Kind = InlineComponent>;

impl<C> Component for Italic<C>
where
    C: Component<Kind = InlineComponent>,
{
    type Kind = InlineComponent;
}

impl<C> Render<HtmlRendering> for Italic<C>
where
    C: Render<HtmlRendering, Kind = InlineComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<HtmlRendering, Self::Kind>,
    ) -> std::fmt::Result {
        write!(
            fmtr,
            "<span class=\"paideia-italic\">{}</span>",
            ctx.render(&self.0)
        )
    }
}

impl<C> Render<MdRendering> for Italic<C>
where
    C: Render<MdRendering, Kind = InlineComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<MdRendering, Self::Kind>,
    ) -> std::fmt::Result {
        write!(fmtr, "_{}_", ctx.render(&self.0))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Preformatted<C>(pub C)
where
    C: Component<Kind = InlineComponent>;

impl<C> Component for Preformatted<C>
where
    C: Component<Kind = InlineComponent>,
{
    type Kind = InlineComponent;
}

impl<C> Render<HtmlRendering> for Preformatted<C>
where
    C: Render<HtmlRendering, Kind = InlineComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<HtmlRendering, Self::Kind>,
    ) -> std::fmt::Result {
        write!(
            fmtr,
            "<span class=\"paideia-preformatted\">{}</span>",
            ctx.render(&self.0)
        )
    }
}

impl<C> Render<MdRendering> for Preformatted<C>
where
    C: Render<MdRendering, Kind = InlineComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<MdRendering, Self::Kind>,
    ) -> std::fmt::Result {
        write!(fmtr, "<pre>{}</pre>", ctx.render(&self.0))
    }
}
