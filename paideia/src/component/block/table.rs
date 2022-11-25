use std::{fmt, fmt::Write};

use crate::{
    component::{block::BlockComponent, Component, ComponentKind},
    render::{Context, Format, Html, Markdown, Render, Renderer, Text},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CellComponent;

impl ComponentKind for CellComponent {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CellComponent;

impl ComponentKind for CellComponent {}

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

impl<T> Cell<T>
where
    T: Component<Kind = BlockComponent>,
{
    fn render_generic_html<R>(
        &self,
        renderer: &mut Renderer<R>,
        ctx: Context<<Self as Component>::Kind>,
    ) -> fmt::Result
    where
        R: Format + ?Sized,
        T: Render<R>,
    {
        if self.attrs.header {
            write!(renderer, "<th class-name=\"paideia-table-header\"")?;
        } else {
            write!(renderer, "<td class-name=\"paideia-table-cell\"")?;
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
        self.render_generic_html(renderer, ctx)
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
        self.render_generic_html(renderer, ctx)
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
