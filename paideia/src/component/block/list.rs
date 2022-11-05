use super::BlockComponent;
use crate::{
    component::{Component, Context, Render, Renderer},
    render::{self, Html, Markdown, Text},
};
use std::fmt::{self, Write};

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
        renderer.write_str("<ul class=\"paideia-ulist\">")?;
        for element in &self.0 {
            renderer.write_str("<li class=\"paideia-list-elem\">")?;
            element.render(renderer, ctx)?;
            renderer.write_str("</li>")?;
        }
        renderer.write_str("</ul>")?;
        Ok(())
    }
}

impl<'sess, L> Render<Markdown<'sess>> for UnorderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a, 'b_sess> <&'a L as IntoIterator>::Item:
        Render<Markdown<'b_sess>, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown<'sess>>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.with_format(
            |render_format, callback| callback(&mut render_format.enter()),
            |renderer| {
                for element in &self.0 {
                    renderer.write_str("-")?;
                    element.render(renderer, ctx)?;
                    renderer.write_str("\n")?;
                }
                Ok(())
            },
        )
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
        renderer.with_format(
            |render_format, callback| callback(&mut render_format.enter()),
            |renderer| {
                for element in &self.0 {
                    renderer.write_str("-")?;
                    element.render(&mut renderer, ctx)?;
                    renderer.write_str("\n")?;
                }
                Ok(())
            },
        )
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
        renderer.write_str("<ol class=\"paideia-olist\">")?;
        for element in &self.0 {
            renderer.write_str("<li class=\"paideia-list-elem\">")?;
            element.render(renderer, ctx)?;
            renderer.write_str("</li>")?;
        }
        renderer.write_str("</ol>")?;
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
        renderer.with_format(
            |render_format, callback| callback(&mut render_format.enter()),
            |renderer| {
                for (i, element) in self.0.into_iter().enumerate() {
                    write!(renderer, "{}. ", i)?;
                    element.render(&mut renderer, ctx)?;
                    renderer.write_str("\n")?;
                }
                Ok(())
            },
        )
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
        renderer.with_format(
            |render_format, callback| callback(&mut render_format.enter()),
            |renderer| {
                for (i, element) in self.0.into_iter().enumerate() {
                    write!(renderer, "{}. ", i)?;
                    element.render(&mut renderer, ctx)?;
                    renderer.write_str("\n")?;
                }
                Ok(())
            },
        )
    }
}
