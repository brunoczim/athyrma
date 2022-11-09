use katalogos::list::List;

use super::{BlockComponent, Component, ComponentKind, InlineComponent};
use crate::{
    location::{Id, InternalLoc, Location},
    render::{Context, Html, Markdown, Render, Renderer, Text},
};
use std::fmt::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SectionComponent;

impl ComponentKind for SectionComponent {}

pub struct Section<T, B, L>
where
    T: Component<Kind = InlineComponent>,
    B: Component<Kind = BlockComponent>,
    L: List,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = SectionComponent>,
{
    pub title: T,
    pub id: Option<Id>,
    pub body: B,
    pub children: L,
}

impl<T, B, L> fmt::Debug for Section<T, B, L>
where
    T: Component<Kind = InlineComponent>,
    B: Component<Kind = BlockComponent>,
    L: List,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = SectionComponent>,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_fmtr = fmtr.debug_struct("UnorderedList");
        debug_fmtr
            .field("title", &self.title)
            .field("id", &self.id)
            .field("body", &self.body);
        for (i, element) in self.children.into_iter().enumerate() {
            debug_fmtr.field(&i.to_string(), &element);
        }
        debug_fmtr.finish()
    }
}

impl<T, B, L> Clone for Section<T, B, L>
where
    T: Component<Kind = InlineComponent> + Clone,
    B: Component<Kind = BlockComponent> + Clone,
    L: List,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = SectionComponent>,
    L: Clone,
{
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            id: self.id.clone(),
            body: self.body.clone(),
            children: self.children.clone(),
        }
    }
}

impl<T, B, L> Component for Section<T, B, L>
where
    T: Component<Kind = InlineComponent>,
    B: Component<Kind = BlockComponent>,
    L: List,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = SectionComponent>,
{
    type Kind = SectionComponent;
}

impl<T, B, L> Render<Html> for Section<T, B, L>
where
    T: Render<Html, Kind = InlineComponent>,
    B: Render<Html, Kind = BlockComponent>,
    L: List,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<Html, Kind = SectionComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        let tag = match ctx.level() {
            0 => "h2",
            1 => "h3",
            2 => "h4",
            3 => "h5",
            _ => "h6",
        };
        write!(
            renderer,
            "<div class=\"paideia-section paideia-section-{}\"",
            ctx.level()
        )?;
        if let Some(id) = &self.id {
            renderer.write_str(" id=\"")?;
            id.render(renderer, ctx.with_kind(&InlineComponent))?;
            renderer.write_str("\"")?;
        }
        write!(renderer, "><{} class=\"paideia-title\">", tag)?;
        if let Some(id) = &self.id {
            let location = Location::Internal(InternalLoc {
                path: ctx.location().clone(),
                id: Some(id.clone()),
            });
            renderer.write_str("<a href=\"")?;
            location.render(renderer, ctx.with_kind(&InlineComponent))?;
            renderer.write_str("\">")?;
        }
        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;
        if self.id.is_some() {
            renderer.write_str("</a>")?;
        }
        write!(renderer, "</{}><div class=\"paideia-body\">", tag)?;
        self.body.render(renderer, ctx.with_kind(&BlockComponent))?;
        renderer.write_str("</div><div class=\"paideia-children\">")?;
        for child in &self.children {
            child.render(renderer, ctx.enter().with_kind(&SectionComponent))?;
        }
        renderer.write_str("</div></div>")?;
        Ok(())
    }
}

impl<T, B, L> Render<Markdown> for Section<T, B, L>
where
    T: Render<Markdown, Kind = InlineComponent>,
    B: Render<Markdown, Kind = BlockComponent>,
    L: List,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<Markdown, Kind = SectionComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        let tag = match ctx.level() {
            0 => "##",
            1 => "###",
            2 => "####",
            3 => "#####",
            _ => "######",
        };
        write!(renderer, "{} ", tag)?;
        if let Some(id) = &self.id {
            renderer.write_str("<span id=\"")?;
            id.render(renderer, ctx.with_kind(&InlineComponent))?;
            renderer.write_str("\">[")?;
        }

        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;

        if let Some(id) = &self.id {
            let location = Location::Internal(InternalLoc {
                path: ctx.location().clone(),
                id: Some(id.clone()),
            });
            renderer.write_str("](")?;
            location.render(renderer, ctx.with_kind(&InlineComponent))?;
            renderer.write_str(")")?;
        }
        renderer.write_str("\n\n")?;
        self.body.render(renderer, ctx.with_kind(&BlockComponent))?;
        for child in &self.children {
            child.render(renderer, ctx.enter().with_kind(&SectionComponent))?;
        }
        Ok(())
    }
}

impl<T, B, L> Render<Text> for Section<T, B, L>
where
    T: Render<Text, Kind = InlineComponent>,
    B: Render<Text, Kind = BlockComponent>,
    L: List,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item:
        Render<Text, Kind = SectionComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\n\n")?;
        self.body.render(renderer, ctx.with_kind(&BlockComponent))?;
        for child in &self.children {
            child.render(renderer, ctx.enter().with_kind(&SectionComponent))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use katalogos::{hlist, HList};

    use super::{Section, SectionComponent};
    use crate::{
        component::block::text::Paragraph,
        location::{Id, InternalPath},
        render::{
            html::test::validate_html_fragment,
            Context,
            Html,
            RenderAsDisplay,
        },
    };

    #[test]
    fn section_with_id_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Section::<_, _, HList![(): SectionComponent]> {
                title: "Hello",
                id: Some(Id::new("hello").unwrap()),
                body: Paragraph("World!"),
                children: hlist![],
            },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &SectionComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn section_without_id_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Section::<_, _, HList![(): SectionComponent]> {
                title: "Hello",
                id: None,
                body: Paragraph("World!"),
                children: hlist![],
            },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &SectionComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn section_with_children_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Section::<_, _, HList![(_, _, _): SectionComponent]> {
                title: "Hello",
                id: None,
                body: Paragraph("World!"),
                children: hlist![
                    Section::<_, _, HList![(): SectionComponent]> {
                        title: "Hey",
                        id: None,
                        body: Paragraph("Hey!"),
                        children: hlist![],
                    },
                    Section::<_, _, HList![(_): SectionComponent]> {
                        title: "Good",
                        id: Some(Id::new("good").unwrap()),
                        body: Paragraph("Afternoon!"),
                        children: hlist![Section::<
                            _,
                            _,
                            HList![(): SectionComponent],
                        > {
                            title: "By",
                            id: None,
                            body: Paragraph("Bye!"),
                            children: hlist![],
                        }],
                    },
                    Section::<_, _, HList![(): SectionComponent]> {
                        title: "Hay",
                        id: None,
                        body: Paragraph("Bay!"),
                        children: hlist![],
                    },
                ],
            },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &SectionComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }
}
