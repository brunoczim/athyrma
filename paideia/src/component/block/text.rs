use std::fmt;

use super::BlockComponent;
use crate::{
    component::{Component, Context, InlineComponent, Render, Renderer},
    render_format::{Html, Markdown, Text},
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

impl<C> Render<Html> for Bold<C>
where
    C: Render<Html, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        write!(
            renderer,
            "<div class=\"paideia-bold\">{}</div>",
            ctx.render(&self.0)
        )
    }
}

impl<'sess, C> Render<Markdown<'sess>> for Bold<C>
where
    C: Render<Markdown<'sess>, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        write!(renderer, "<b>{}</b>", ctx.render(&self.0))
    }
}

impl<'sess, C> Render<Text<'sess>> for Bold<C>
where
    C: Render<Markdown<'sess>, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        write!(renderer, "{}", ctx.render(&self.0))
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

impl<C> Render<Html> for Italic<C>
where
    C: Render<Html, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        write!(
            renderer,
            "<div class=\"paideia-italic\">{}</div>",
            ctx.render(&self.0)
        )
    }
}

impl<'sess, C> Render<Markdown<'sess>> for Italic<C>
where
    C: Render<Markdown<'sess>, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        write!(renderer, "<i>{}</i>", ctx.render(&self.0))
    }
}

impl<'sess, C> Render<Text<'sess>> for Italic<C>
where
    C: Render<Markdown<'sess>, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        write!(renderer, "{}", ctx.render(&self.0))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Preformatted<C>(pub C)
where
    C: Component<Kind = BlockComponent>;

impl<C> Render<Html> for Preformatted<C>
where
    C: Render<Html, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        write!(
            renderer,
            "<div class=\"paideia-preformatted\">{}</div>",
            ctx.render(&self.0)
        )
    }
}

impl<'sess, C> Render<Markdown<'sess>> for Preformatted<C>
where
    C: Render<Markdown<'sess>, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        write!(renderer, "<pre>{}</pre>", ctx.render(&self.0))
    }
}

impl<'sess, C> Render<Text<'sess>> for Preformatted<C>
where
    C: Render<Markdown<'sess>, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        write!(renderer, "{}", ctx.render(&self.0))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Paragraph<C>(pub C)
where
    C: Component<Kind = InlineComponent>;

impl<C> Component for Paragraph<C>
where
    C: Component<Kind = InlineComponent>,
{
    type Kind = BlockComponent;
}

impl<C> Render<Html> for Paragraph<C>
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
            "<p class=\"paideia-paragraph\">{}</p>",
            Context::new(ctx.location(), ctx.level(), &InlineComponent::new(),)
                .render(&self.0)
        )
    }
}

impl<'sess, C> Render<Markdown<'sess>> for Paragraph<C>
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
            "{}\n\n",
            Context::new(ctx.location(), ctx.level(), &InlineComponent::new(),)
                .render(&self.0)
        )
    }
}

impl<'sess, C> Render<Text<'sess>> for Paragraph<C>
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
            "{}\n\n",
            Context::new(ctx.location(), ctx.level(), &InlineComponent::new(),)
                .render(&self.0)
        )
    }
}
