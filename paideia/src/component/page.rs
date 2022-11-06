use super::{
    section::SectionComponent,
    BlockComponent,
    Component,
    ComponentKind,
    InlineComponent,
};
use crate::{
    location::{Id, InternalLoc, Location},
    render::{Context, Html, Markdown, Render, Renderer, Text},
};
use std::fmt::{self, Write};

#[derive(Debug)]
pub struct PageComponent {
    _priv: (),
}

impl PageComponent {
    pub(crate) fn new() -> Self {
        Self { _priv: () }
    }
}

impl ComponentKind for PageComponent {}

pub struct Page<T, B, L>
where
    T: Component<Kind = InlineComponent>,
    B: Component<Kind = BlockComponent>,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = SectionComponent>,
{
    pub title: T,
    pub body: B,
    pub children: L,
}

impl<T, B, L> fmt::Debug for Page<T, B, L>
where
    T: Component<Kind = InlineComponent>,
    B: Component<Kind = BlockComponent>,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = SectionComponent>,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_fmtr = fmtr.debug_struct("UnorderedList");
        debug_fmtr.field("title", &self.title).field("body", &self.body);
        for (i, element) in self.children.into_iter().enumerate() {
            debug_fmtr.field(&i.to_string(), &element);
        }
        debug_fmtr.finish()
    }
}

impl<T, B, L> Component for Page<T, B, L>
where
    T: Component<Kind = InlineComponent>,
    B: Component<Kind = BlockComponent>,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = SectionComponent>,
{
    type Kind = PageComponent;
}

impl<T, B, L> Render<Html> for Page<T, B, L>
where
    T: Render<Html, Kind = InlineComponent>,
    B: Render<Html, Kind = BlockComponent>,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<Html, Kind = SectionComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str(
            "<div class=\"paideia-page-wrapper\" id=\"paideia-page-root\"><h1 \
             class=\"paideia-title\"><a href=\"#paideia-page-root\">",
        )?;
        self.title.render(renderer, ctx.with_kind(&InlineComponent::new()))?;
        write!(renderer, "</a></h1><div class=\"paideia-body\">")?;
        self.body.render(renderer, ctx.with_kind(&BlockComponent::new()))?;
        renderer.write_str("</div><div class=\"paideia-children\"")?;
        for child in &self.children {
            child.render(renderer, ctx.with_kind(&SectionComponent::new()))?;
        }
        renderer.write_str("</div></div>")?;
        Ok(())
    }
}

impl<T, B, L> Render<Markdown> for Page<T, B, L>
where
    T: Render<Markdown, Kind = InlineComponent>,
    B: Render<Markdown, Kind = BlockComponent>,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<Markdown, Kind = SectionComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("# ")?;
        self.title.render(renderer, ctx.with_kind(&InlineComponent::new()))?;
        renderer.write_str("\n\n")?;
        self.body.render(renderer, ctx.with_kind(&BlockComponent::new()))?;
        for child in &self.children {
            child.render(renderer, ctx.with_kind(&SectionComponent::new()))?;
        }
        Ok(())
    }
}

impl<T, B, L> Render<Text> for Page<T, B, L>
where
    T: Render<Text, Kind = InlineComponent>,
    B: Render<Text, Kind = BlockComponent>,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<Text, Kind = SectionComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.title.render(renderer, ctx.with_kind(&InlineComponent::new()))?;
        renderer.write_str("\n\n")?;
        self.body.render(renderer, ctx.with_kind(&BlockComponent::new()))?;
        for child in &self.children {
            child.render(renderer, ctx.with_kind(&SectionComponent::new()))?;
        }
        Ok(())
    }
}
