use std::fmt::{self, Write};

use super::InlineComponent;
use crate::{
    component::{Component, Context, Render, Renderer},
    render::{Html, Markdown, Text},
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
        renderer.write_str("<span class=\"paideia-bold\">")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</span>")?;
        Ok(())
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
        renderer.write_str("**")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("**")?;
        Ok(())
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
        self.0.render(renderer, ctx)
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

impl<C> Render<Html> for Italic<C>
where
    C: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        renderer.write_str("<span class=\"paideia-bold\">")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</span>")?;
        Ok(())
    }
}

impl<'sess, C> Render<Markdown<'sess>> for Italic<C>
where
    C: Render<Markdown<'sess>, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        renderer.write_str("_")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("_")?;
        Ok(())
    }
}

impl<'sess, C> Render<Text<'sess>> for Italic<C>
where
    C: Render<Text<'sess>, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        self.0.render(renderer, ctx)
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

impl<C> Render<Html> for Preformatted<C>
where
    C: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<span class=\"paideia-preformatted\">")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</span>")?;
        Ok(())
    }
}

impl<'sess, C> Render<Markdown<'sess>> for Preformatted<C>
where
    C: Render<Markdown<'sess>, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<pre>")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</pre>")?;
        Ok(())
    }
}

impl<'sess, C> Render<Text<'sess>> for Preformatted<C>
where
    C: Render<Text<'sess>, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.0.render(renderer, ctx)
    }
}
