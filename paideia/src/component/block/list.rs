use super::BlockComponent;
use crate::{
    component::{Component, Context, Render, Renderer},
    render_format::{Html, Markdown, Text},
};
use std::fmt;

#[derive(Debug)]
pub struct UnorderedList<L>(pub L)
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = BlockComponent>;

impl<L> Clone for UnorderedList<L>
where
    L: Clone,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = BlockComponent>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<L> Copy for UnorderedList<L>
where
    L: Copy,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = BlockComponent>,
{
}

impl<L> Component for UnorderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = BlockComponent>,
{
    type Kind = BlockComponent;
}

impl<L> Render<Html> for UnorderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Render<Html, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        write!(renderer, "<ul class=\"paideia-ulist\">")?;
        for element in &self.0 {
            write!(
                renderer,
                "<li class=\"paideia-list-elem\">{}</li>",
                ctx.render(element)
            )?;
        }
        write!(renderer, "</ul>")?;
        Ok(())
    }
}

impl<'sess, L> Render<Markdown<'sess>> for UnorderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<Markdown<'sess>, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        let mut renderer =
            renderer.with_format(&mut renderer.format_mut().enter());
        for element in &self.0 {
            write!(renderer, "- {}\n", ctx.render(element))?;
        }
        Ok(())
    }
}

impl<'sess, L> Render<Text<'sess>> for UnorderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<Text<'sess>, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        let mut renderer =
            renderer.with_format(&mut renderer.format_mut().enter());
        for element in &self.0 {
            write!(renderer, "- {}\n", ctx.render(element))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct OrderedList<L>(pub L)
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = BlockComponent>;

impl<L> Clone for OrderedList<L>
where
    L: Clone,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = BlockComponent>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<L> Copy for OrderedList<L>
where
    L: Copy,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = BlockComponent>,
{
}

impl<L> Component for OrderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = BlockComponent>,
{
    type Kind = BlockComponent;
}

impl<L> Render<Html> for OrderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Render<Html, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        write!(renderer, "<ol class=\"paideia-olist\">")?;
        for element in &self.0 {
            write!(
                renderer,
                "<li class=\"paideia-list-elem\">{}</li>",
                ctx.render(element)
            )?;
        }
        write!(renderer, "</ol>")?;
        Ok(())
    }
}

impl<'sess, L> Render<Markdown<'sess>> for OrderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<Markdown<'sess>, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        let mut renderer =
            renderer.with_format(&mut renderer.format_mut().enter());
        for (i, element) in self.0.into_iter().enumerate() {
            write!(renderer, "{}. {}\n", i + 1, ctx.render(element))?;
        }
        Ok(())
    }
}

impl<'sess, L> Render<Text<'sess>> for OrderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<Text<'sess>, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        let mut renderer =
            renderer.with_format(&mut renderer.format_mut().enter());
        for (i, element) in self.0.into_iter().enumerate() {
            write!(renderer, "{}. {}\n", i + 1, ctx.render(element))?;
        }
        Ok(())
    }
}
