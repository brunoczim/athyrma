use std::fmt::{self, Write};

use crate::{
    component::{Component, InlineComponent},
    location::Location,
    render::{Context, Html, Markdown, Render, Renderer, Text},
};

use super::BlockComponent;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Image {
    pub location: Location,
    pub alt: String,
}

impl Component for Image {
    type Kind = BlockComponent;
}

impl Render<Html> for Image {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<img class=\"paideia-image\" src=\"")?;
        self.location.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\" alt=\"")?;
        self.location.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\">")?;
        Ok(())
    }
}

impl Render<Markdown> for Image {
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("![")?;
        self.alt.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("](")?;
        self.location.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str(")\n\n")?;
        Ok(())
    }
}

impl Render<Text> for Image {
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("[")?;
        self.alt.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("]")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Figure<L>
where
    L: Component<Kind = InlineComponent>,
{
    pub image: Image,
    pub legend: L,
}

impl<L> Component for Figure<L>
where
    L: Component<Kind = InlineComponent>,
{
    type Kind = BlockComponent;
}

impl<L> Render<Html> for Figure<L>
where
    L: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<div class=\"paideia-figure\">")?;
        self.image.render(renderer, ctx)?;
        renderer.write_str("<div class=\"paideia-figure-legend\">")?;
        self.legend.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("</div></div>")?;
        Ok(())
    }
}

impl<L> Render<Markdown> for Figure<L>
where
    L: Render<Markdown, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("![")?;
        self.image.alt.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("](")?;
        self.image
            .location
            .render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str(")\n")?;
        self.legend.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\n")?;
        Ok(())
    }
}

impl<L> Render<Text> for Figure<L>
where
    L: Render<Text, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.image.render(renderer, ctx)?;
        renderer.write_str("(")?;
        self.legend.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str(")")?;
        Ok(())
    }
}
