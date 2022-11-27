use std::{
    cmp::Ordering,
    fmt,
    fmt::Write,
    hash::{Hash, Hasher},
};

use katalogos::IntoIterRef;

use crate::{
    component::{
        block::BlockComponent,
        Component,
        ComponentKind,
        InlineComponent,
    },
    render::{Context, Html, Markdown, Render, Renderer, Text},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CellComponent;

impl ComponentKind for CellComponent {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RowComponent;

impl ComponentKind for RowComponent {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CellAttrs {
    pub header: bool,
    pub rowspan: u32,
    pub colspan: u32,
}

impl Default for CellAttrs {
    fn default() -> Self {
        Self { header: false, rowspan: 1, colspan: 1 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Cell<T>
where
    T: Component<Kind = BlockComponent>,
{
    pub child: T,
    pub attrs: CellAttrs,
}

impl<T> Component for Cell<T>
where
    T: Component<Kind = BlockComponent>,
{
    type Kind = CellComponent;
}

impl<T> Render<Html> for Cell<T>
where
    T: Render<Html, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        if self.attrs.header {
            write!(renderer, "<th class=\"paideia-table-header\"")?;
        } else {
            write!(renderer, "<td class=\"paideia-table-cell\"")?;
        }
        if self.attrs.rowspan != 1 {
            write!(renderer, " rowspan=\"{}\"", self.attrs.rowspan)?;
        }
        if self.attrs.colspan != 1 {
            write!(renderer, " colspan=\"{}\"", self.attrs.colspan)?;
        }
        write!(renderer, ">")?;
        self.child.render(renderer, ctx.with_kind(&BlockComponent))?;
        if self.attrs.header {
            write!(renderer, "</th>")?;
        } else {
            write!(renderer, "</td>")?;
        }
        Ok(())
    }
}

impl<T> Render<Markdown> for Cell<T>
where
    T: Render<Markdown, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        if self.attrs.header {
            write!(renderer, "<th")?;
        } else {
            write!(renderer, "<td")?;
        }
        if self.attrs.rowspan != 1 {
            write!(renderer, " rowspan=\"{}\"", self.attrs.rowspan)?;
        }
        if self.attrs.colspan != 1 {
            write!(renderer, " colspan=\"{}\"", self.attrs.colspan)?;
        }
        write!(renderer, ">")?;
        self.child.render(renderer, ctx.with_kind(&BlockComponent))?;
        if self.attrs.header {
            write!(renderer, "</th>")?;
        } else {
            write!(renderer, "</td>")?;
        }
        Ok(())
    }
}

impl<T> Render<Text> for Cell<T>
where
    T: Render<Text, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        self.child.render(renderer, ctx.with_kind(&BlockComponent))
    }
}

pub struct Row<C>(pub C)
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent>;

impl<C> fmt::Debug for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent>,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_fmtr = fmtr.debug_tuple("Row");
        for element in self.0.iter() {
            debug_fmtr.field(&element);
        }
        debug_fmtr.finish()
    }
}

impl<C> Clone for Row<C>
where
    C: IntoIterRef + Clone,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<C> Copy for Row<C>
where
    C: IntoIterRef + Copy,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent>,
{
}

impl<C> PartialEq for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent> + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().eq(other.0.iter())
    }
}

impl<C> Eq for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent> + Eq,
{
}

impl<C> PartialOrd for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent> + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.iter().partial_cmp(other.0.iter())
    }
}

impl<C> Ord for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent> + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.iter().cmp(other.0.iter())
    }
}

impl<C> Hash for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent> + Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        for (i, element) in self.0.iter().enumerate() {
            i.hash(state);
            element.hash(state);
        }
    }
}

impl<C> Default for Row<C>
where
    C: IntoIterRef + Default,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent>,
{
    fn default() -> Self {
        Self(C::default())
    }
}

impl<C> Component for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent>,
{
    type Kind = RowComponent;
}

impl<C> Render<Html> for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Render<Html, Kind = CellComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<tr class=\"paideia-table-row\">")?;
        for cell in self.0.iter() {
            cell.render(renderer, ctx.with_kind(&CellComponent))?;
        }
        renderer.write_str("</tr>")?;
        Ok(())
    }
}

impl<C> Render<Markdown> for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Render<Markdown, Kind = CellComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<tr>")?;
        for cell in self.0.iter() {
            cell.render(renderer, ctx.with_kind(&CellComponent))?;
        }
        renderer.write_str("</tr>")?;
        Ok(())
    }
}

impl<C> Render<Text> for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Render<Text, Kind = CellComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        for cell in self.0.iter() {
            cell.render(renderer, ctx.with_kind(&CellComponent))?;
            renderer.write_str("\n")?;
        }
        Ok(())
    }
}

pub struct Table<L>(pub L)
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>;

impl<L> fmt::Debug for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_fmtr = fmtr.debug_tuple("Table");
        for element in self.0.iter() {
            debug_fmtr.field(&element);
        }
        debug_fmtr.finish()
    }
}

impl<L> Clone for Table<L>
where
    L: IntoIterRef + Clone,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<L> Copy for Table<L>
where
    L: IntoIterRef + Copy,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
}

impl<L> PartialEq for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().eq(other.0.iter())
    }
}

impl<L> Eq for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + Eq,
{
}

impl<L> PartialOrd for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.iter().partial_cmp(other.0.iter())
    }
}

impl<L> Ord for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.iter().cmp(other.0.iter())
    }
}

