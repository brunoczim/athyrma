use super::BlockComponent;
use crate::component::{
    Component,
    Context,
    HtmlRendering,
    MdRendering,
    Render,
    TextRendering,
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

impl<L> Render<HtmlRendering> for UnorderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<HtmlRendering, Kind = BlockComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<HtmlRendering, Self::Kind>,
    ) -> fmt::Result {
        write!(fmtr, "<ul class=\"paideia-ulist\">")?;
        for element in &self.0 {
            write!(
                fmtr,
                "<li class=\"paideia-list-elem\">{}</li>",
                ctx.render(element)
            )?;
        }
        write!(fmtr, "</ul>")?;
        Ok(())
    }
}

impl<L> Render<MdRendering> for UnorderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<MdRendering, Kind = BlockComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<MdRendering, Self::Kind>,
    ) -> fmt::Result {
        for element in &self.0 {
            write!(fmtr, "- {}\n", ctx.render(element))?;
        }
        Ok(())
    }
}

impl<L> Render<TextRendering> for UnorderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<TextRendering, Kind = BlockComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<TextRendering, Self::Kind>,
    ) -> fmt::Result {
        for element in &self.0 {
            write!(fmtr, "- {}\n", ctx.render(element))?;
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

impl<L> Render<HtmlRendering> for OrderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<HtmlRendering, Kind = BlockComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<HtmlRendering, Self::Kind>,
    ) -> fmt::Result {
        write!(fmtr, "<ol class=\"paideia-olist\">")?;
        for element in &self.0 {
            write!(
                fmtr,
                "<li class=\"paideia-list-elem\">{}</li>",
                ctx.render(element)
            )?;
        }
        write!(fmtr, "</ol>")?;
        Ok(())
    }
}

impl<L> Render<MdRendering> for OrderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<MdRendering, Kind = BlockComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<MdRendering, Self::Kind>,
    ) -> fmt::Result {
        for (i, element) in self.0.into_iter().enumerate() {
            write!(fmtr, "{}. {}\n", i + 1, ctx.render(element))?;
        }
        Ok(())
    }
}

impl<L> Render<TextRendering> for OrderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<TextRendering, Kind = BlockComponent>,
{
    fn render(
        &self,
        fmtr: &mut fmt::Formatter,
        ctx: &Context<TextRendering, Self::Kind>,
    ) -> fmt::Result {
        for (i, element) in self.0.into_iter().enumerate() {
            write!(fmtr, "{}. {}\n", i + 1, ctx.render(element))?;
        }
        Ok(())
    }
}
