use katalogos::list::List;

use super::BlockComponent;
use crate::{
    component::Component,
    render::{markdown, text, Context, Html, Markdown, Render, Renderer, Text},
};
use std::fmt::{self, Write};

pub struct UnorderedList<L>(pub L)
where
    L: List<Meta = BlockComponent>,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = BlockComponent>;

impl<L> fmt::Debug for UnorderedList<L>
where
    L: List<Meta = BlockComponent>,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = BlockComponent>,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_fmtr = fmtr.debug_tuple("UnorderedList");
        for element in &self.0 {
            debug_fmtr.field(&element);
        }
        debug_fmtr.finish()
    }
}

impl<L> Clone for UnorderedList<L>
where
    L: List<Meta = BlockComponent> + Clone,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = BlockComponent>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<L> Copy for UnorderedList<L>
where
    L: List<Meta = BlockComponent> + Copy,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = BlockComponent>,
{
}

impl<L> Component for UnorderedList<L>
where
    L: List<Meta = BlockComponent>,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = BlockComponent>,
{
    type Kind = BlockComponent;
}

impl<L> Render<Html> for UnorderedList<L>
where
    L: List<Meta = BlockComponent>,
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

impl<L> Render<Markdown> for UnorderedList<L>
where
    L: List<Meta = BlockComponent>,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<Markdown, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.scoped(markdown::Nest, |renderer| {
            for element in &self.0 {
                renderer.write_str("-")?;
                element.render(renderer, ctx)?;
                renderer.write_str("\n")?;
            }
            Ok(())
        })
    }
}

impl<L> Render<Text> for UnorderedList<L>
where
    L: List<Meta = BlockComponent>,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Render<Text, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.scoped(text::Nest, |renderer| {
            for element in &self.0 {
                renderer.write_str("-")?;
                element.render(renderer, ctx)?;
                renderer.write_str("\n")?;
            }
            Ok(())
        })
    }
}

pub struct OrderedList<L>(pub L)
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = BlockComponent>;

impl<L> fmt::Debug for OrderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = BlockComponent>,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_fmtr = fmtr.debug_tuple("OrderedList");
        for element in &self.0 {
            debug_fmtr.field(&element);
        }
        debug_fmtr.finish()
    }
}

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

impl<L> Render<Markdown> for OrderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<Markdown, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.scoped(markdown::Nest, |renderer| {
            for (i, element) in self.0.into_iter().enumerate() {
                write!(renderer, "{}. ", i)?;
                element.render(renderer, ctx)?;
                renderer.write_str("\n")?;
            }
            Ok(())
        })
    }
}

impl<L> Render<Text> for OrderedList<L>
where
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Render<Text, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.scoped(text::Nest, |renderer| {
            for (i, element) in self.0.into_iter().enumerate() {
                write!(renderer, "{}. ", i)?;
                element.render(renderer, ctx)?;
                renderer.write_str("\n")?;
            }
            Ok(())
        })
    }
}

#[cfg(test)]
mod test {
    use katalogos::{hlist, HList};

    use super::{OrderedList, UnorderedList};
    use crate::{
        component::{
            block::{text::Paragraph, InlineBlock},
            BlockComponent,
        },
        location::InternalPath,
        render::{
            html::test::validate_html_fragment,
            Context,
            Html,
            RenderAsDisplay,
        },
    };

    #[test]
    fn unordered_list_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            UnorderedList::<
                HList![(InlineBlock<&str>, Paragraph<&str>): BlockComponent],
            >(hlist![InlineBlock("abc"), Paragraph("def")]),
            &mut Html::default(),
            Context::new(&InternalPath::default(), &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn ordered_list_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            OrderedList::<
                HList![(InlineBlock<&str>, Paragraph<&str>): BlockComponent],
            >(hlist![InlineBlock("abc"), Paragraph("def")]),
            &mut Html::default(),
            Context::new(&InternalPath::default(), &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }
}