impl<L> Hash for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        for (i, element) in self.0.iter().enumerate() {
            i.hash(state);
            element.hash(state);
        }
    }
}

impl<L> Default for Table<L>
where
    L: IntoIterRef + Default,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    fn default() -> Self {
        Self(L::default())
    }
}

impl<L> Component for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    type Kind = BlockComponent;
}

impl<L> Render<Html> for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Html, Kind = RowComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<div class=\"paideia-table\"><table>")?;
        for row in self.0.iter() {
            row.render(renderer, ctx.with_kind(&RowComponent))?;
        }
        renderer.write_str("</table></div>")?;
        Ok(())
    }
}

impl<L> Render<Markdown> for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Markdown, Kind = RowComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<table>")?;
        for row in self.0.iter() {
            row.render(renderer, ctx.with_kind(&RowComponent))?;
        }
        renderer.write_str("</table>")?;
        Ok(())
    }
}

impl<L> Render<Text> for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Text, Kind = RowComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        for row in self.0.iter() {
            renderer.write_str("- - - - - - - - - - -\n\n")?;
            row.render(renderer, ctx.with_kind(&RowComponent))?;
            renderer.write_str("\n")?;
        }
        Ok(())
    }
}

pub struct TitledTable<T, L>
where
    T: Component<Kind = InlineComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    title: T,
    table: Table<L>,
}

impl<T, L> fmt::Debug for TitledTable<T, L>
where
    T: Component<Kind = InlineComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_struct("TitledTable")
            .field("title", &self.title)
            .field("table", &self.table)
            .finish()
    }
}

impl<T, L> Clone for TitledTable<T, L>
where
    T: Component<Kind = InlineComponent> + Clone,
    L: IntoIterRef + Clone,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    fn clone(&self) -> Self {
        Self { title: self.title.clone(), table: self.table.clone() }
    }
}

impl<T, L> Copy for TitledTable<T, L>
where
    T: Component<Kind = InlineComponent> + Copy,
    L: IntoIterRef + Copy,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
}

impl<T, L> PartialEq for TitledTable<T, L>
where
    T: Component<Kind = InlineComponent> + PartialEq,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.table == other.table
    }
}

impl<T, L> Eq for TitledTable<T, L>
where
    T: Component<Kind = InlineComponent> + Eq,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + Eq,
{
}

impl<T, L> PartialOrd for TitledTable<T, L>
where
    T: Component<Kind = InlineComponent> + PartialOrd,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.title
                .partial_cmp(&other.title)?
                .then(self.table.partial_cmp(&other.table)?),
        )
    }
}

impl<T, L> Ord for TitledTable<T, L>
where
    T: Component<Kind = InlineComponent> + Ord,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.title.cmp(&other.title).then_with(|| self.table.cmp(&other.table))
    }
}

impl<T, L> Hash for TitledTable<T, L>
where
    T: Component<Kind = InlineComponent> + Hash,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.title.hash(state);
        self.table.hash(state);
    }
}

impl<T, L> Default for TitledTable<T, L>
where
    T: Component<Kind = InlineComponent> + Default,
    L: IntoIterRef + Default,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    fn default() -> Self {
        Self { title: T::default(), table: Table::default() }
    }
}

impl<T, L> Component for TitledTable<T, L>
where
    T: Component<Kind = InlineComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    type Kind = BlockComponent;
}

impl<T, L> Render<Html> for TitledTable<T, L>
where
    T: Render<Html, Kind = InlineComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Html, Kind = RowComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str(
            "<div class=\"paideia-titled-table\"><div \
             class=\"paideia-table-title\">",
        )?;
        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("</div>")?;
        self.table.render(renderer, ctx)?;
        renderer.write_str("</div>")?;
        Ok(())
    }
}

impl<T, L> Render<Markdown> for TitledTable<T, L>
where
    T: Render<Markdown, Kind = InlineComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Markdown, Kind = RowComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\n")?;
        self.table.render(renderer, ctx)?;
        Ok(())
    }
}

impl<T, L> Render<Text> for TitledTable<T, L>
where
    T: Render<Text, Kind = InlineComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Text, Kind = RowComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\n")?;
        self.table.render(renderer, ctx)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{Cell, CellAttrs, Row, Table, TitledTable};
    use crate::{
        component::{
            block::{text::Paragraph, InlineBlock},
            inline::text::Bold,
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
    use katalogos::harray;

    #[test]
    fn table_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Table(harray![
                Row(harray![
                    Cell {
                        child: InlineBlock("abc"),
                        attrs: CellAttrs::default()
                    },
                    Cell {
                        child: Paragraph("123"),
                        attrs: CellAttrs::default()
                    },
                ]),
                Row(harray![Cell {
                    child: Paragraph("a c r m f m"),
                    attrs: CellAttrs::default()
                }])
            ]),
            &mut Html::default(),
            Context::new(&InternalPath::default(), &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn titled_table_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            TitledTable {
                title: Bold("aaaaaaa"),
                table: Table(harray![
                    Row(harray![
                        Cell {
                            child: InlineBlock("abc"),
                            attrs: CellAttrs::default()
                        },
                        Cell {
                            child: Paragraph("123"),
                            attrs: CellAttrs::default()
                        },
                    ]),
                    Row(harray![Cell {
                        child: Paragraph("a c r m f m"),
                        attrs: CellAttrs::default()
                    }])
                ]),
            },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }
}
