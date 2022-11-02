use std::fmt;

use super::InlineComponent;
use crate::{
    component::{Component, Context, Render, Renderer},
    render_format::{Html, Markdown, Text},
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

impl<C> Render<Html> for Bold<C>
where
    C: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        write!(
            renderer,
            "<span class=\"paideia-bold\">{}</span>",
            ctx.render(&self.0)
        )
    }
}

impl<'sess, C> Render<Markdown<'sess>> for Bold<C>
where
    C: Render<Markdown<'sess>, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        write!(renderer, "**{}**", ctx.render(&self.0))
    }
}

impl<'sess, C> Render<Text<'sess>> for Bold<C>
where
    C: Render<Text<'sess>, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        write!(renderer, "{}", ctx.render(&self.0))
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
