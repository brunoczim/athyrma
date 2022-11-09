use super::{
    asset::AssetComponent,
    section::SectionComponent,
    BlockComponent,
    Component,
    ComponentKind,
    InlineComponent,
};
use crate::render::{Context, Html, Markdown, Render, Renderer, Text};
use std::fmt::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PageComponent;

impl ComponentKind for PageComponent {}

pub struct Page<A, B, L>
where
    for<'a> &'a A: IntoIterator,
    for<'a> <&'a A as IntoIterator>::Item: Component<Kind = AssetComponent>,
    B: Component<Kind = BlockComponent>,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = SectionComponent>,
{
    pub title: String,
    pub assets: A,
    pub body: B,
    pub children: L,
}

impl<A, B, L> fmt::Debug for Page<A, B, L>
where
    for<'a> &'a A: IntoIterator,
    for<'a> <&'a A as IntoIterator>::Item: Component<Kind = AssetComponent>,
    B: Component<Kind = BlockComponent>,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = SectionComponent>,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_fmtr = fmtr.debug_struct("UnorderedList");
        debug_fmtr.field("title", &self.title).field("body", &self.body);
        for (i, element) in self.assets.into_iter().enumerate() {
            debug_fmtr.field(&format!("asset[{}]", i), &element);
        }
        for (i, element) in self.children.into_iter().enumerate() {
            debug_fmtr.field(&format!("children[{}]", i), &element);
        }
        debug_fmtr.finish()
    }
}

impl<A, B, L> Clone for Page<A, B, L>
where
    A: Clone,
    for<'a> &'a A: IntoIterator,
    for<'a> <&'a A as IntoIterator>::Item: Component<Kind = AssetComponent>,
    B: Component<Kind = BlockComponent> + Clone,
    L: Clone,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = SectionComponent>,
{
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            assets: self.assets.clone(),
            body: self.body.clone(),
            children: self.children.clone(),
        }
    }
}

impl<A, B, L> Component for Page<A, B, L>
where
    for<'a> &'a A: IntoIterator + Clone,
    for<'a> <&'a A as IntoIterator>::Item: Component<Kind = AssetComponent>,
    B: Component<Kind = BlockComponent>,
    for<'a> &'a L: IntoIterator,
    for<'a> <&'a L as IntoIterator>::Item: Component<Kind = SectionComponent>,
{
    type Kind = PageComponent;
}

impl<A, B, L> Render<Html> for Page<A, B, L>
where
    for<'a> &'a A: IntoIterator,
    for<'a> <&'a A as IntoIterator>::Item: Render<Html, Kind = AssetComponent>,
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
            "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><meta \
             name=\"viewport\" content=\"width=device-width, \
             initial-scale=1.0\">",
        )?;
        for asset in &self.assets {
            asset.render(renderer, ctx.with_kind(&AssetComponent))?;
        }
        renderer.write_str("<title>")?;
        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str(
            "</title></head><body><div class=\"paideia-page-wrapper\" \
             id=\"paideia-page-root\"><h1 class=\"paideia-title\"><a \
             href=\"#paideia-page-root\">",
        )?;
        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;
        write!(renderer, "</a></h1><div class=\"paideia-body\">")?;
        self.body.render(renderer, ctx.with_kind(&BlockComponent))?;
        renderer.write_str("</div><div class=\"paideia-children\">")?;
        for child in &self.children {
            child.render(renderer, ctx.with_kind(&SectionComponent))?;
        }
        renderer.write_str("</div></div></body></html>")?;
        Ok(())
    }
}

impl<A, B, L> Render<Markdown> for Page<A, B, L>
where
    for<'a> &'a A: IntoIterator + Clone,
    for<'a> <&'a A as IntoIterator>::Item: Component<Kind = AssetComponent>,
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
        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\n\n")?;
        self.body.render(renderer, ctx.with_kind(&BlockComponent))?;
        for child in &self.children {
            child.render(renderer, ctx.with_kind(&SectionComponent))?;
        }
        Ok(())
    }
}

impl<A, B, L> Render<Text> for Page<A, B, L>
where
    for<'a> &'a A: IntoIterator + Clone,
    for<'a> <&'a A as IntoIterator>::Item: Component<Kind = AssetComponent>,
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
        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\n\n")?;
        self.body.render(renderer, ctx.with_kind(&BlockComponent))?;
        for child in &self.children {
            child.render(renderer, ctx.with_kind(&SectionComponent))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use katalogos::{hlist, HList};

    use super::{Page, PageComponent};
    use crate::{
        component::{
            asset::{AssetComponent, Script, Stylesheet},
            block::text::Paragraph,
            section::{Section, SectionComponent},
        },
        location::{Id, InternalPath, Location},
        render::{
            html::test::validate_html_document,
            Context,
            Html,
            RenderAsDisplay,
        },
    };

    #[test]
    fn page_without_assets_is_valid_html() {
        let rendered =
            RenderAsDisplay::new(
                Page::<
                    HList![(): AssetComponent],
                    _,
                    HList![(): SectionComponent],
                > {
                    title: String::from("Hello"),
                    assets: hlist![],
                    body: Paragraph("World!"),
                    children: hlist![],
                },
                &mut Html::default(),
                Context::new(&InternalPath::default(), &PageComponent),
            )
            .to_string();

        validate_html_document(&rendered).unwrap();
    }

    #[test]
    fn page_with_assets_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Page::<
                HList![(_, _): AssetComponent],
                _,
                HList![(): SectionComponent],
            > {
                title: String::from("Hello"),
                assets: hlist![
                    Stylesheet {
                        location: Location::internal("styles/main.css"),
                    },
                    Script { location: Location::internal("js/main.js") }
                ],
                body: Paragraph("World!"),
                children: hlist![],
            },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &PageComponent),
        )
        .to_string();

        validate_html_document(&rendered).unwrap();
    }

    #[test]
    fn page_with_children_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Page::<
                HList![(_): AssetComponent],
                _,
                HList![(_, _, _): SectionComponent],
            > {
                title: String::from("Hello"),
                assets: hlist![Stylesheet {
                    location: Location::internal("styles/main.css"),
                },],
                body: Paragraph("World, aaaa!"),
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
            Context::new(&InternalPath::default(), &PageComponent),
        )
        .to_string();

        validate_html_document(&rendered).unwrap();
    }
}
